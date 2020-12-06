crate mod identities;

#[derive(Debug, thiserror::Error)]
crate enum Error {
    #[error("API response {code}, {msg}")]
    Api { msg: String, code: u16 },

    #[error(transparent)]
    UnknownUreq(#[from] ureq::Error),

    #[error(transparent)]
    UnknownIo(#[from] std::io::Error),
}

#[derive(Debug, serde::Deserialize)]
struct ErrorResponse {
    message: String,
    variant: String,
}

trait ApiResponseExt: Sized {
    #[fehler::throws]
    fn check_error(self) -> Self;
}

impl ApiResponseExt for ureq::Response {
    #[fehler::throws]
    fn check_error(self) -> Self {
        if self.error() {
            if self.synthetic() {
                fehler::throw!(self.into_synthetic_error().unwrap());
            }
            let code = self.status();
            let ErrorResponse { message: msg, .. } = self.into_json_deserialize()?;
            fehler::throw!(Error::Api { msg, code });
        }
        self
    }
}
