use {
    crate::user::SessionUser,
    actix_session::Session,
    actix_web::{
        get,
        http::{
            header::{self, HeaderValue},
            Method, StatusCode,
        },
        web, Error, HttpRequest, HttpResponse, HttpResponseBuilder, Route,
    },
};

const PATHS: svelte_path_finder::SveltePathFinder<'static> =
    svelte_path_finder::build_from_filesystem!("./frontend/src/");

const CACHE_CONTROL: &str = "public, immutable, max-age=86400"; // 60*60*24 = 86400

lazy_static::lazy_static! {
    static ref PATHS_JSON: String = serde_json::to_string(&PATHS).unwrap();
    static ref CACHE_CONTROL_HEADER: HeaderValue = HeaderValue::from_str(CACHE_CONTROL).unwrap();
}

fn set_cache_header(path: &str, mut response: HttpResponse) -> HttpResponse {
    if path.starts_with("_app/immutable") {
        response
            .headers_mut()
            .append(header::CACHE_CONTROL, CACHE_CONTROL_HEADER.clone());
    }
    response
}

#[derive(Debug, serde::Deserialize)]
struct Query {
    path: Option<String>,
}

#[cfg(debug_assertions)]
mod debug {
    use {super::*, actix_files::NamedFile, std::path::Path};

    pub fn get_file(path: &str, req: &HttpRequest) -> HttpResponse {
        let full_path = format!("./frontend/build/{path}");
        let full_path = Path::new(&full_path);
        let mut file = None;
        if full_path.is_file() {
            file = NamedFile::open(full_path).ok();
        }
        match file {
            Some(file) => set_cache_header(path, file.into_response(req)),
            None => match NamedFile::open("./frontend/build/index.html") {
                Ok(file) => {
                    let mut res = file.into_response(req);
                    *res.status_mut() = StatusCode::NOT_FOUND;
                    res
                }
                Err(_) => HttpResponse::NotFound().finish(),
            },
        }
    }

    pub(super) async fn serve_file(
        req: HttpRequest,
        session: Session,
        query: web::Query<Query>,
    ) -> Result<HttpResponse, Error> {
        if req.method() != Method::GET {
            return Ok(HttpResponse::NotFound().finish());
        }
        let path = req.path().trim_matches('/');

        match PATHS.find(path) {
            Some(_) if SessionUser::try_from(&session).is_ok() => {
                if path == "login" {
                    Ok(HttpResponse::TemporaryRedirect()
                        .append_header(("location", query.path.as_deref().unwrap_or("/")))
                        .finish())
                } else {
                    Ok(get_file("index.html", &req))
                }
            }
            Some(false) => Ok(get_file("index.html", &req)),
            Some(true) => Ok(HttpResponse::TemporaryRedirect()
                .append_header(("location", format!("/login?path={}", req.uri())))
                .finish()),
            None => Ok(get_file(path, &req)),
        }
    }
}

#[cfg(debug_assertions)]
pub use debug::get_file;
#[cfg(debug_assertions)]
use debug::serve_file;

#[cfg(not(debug_assertions))]
mod release {
    use {
        super::*,
        actix_web::http::{header::ContentType, StatusCode},
    };

    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
    lazy_static::lazy_static! {
        static ref DATA: std::collections::HashMap<&'static str, static_files::Resource> = generate();
    }

    pub fn get_file(path: &str, _req: &HttpRequest) -> HttpResponse {
        match DATA.get(path) {
            Some(file) => {
                let mut resp = HttpResponse::build(StatusCode::OK);
                resp.content_type(file.mime_type);
                set_cache_header(path, resp.body(file.data))
            }
            None => match DATA.get("index.html") {
                Some(file) => {
                    let mut resp = HttpResponse::build(StatusCode::NOT_FOUND);
                    resp.content_type(ContentType::html());
                    resp.body(file.data)
                }
                None => HttpResponse::NotFound().finish(),
            },
        }
    }

    pub(super) async fn serve_file(
        req: HttpRequest,
        session: Session,
        query: web::Query<Query>,
    ) -> Result<HttpResponse, Error> {
        if req.method() != Method::GET {
            return Ok(HttpResponse::NotFound().finish());
        }
        let path = req.path().trim_matches('/');

        match PATHS.find(path) {
            Some(_) if SessionUser::try_from(&session).is_ok() => {
                if path == "login" {
                    Ok(HttpResponse::TemporaryRedirect()
                        .append_header((
                            "location",
                            query.path.as_ref().map(String::as_str).unwrap_or("/"),
                        ))
                        .finish())
                } else {
                    Ok(get_file("index.html", &req))
                }
            }
            Some(false) => Ok(get_file("index.html", &req)),
            Some(true) => Ok(HttpResponse::TemporaryRedirect()
                .append_header(("location", format!("/login?path={}", req.uri())))
                .finish()),
            None => Ok(get_file(path, &req)),
        }
    }
}

#[cfg(not(debug_assertions))]
pub use release::get_file;
#[cfg(not(debug_assertions))]
use release::serve_file;

#[get("svelte_path_finder.json")]
async fn serve_paths() -> HttpResponse {
    let cache_control = match cfg!(debug_assertions) {
        true => "no-cache",
        false => CACHE_CONTROL,
    };
    HttpResponseBuilder::new(StatusCode::OK)
        .content_type("application/json")
        .append_header((header::CACHE_CONTROL, cache_control))
        .body(PATHS_JSON.as_str())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(serve_paths);
    cfg.default_service(Route::new().to(serve_file));
}
