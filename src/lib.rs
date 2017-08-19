//! The crate provides by-value futures that make certain patterns easier
//! and more typesafe
//!
//! [Examples](https://github.com/tailhook/valuable-futures/tree/master/examples)
//! | [Github](https://github.com/tailhook/valuable-futures)
//!
//! There are two by-value future traits:
//!
//! * `Future` -- similar to normal future, but receives self by value. You
//!   must call `into_future()` to convert it to `futures::Future`
//! * `StateMachine` -- similar to `Future` but also receives a mutable
//!   pointer of `Supply` type. Basically having both mutable state, and
//!   type-safe state machine. Converted to `futures::Future` by calling
//!   `Supply::new`
//!
//! This crate also has it's own `Async` type that contains a new state in
//! `NotReady` option.
//!
#![warn(missing_docs)]
extern crate futures;

mod async;
mod future;
mod supply;

pub use async::Async;
pub use future::{Future, FutureWrapper};
pub use supply::{Supply, StateMachine};
