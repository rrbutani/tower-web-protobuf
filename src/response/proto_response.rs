use std::io::Cursor;
use std::ops::Deref;

use bytes::Bytes;
use bytes::BytesMut;
use http::status::StatusCode;
use serde_json;
use serde_plain;
use tower_web::error::Error;
use tower_web::response::Context;
use tower_web::response::Response;
use tower_web::response::Serializer;
use tower_web::util::buf_stream::BufStream;

use crate::common::*;
use crate::extensions::MessageEncodeStrategy;
use crate::extensions::MessageStrategy;
use crate::types::MessagePlus;
use crate::types::Proto;

// Let's make a newtype around Bytes:
#[doc(hidden)]
#[derive(Debug)]
pub struct BytesWrapper(Bytes);

impl Deref for BytesWrapper {
    type Target = Bytes;

    fn deref(&self) -> &Bytes {
        &self.0
    }
}

impl From<Bytes> for BytesWrapper {
    fn from(bytes: Bytes) -> Self {
        BytesWrapper(bytes)
    }
}

// So that we can implement BufStream on Bytes, our way:
// (specialization, but the hard way)
impl BufStream for BytesWrapper {
    type Item = Cursor<Bytes>;

    // This is why we need our own impl; tower-web requires the Body type in a
    // Response impl to be of type BufStream<Error = tower_web::error::Error>.
    type Error = tower_web::error::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // This impl is borrowed basically verbatim from tower-web.
        // (https://docs.rs/tower-web/0.3.6/src/tower_web/util/buf_stream/bytes.rs.html#8-24)
        use std::mem;

        if self.is_empty() {
            return Ok(None.into());
        }

        let bytes = mem::replace(self, BytesWrapper(Bytes::new()));
        let buf = Cursor::new(bytes.0);

        Ok(Some(buf).into())
    }
}

#[inline]
fn serialize_proto<M: MessagePlus>(message: &Proto<M>) -> Result<BytesMut, Error> {
    let mut buf = BytesMut::with_capacity(message.encoded_len());

    message
        .encode(&mut buf)
        .map_err(|err| {
            Error::new(
                &format!("{}", err),
                "Serialization Error: Insufficient Capacity",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
        .map(|_| buf)
}

impl<M> Response for Proto<M>
where
    M: MessagePlus,
{
    type Buf = Cursor<Bytes>;
    type Body = BytesWrapper;

    fn into_http<S: Serializer>(
        self,
        context: &Context<S>,
    ) -> Result<HttpResponse<Self::Body>, Error> {
        use MessageStrategy::*;

        let strat: MessageEncodeStrategy = context
            .request()
            .extensions()
            .get()
            .cloned()
            .unwrap_or_default();
        let buf = match *strat {
            NamedProto(ref name) => {
                // TODO: message name check
                serialize_proto(&self)?
            }
            Proto => serialize_proto(&self)?,
            Json => serde_json::to_vec_pretty(&*self)
                .map(|vec| vec.into())
                .map_err(|err| {
                    Error::new(
                        &format!("{}", err),
                        "Serialization Error: serde_json",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )
                })?,
            Plaintext => serde_plain::to_string(&*self)
                .map(|str| str.into())
                .map_err(|err| {
                    Error::new(
                        &format!("{}", err),
                        "Serialization Error: serde_plain",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )
                })?,
        };

        let buf = buf.freeze();

        http::Response::builder()
            .header("Content-Type", strat.content_type())
            .body(buf.into())
            .map_err(|err| {
                Error::new(
                    &format!("{}", err),
                    "Response Builder Error",
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })
    }
}
