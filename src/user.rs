use {
    crate::error::ToErr,
    actix_session::Session,
    actix_web::{
        error::ErrorUnauthorized,
        get,
        http::header,
        post,
        web::{self},
        Error, HttpRequest, HttpResponse, Responder,
    },
    serde::{Deserialize, Serialize},
};

use crate::AppData;

#[derive(Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i32,
    pub name: String,
}

impl std::convert::TryFrom<&Session> for SessionUser {
    type Error = Error;
    fn try_from(session: &Session) -> Result<SessionUser, Error> {
        match session.get::<SessionUser>("user")? {
            Some(id) => Ok(id),
            None => Err(ErrorUnauthorized("Not logged in")),
        }
    }
}

#[get("/auth")]
async fn auth_redirect() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .append_header((
            header::LOCATION,
            AppData::get().authentication_service.get_redirect_url(),
        ))
        .finish()
}

#[derive(Deserialize)]
struct AuthServerId {
    id: Option<u64>,
}

#[get("/auth/auth_server")]
async fn auth_server_login(
    req: HttpRequest,
    session: Session,
    id: web::Query<AuthServerId>,
) -> Result<impl Responder, Error> {
    match id.into_inner().id {
        Some(id) => {
            let user = AppData::get()
                .authentication_service
                .query_authentication_request(id)
                .await
                .to_err()?;

            session
                .insert(
                    "user",
                    SessionUser {
                        id: user.id,
                        name: user.display_name,
                    },
                )
                .to_err()?;

            let mut response = HttpResponse::Found();
            match req.cookie("path") {
                Some(cookie) => response.append_header(("location", cookie.value())),
                None => response.append_header(("location", "/")),
            };
            Ok(response.finish())
        }
        None => Ok(HttpResponse::Found()
            .append_header((
                "location",
                AppData::get().authentication_service.get_redirect_url(),
            ))
            .finish()),
    }
}

#[get("/auth/user")]
async fn get_user(session: Session) -> Result<impl Responder, Error> {
    SessionUser::try_from(&session).map(|user| HttpResponse::Ok().json(user))
}

#[post("/auth/logout")]
async fn logout(session: Session) -> Result<impl Responder, Error> {
    session.remove("user");
    Ok(HttpResponse::Ok().finish())
}

#[get("/auth/users/{id}/picture")]
async fn get_profile_picture(id: web::Path<i32>) -> Result<impl Responder, Error> {
    let id = id.into_inner();
    Ok(HttpResponse::PermanentRedirect()
        .append_header((
            header::LOCATION,
            format!(
                "https://{}/api/profiles/{id}/picture",
                AppData::get().authentication_service.host()
            ),
        ))
        .finish())
}

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(auth_redirect);
    cfg.service(auth_server_login);
    cfg.service(get_user);
    cfg.service(logout);
    cfg.service(get_profile_picture);
}
