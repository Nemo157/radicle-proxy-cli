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
