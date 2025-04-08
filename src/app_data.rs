use {
    std::{cell::UnsafeCell, collections::HashMap, mem::MaybeUninit},
    tokio::sync::RwLock,
    crate::room::Room,
};

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
    pub authentication_service: authentication_service::Client,
    pub rooms: RwLock<HashMap<u32, RwLock<Room>>>,
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

        Self {
            authentication_service,
            rooms: RwLock::new(HashMap::new()),
        }
    }

    pub async unsafe fn init() {
        APP_DATA.init(Self::new().await)
    }

    pub fn get() -> &'static Self {
        APP_DATA.get()
    }
}
