use tower_web::util::buf_stream::Collect;
use tower_web::error::IntoCatch;
use futures::Poll;
use ::extensions::MessageParseStrategy;
use tower_web::error::Catch;
use extractors::*;

use std::convert::{From, Into};
use std::ops::Deref;


/// A wrapper struct for a protobuf message type.
///
/// This has to exist because `impl<T> Trait for T` requires T to be 'covered'
/// by a local type (i.e. Proto<T>), when Self is used (I think). Self _is_
/// used by Extract (Future: ExtractFuture<Item = Self>) which I think is why:
/// ```
/// impl<M, B: BufStream> Extract<B> for M
/// where
///     M: 'static + Message + MessageWrapper<M>
/// {
///     type Future = Immediate<M>;
/// }
/// ```
/// doesn't work.
///
/// Niko's excellent [blog post](http://smallcultfollowing.com/babysteps/blog/2015/01/14/little-orphan-impls/) has a full writeup.
///
/// The effect of this is that you'll have to specify Proto<Message> instead
/// of just Message in your functions within `impl_web!()`. This, in turn hurts
/// testability (you have to wrap your Messages to pass them into the endpoint
/// functions) and kind of works counter to the PORTs (plain old Rust types)
/// philosophy. But, I'm pretty sure it's the best we can do without using
/// macros or modifying tower-web.
///
/// Into<M> for Proto<M> and From<M> for Proto<M> are implemented to ease the
/// pain a little bit. Into<M> for Proto<M> instead of From<Proto<M>> for M
/// for the same reasons we aren't just implementing Extract for M.
///
/// Deref is also implemented it should be possible to use Proto<M> as an M
/// for most everything.
/*
/// If it helps we could also implement Message on Proto though I can't really
/// fathom why this might help anything right now.*/
pub struct Proto<'a, M: MessagePlus>(pub &'a M);

/// A thin trait alias to make stuff more legible.
pub trait MessagePlus: Message + Default {}
impl<M: Message + Default> MessagePlus for M {}
// trait MessagePlus = Message + Default; // For when RFC 1733 lands

impl<'a, M: MessagePlus> Default for Proto<'a, M> {
    fn default() -> Self {
        Proto::new(&M::default())
    }
}

impl<'a, M: MessagePlus> Proto<'a, M> {
    pub fn message(&self) -> &M {
        self.0
    }

    pub fn new(ref message: &'a M) -> Self {
        Proto::<M>(&message)
    }
}

// Provides (_:&M).into() -> Proto<M> and Proto::<M>::from(_:&M) -> Proto<M>
impl<'a, M: MessagePlus> From<&'a M> for Proto<'a, M> {
    fn from(ref message: &'a M) -> Self {
        Self::new(&message)
    }
}

// This is bad but I don't know what the right answer is. I'm hoping someone
// replies to this: https://github.com/rust-lang/rust/issues/46205
// Also, this is the recommended approach in the docs for `Into`.
#[allow(incoherent_fundamental_impls)]
// Provides (_:Proto<M>).into() but probably not M::from(_:Proto<M>) -> M
// I am okay with this for now.
impl<'a, M: MessagePlus> Into<&'a M> for Proto<'a, M> {
    fn into(self) -> &'a M {
        self.0
    }
}

impl<'a, M: MessagePlus> Deref for Proto<'a, M> {
    type Target = M;

    fn deref(&self) -> &M {
        &self.0
    }
}

#[derive(Clone, Debug)]
enum ErrorKind {
    ///
    InvalidContentTypeForMessage,
    ErrorParsingJson,
    ErrorParsingProtobuf,
}

#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
    err_message: Option<String>
}

impl Error {
    fn new(kind: ErrorKind) -> Self {
        Self { kind, err_message: None }
    }

    fn new_with_message(kind: ErrorKind, message: String) -> Self {
        Self { kind, err_message: Some(message) }
    }

    fn get_code_and_message(&self) -> (u16, String) {
        let (status, msg) = match self.kind {
            InvalidHeadersForMessage => (415, "TODO"),
            ErrorParsingProtobuf => (415, "TODO"),
            ErrorParsingProtobuf => (415, "TODO"),
        };

        let msg = if let Some(err) = self.err_message {
            let e = String::from(msg);
                e.push_str("; ");
                e.push_str(err.as_str());
                e
        } else { String::from(msg) };

        (status, msg)
    }
}

