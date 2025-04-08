use {
    actix_web::error::{Error, ErrorInternalServerError, ErrorNotFound},
    std::fmt,
};

pub trait ToErr {
    type Return;
    fn to_err(self) -> Self::Return;
}

impl<T, E: ToErr> ToErr for Result<T, E> {
    type Return = Result<T, E::Return>;

    fn to_err(self) -> Self::Return {
        self.map_err(ToErr::to_err)
    }
}

pub trait MapToErr {
    type Return;
    fn map_to_err(self) -> Self::Return;
}

impl<T, E: ToErr<Return = Error>> MapToErr for Result<Option<T>, E> {
    type Return = Result<T, Error>;

    fn map_to_err(self) -> Self::Return {
        match self {
            Ok(Some(x)) => Ok(x),
            Ok(None) => Err(ErrorNotFound("Not Found")),
            Err(err) => Err(err.to_err()),
        }
    }
}

fn internal_server_error<T: fmt::Debug>(err: T) -> Error {
    log::error!("{err:?}");
    #[cfg(debug_assertions)]
    return ErrorInternalServerError(format!("Internal Server Error: {err:?}"));
    #[cfg(not(debug_assertions))]
    return ErrorInternalServerError("Internal Server Error");
}

impl ToErr for authentication_service::client::Error {
    type Return = Error;

    fn to_err(self) -> Self::Return {
        internal_server_error(self)
    }
}

impl ToErr for actix_session::SessionInsertError {
    type Return = Error;

    fn to_err(self) -> Self::Return {
        internal_server_error(self)
    }
}
