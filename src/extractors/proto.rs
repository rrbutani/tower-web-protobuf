use tower_web::extract::Error;
use tower_web::util::buf_stream::{BufStream, Collect};
use tower_web::codegen::CallSite;
use tower_web::extract::ExtractFuture;
use tower_web::extract::Context;
use tower_web::extract::Extract;

use crate::common::*;
use crate::types::{MessagePlus, Proto};
use crate::errors::{DeserializeError, DeserializeErrorKind};
use crate::extensions::MessageParseStrategy;


pub struct MessageFuture<B: BufStream, M: MessagePlus> {
    collect: Collect<B, Vec<u8>>,
    message: Option<M>,
    strat: MessageParseStrategy,
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

        // We _should_ we trying to parse the message from the body:
        // let source = ctx.callsite().source();
        // if source != tower_web::codegen::Source::Body {
        //     unimplemented!("Err: {:?}" ctx.callsite().source())
        // }

        let strat = ctx.request().extensions().get::<MessageParseStrategy>();

        // If the user isn't using the Middleware, they get strict Protobuf
        // parsing regardless of the header:
        let strat = strat.map(|p| *p).unwrap_or(MessageParseStrategy::Proto);

        MessageFuture {
            collect: body.collect(),
            message: None,
            strat
        }
    }

    // TODO: make sure this is enforced..
    // Protobuf messages can only be extracted from a request body for now.
    fn requires_body(callsite: &CallSite) -> bool {
        drop(callsite);
        true
    }
}

// impl<T, U> Future for Collect<T, U>
// where
//     T: BufStream,
//     U: FromBufStream,
// {
//     type Item = U;
//     type Error = T::Error;

//     fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//         loop {
//             match try_ready!(self.stream.poll()) {
//                 Some(mut buf) => {
//                     let builder = self.builder.as_mut().expect("cannot poll after done");

//                     U::extend(builder, &mut buf);
//                 }
//                 None => {
//                     let builder = self.builder.take().expect("cannot poll after done");
//                     let value = U::build(builder);
//                     return Ok(value.into());
//                 }
//             }
//         }
//     }
// }

impl<B: BufStream, M: MessagePlus> ExtractFuture for MessageFuture<B, M> {
    type Item = Proto<M>;

    fn poll(&mut self) -> Poll<(), Error> {
        let resp = self.collect
            .poll()
            .map_err(|_| Error::invalid_argument(&String::from("internal error")));

        let bytes: Vec<u8> = try_ready!(resp);

        let msg_res: Result<M, DeserializeError> = match self.strat {
            MessageParseStrategy::Proto => {
                M::decode(bytes)
                    .map_err(|e| DeserializeError::new_with_message(DeserializeErrorKind::ErrorParsingProtobuf, e.to_string()))
            },
            MessageParseStrategy::Json => {
                unimplemented!()
            },
            MessageParseStrategy::None => {
                unimplemented!()
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
