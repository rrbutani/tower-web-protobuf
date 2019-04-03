use tower_web::extract::Error;
use tower_web::util::buf_stream::{BufStream, Collect};
use tower_web::codegen::CallSite;
use tower_web::extract::ExtractFuture;
use tower_web::extract::Context;
use tower_web::extract::Extract;

use crate::common::*;
use crate::types::{MessagePlus, Proto};
use crate::errors::{DeserializeError, DeserializeErrorKind};
use crate::extensions::{MessageParseStrategy, MessageStrategy};


pub struct MessageFuture<B: BufStream, M: MessagePlus> {
    collect: Collect<B, Vec<u8>>,
    message: Option<M>,
    parse_strat: MessageParseStrategy,
}

impl<B, M> Extract<B> for Proto<M>
where
    B: BufStream,
    M: 'static + MessagePlus
{
    type Future = MessageFuture<B, M>;

    fn extract(_ctx: &Context) -> Self::Future {
        // Since protobuf messages can only be extracted from the body, we
        // should never get here.

        // unimplemented!("Err: {:?}", ctx.callsite().source())
        unimplemented!("Err: can only extract protobuf messages from a request body")
    }

    fn extract_body(ctx: &Context, body: B) -> Self::Future {

        // We _should_ be trying to parse the message from the body:
        // let source = ctx.callsite().source();
        // if source != tower_web::codegen::Source::Body {
        //     unimplemented!("Err: {:?}" ctx.callsite().source())
        // }
        //
        // Unfortunately since callsite().source() is private, we can't do the
        // check above.

        // If the user isn't using the Middleware, they get the Default:
        let strat = ctx.request().extensions()
            .get::<MessageParseStrategy>()
            .map(|p| p.clone())
            .unwrap_or_default();

        MessageFuture {
            collect: body.collect(),
            message: None,
            parse_strat: strat
        }
    }

    // TODO: make sure this is enforced..
    // Protobuf messages can only be extracted from a request body for now.
    fn requires_body(callsite: &CallSite) -> bool {
        drop(callsite);
        true
    }
}

impl<B: BufStream, M: MessagePlus> ExtractFuture for MessageFuture<B, M> {
    type Item = Proto<M>;

    fn poll(&mut self) -> Poll<(), Error> {
        let resp = self.collect
            .poll()
            .map_err(|_| Error::invalid_argument(&String::from("internal error")));

        let bytes: Vec<u8> = try_ready!(resp);

        let msg_res: Result<M, DeserializeError> = match *self.parse_strat {
            MessageStrategy::NamedProto(ref name) => {
                // TODO: check message name
                M::decode(bytes)
                    .map_err(|e| DeserializeError::new_with_error(DeserializeErrorKind::ErrorParsingProtobuf, e))
            }

            MessageStrategy::Proto => {
                M::decode(bytes)
                    .map_err(|e| DeserializeError::new_with_error(DeserializeErrorKind::ErrorParsingProtobuf, e))
            },
            MessageStrategy::Json => {
                serde_json::from_slice(&bytes)
                    .map_err(|e| DeserializeError::new_with_error(DeserializeErrorKind::ErrorParsingJson, e))
            },
            MessageStrategy::Plaintext => {
                serde_plain::from_str(
                    String::from_utf8(bytes)
                        .map_err(|e| Error::invalid_argument(&e))?
                        .as_str())
                    .map_err(|e| DeserializeError::new_with_error(DeserializeErrorKind::ErrorParsingPlaintext, e))
            }
        };

        match msg_res {
            Ok(msg) => {
                self.message = Some(msg);
                Ok(futures::Async::Ready(()))
            },

            Err(err) => {
                Err(Error::invalid_argument(&err))
            }
        }
    }

    fn extract(self) -> Self::Item {
        Proto::from(self.message.unwrap())
    }
}
