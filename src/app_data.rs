use std::{cell::UnsafeCell, mem::MaybeUninit};

static APP_DATA: InitOnce<AppData> = InitOnce::new();

struct InitOnce<T>(UnsafeCell<MaybeUninit<T>>);

impl<T> InitOnce<T> {
    pub const fn new() -> Self {
        Self(UnsafeCell::new(MaybeUninit::uninit()))
    }

    pub unsafe fn init(&self, data: T) {
        *self.0.get() = MaybeUninit::new(data);
    }

    pub fn get(&self) -> &T {
        unsafe { (*self.0.get()).assume_init_ref() }
    }
}

unsafe impl<T> Send for InitOnce<T> {}
unsafe impl<T> Sync for InitOnce<T> {}

pub struct AppData {
    pub conn: sea_orm::DbConn,
    pub authentication_service: authentication_service::Client,
}

impl AppData {
    pub async fn new() -> Self {
        let authentication_service = {
            let server_key = std::env::var("AUTH_SERVER_KEY")
                .unwrap_or_else(|_| String::from("auth_server.pem"));

            let private_key =
                std::env::var("PRIVATE_KEY").unwrap_or_else(|_| String::from("private.pem"));

            let server_key = match std::fs::read(&*shellexpand::tilde(&server_key)) {
                Ok(x) => x,
                Err(err) => panic!("Unable to open {server_key:?}: {err:?}"),
            };

            let private_key = match std::fs::read(&*shellexpand::tilde(&private_key)) {
                Ok(x) => x,
                Err(err) => panic!("Unable to open {private_key:?}: {err:?}"),
            };

            let host = std::env::var("AUTH_SERVER_HOST")
                .expect("Missing environment variable AUTH_SERVER_HOST");

            let server_id = std::env::var("AUTH_SERVER_ID")
                .expect("Missing environment variable AUTH_SERVER_ID")
                .parse()
                .expect("Invalid environment variable AUTH_SERVER_ID");

            authentication_service::Client::new(server_id, &private_key, host, &server_key).unwrap()
        };

        let database_url =
            std::env::var("DATABASE_URL").expect("Missing environment variable DATABASE_URL");

        let conn = sea_orm::Database::connect(&database_url)
            .await
            .expect("Unable to connect to database");
        <migration::Migrator as migration::MigratorTrait>::up(&conn, None)
            .await
            .expect("Failed to run migrations");

        Self {
            conn,
            authentication_service,
        }
    }

    pub async unsafe fn init() {
        APP_DATA.init(Self::new().await)
    }

    pub fn get() -> &'static Self {
        APP_DATA.get()
    }
}
