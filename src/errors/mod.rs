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
    pub fn as_result<T>(&self) -> Result<T> {
        Err(self.clone())
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

        Kind::UnknownRequest.to_error()
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        match err {
            _ => Kind::UnknownParsing.to_error(),
        }
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        Kind::NoApiKey.to_error()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    NoArgs,
    InvalidArgs,
    NoApiKey,
    InvalidApiKey,
    TmdbConnectionFailure,
    ParsingFailed,
    InvalidMovieId,
    UnknownRequest,
    UnknownParsing,
    DataParsing((usize, usize, String)),
    PersonSearchFailed,
    PersonSearchNoResults,
}

impl Kind {
    pub fn to_error(self) -> Error {
        Error { kind: self }
    }
    pub fn to_result<T>(self) -> Result<T> {
        Err(self.to_error())
    }
}
