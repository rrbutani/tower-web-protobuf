use core::fmt::Formatter;
use std::convert::From;
use std::fmt::Debug;
use std::ops::Deref;

use prost::Message;
use serde::{de::DeserializeOwned, Serialize};

/// A wrapper struct for a protobuf message type.
///
/// This has to exist because `impl<T> Trait for T` requires T to be 'covered'
/// by a local type (i.e. Proto<T>), when Self is used (I think). Self _is_
/// used by Extract (Future: ExtractFuture<Item = Self>) which I think is why:
/// ```compile_fail
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
///
// If it helps we could also implement Message on Proto though I can't really
// fathom why this might help anything right now.
pub struct Proto<M: MessagePlus>(pub M);

/// A thin trait alias to make stuff more legible.
pub trait MessagePlus: Message + DeserializeOwned + Serialize + Default {}
impl<M: Message + DeserializeOwned + Serialize + Default> MessagePlus for M {}
// trait MessagePlus = Message + DeserializeOwned + Serialize + Default; // For when RFC 1733 lands

impl<M: MessagePlus> Default for Proto<M> {
    fn default() -> Self {
        Proto::new(M::default())
    }
}

impl<M: MessagePlus> Proto<M> {
    pub fn move_inner(self) -> M {
        self.0
    }

    pub fn new(message: M) -> Self {
        Proto::<M>(message)
    }
}

// Provides (_:&M).into() -> Proto<M> and Proto::<M>::from(_:&M) -> Proto<M>
impl<M: MessagePlus> From<M> for Proto<M> {
    fn from(message: M) -> Self {
        Self::new(message)
    }
}

// // This is bad but I don't know what the right answer is. I'm hoping someone
// // replies to this: https://github.com/rust-lang/rust/issues/46205
// // Also, this is the recommended approach in the docs for `Into`.
// #[allow(incoherent_fundamental_impls)]
// // Provides (_:Proto<M>).into() but probably not M::from(_:Proto<M>) -> M
// // I am okay with this for now.
// impl<M: MessagePlus> Into<M> for Proto<M> {
//     fn into(self) -> M {
//         self.0
//     }
// }

impl<M: MessagePlus> Deref for Proto<M> {
    type Target = M;

    fn deref(&self) -> &M {
        &self.0
    }
}

impl<M: MessagePlus> AsRef<M> for Proto<M> {
    fn as_ref(&self) -> &M {
        self.deref()
    }
}

impl<M: MessagePlus + Debug> Debug for Proto<M> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        self.0.fmt(f)
    }
}

impl<M: MessagePlus + Clone> Clone for Proto<M> {
    fn clone(&self) -> Self {
        Self(M::clone(&self.0))
    }
}
