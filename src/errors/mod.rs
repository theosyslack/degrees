use std::env::VarError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub kind: Kind,
}

impl Error {
    pub fn from_kind(kind: Kind) -> Self {
        Error { kind }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_connect() {
            return Error::from_kind(Kind::TmdbConnectionFailure);
        }

        if err.is_decode() {
            return Error::from_kind(Kind::ParsingFailed);
        }

        Kind::UnknownRequest.as_error()
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Kind::SerdeParsingError(err.to_string()).as_error()
    }
}

impl From<VarError> for Error {
    fn from(_: VarError) -> Self {
        Kind::NoApiKey.as_error()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    NoArgs,
    NoApiKey,
    // InvalidApiKey,
    TmdbConnectionFailure,
    ParsingFailed,
    // InvalidMovieId,
    UnknownRequest,
    // UnknownParsing,
    SerdeParsingError(String),
    DataParsing((usize, usize, String)),
    // PersonSearchFailed,
    PersonSearchNoResults,
}

impl Kind {
    pub fn as_error(&self) -> Error {
        Error { kind: self.clone() }
    }
}
