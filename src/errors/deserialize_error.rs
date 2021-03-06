use crate::common::*;
use std::fmt::{Display, Error};
use tower_web::error::Catch;
use tower_web::error::IntoCatch; // TODO
use tower_web::Error as TowerError;

#[derive(Clone, Debug)]
/// Represents errors encountered when attempting to deserialize a message
pub(crate) enum DeserializeErrorKind {
    /// If a request specifies a content type other than JSON or Protobuf
    InvalidContentTypeForMessage,
    /// TODO: actually use..
    InvalidHeadersForMessage,
    /// If we hit an error while trying to parse a message as JSON
    ErrorParsingJson,
    /// If we hit an error while trying to parse a message as a Protobuf message
    ErrorParsingProtobuf,
    /// If we hit an error while trying to parse a message as Plaintext
    ErrorParsingPlaintext,
}

#[derive(Clone, Debug)]
/// Represents an error encountered while attempting to deserialize a message
pub(crate) struct DeserializeError {
    kind: DeserializeErrorKind,
    err_message: Option<String>,
}

impl DeserializeError {
    #[allow(dead_code)]
    pub(crate) fn new(kind: DeserializeErrorKind) -> Self {
        Self {
            kind,
            err_message: None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_with_message(kind: DeserializeErrorKind, message: String) -> Self {
        Self {
            kind,
            err_message: Some(message),
        }
    }

    pub(crate) fn new_with_error<T: Display>(kind: DeserializeErrorKind, err: T) -> Self {
        Self {
            kind,
            err_message: Some(format!("{}", err)),
        }
    }

    pub(crate) fn get_code_and_message(&self) -> (u16, String) {
        use DeserializeErrorKind::*;

        let (status, msg) = match &self.kind {
            InvalidHeadersForMessage => (415, "TODO"),
            InvalidContentTypeForMessage => (415, "TODO"),
            ErrorParsingPlaintext => (415, "TODO"),
            ErrorParsingProtobuf => (415, "TODO"),
            ErrorParsingJson => (415, "TODO"),
        };

        let msg = if let Some(ref err) = self.err_message {
            let mut e = String::from(msg);
            e.push_str("; ");
            e.push_str(err.as_str());
            e
        } else {
            String::from(msg)
        };

        (status, msg)
    }
}

impl From<&DeserializeError> for String {
    fn from(err: &DeserializeError) -> Self {
        if let Some(ref msg) = err.err_message {
            msg.clone()
        } else {
            "Unknown Error".into()
        }
    }
}

impl Display for DeserializeError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), Error> {
        let s = String::from(self);
        fmt.write_str(s.as_str())
    }
}

// impl<S> IntoCatch<S> for Error {
//     type Catch = Self;

//     fn into_catch(self) -> Self {
//         self
//     }
// }

impl Catch for DeserializeError {
    type Body = String;
    type Future = FutureResult<http::Response<Self::Body>, TowerError>;

    fn catch(&mut self, _request: &http::Request<()>, error: TowerError) -> Self::Future {
        let (status, msg) = self.get_code_and_message();

        let response = http::response::Builder::new()
            .status(status)
            .header("content-type", "text/plain")
            .body(msg);

        if response.is_ok() {
            futures::future::ok(response.unwrap())
        } else {
            futures::future::err(error)
        }
    }
}