// impl<S> IntoCatch<S> for Error {
//     type Catch = Self;

//     fn into_catch(self) -> Self {
//         self
//     }
// }

impl Catch for Error {
    type Body = &'static str;
    type Future = FutureResult<http::Response<Self::Body>, TowerError>;

    fn catch(&mut self, request: &http::Request<()>, error: TowerError) -> Self::Future {
        let (status, msg) = self.get_code_and_message();

        let response = http::response::Builder::new()
            .status(status)
            .header("content-type", "text/plain")
            .body(msg.as_str());

        if response.is_ok() {
            future::ok(response.unwrap())
        } else {
            future::err(error)
        }
    }
}

pub struct MessageFuture<'a, B, M: MessagePlus>
where
{
    collect: Collect<B, Vec<u8>>,
    message: Result<Proto<'a, M>, Error>,
    strat: MessageParseStrategy,
}

impl<'a, B, M: MessagePlus> ExtractFuture for MessageFuture<'a, B, M> {
    type Item = Proto<'a, M>;
    // type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        let bytes: Vec<u8> = try_ready!(self.collect);

        match self.strat {
            MessageParseStrategy::Proto => {
                M::decode(bytes)
                    .map_err(|e| Error::new_with_message(ErrorKind::ErrorParsingProtobuf, e.to_string()))
            },
            MessageParseStrategy::Json => {

            },
            MessageParseStrategy::None => {

            }
        }
    }

    fn extract(self) -> Self::Item {
        self.message.unwrap()
    }
}

impl<B, M> Extract<B> for Proto<'static, M>
where
    B: BufStream,
    M: 'static + MessagePlus
{
    type Future = MessageFuture<'static, B, M>;

    fn extract(_ctx: &Context) -> Self::Future {

    }

    fn extract_body(ctx: &Context, mut body: B) -> Self::Future {
        let strat = ctx.request.extensions().get::<MessageParseStrategy>();

        let strat = if let strat = Some(strat) {
            strat
        } else {
            // If the user isn't using the Middleware, they get strict
            // Protobuf parsing; regardless of header:
            MessageParseStrategy::Proto
        };

        MessageFuture {
            collect: body.collect(),
            // message: None,
            strat
        }
    }

    fn requires_body(callsite: &CallSite) -> bool {
        drop(callsite);
        true
    }
}

// trait Animal {
//     fn printName(_nom: Self) -> String;
// }

// use std::fmt::Debug;

// auto trait Debug2 {}

// impl<M: Debug> !M for Debug2 {}

// impl<M: AnimalPrintMarker> Animal for M {
//     fn printName(_nom: M) -> String
//     where
//         M: Debug
//     {
//         String::from("echo!")
//     }
// }

// impl<M: !Debug> Animal for M {
//     fn printName(_nom: M) -> String { String::from("eeek!") }
// }

// struct Wrapper<T>(Vec<T>);
// impl<T> Into<Vec<T>> for Wrapper<T> {
//     fn into(self) -> Vec<T> {
//         self.0
//     }
// }

// impl<'a, M: MessagePlus> Into<M> for Proto<'a, M> {

// }

// impl<'a, M: MessagePlus> Into<M> for Proto<'a, M> {

// }

// impl<'a, M: MessagePlus> From<Proto<'a, M>> for M {
//     fn from(&message_wrapper: &Proto<M>) -> M {
//         message_wrapper.0
//     }
// }

// impl<B, M> Extract<B> for Proto<M>
// where
//     B: BufStream,
//     M: 'static + Message + Default
// {
//     type Future = Immediate<Self>;

//     fn extract(ctx: Context)
// }

/*
pub struct Proto<'a, M: Message + Default> {
    message: &'a M
}

impl<'a, M: Message + Default> Proto<'a, M> {
    fn new(&message: &M) -> Self {
        Self { message: &message }
    }
}

// impl<'a, M: Message> From<Proto<'a, M>> for M {
//     fn from(proto: Proto<M>) -> &Self {
//         proto.message
//     }
// }

impl<'a, M: Message + Default> From<&'a M> for Proto<'a, M> {
    fn from(&message: &M) -> Self {
        Self::new(&message)
    }
}

// impl<B, M> Extract<B> for Proto<'static, M>
// where
//     B: BufStream,
//     M: 'static + Message + Default
// {
//     type Future = Immediate<Self>;

//     // fn extract(ctx: Context)
// }
*/
