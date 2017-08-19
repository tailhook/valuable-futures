use futures;

use {Async};


/// A wrapper that provides mutable state to future-like `StateMachine`
pub struct Supply<S, M>(S, Option<M>);

/// A type-safe future-like trait that has also borrowed mutable state
pub trait StateMachine: Sized {
    /// A data, mutable pointer of which is supplied as the second
    /// argument of the `poll()` method.
    type Supply;

    /// The type of value that this future will resolved with
    /// if it is successful
    type Item;

    /// The type of error that this future will resolve with
    /// if it fails in a normal fashion.
    type Error;

    /// Query this future to see if its value has become available,
    /// registering interest if it is not.
    ///
    /// The difference of this method to the `futures::Future::poll`
    /// is that `self` is passed by value and supply is passed by a
    /// mutable reference.
    ///
    /// See [documentation of futures](https://docs.rs/futures/0.1.14/futures/future/trait.Future.html#tymethod.poll)
    /// for more more information on how the method should be implemented.
    fn poll(self, &mut Self::Supply)
        -> Result<Async<Self::Item, Self>, Self::Error>;
}

impl<S, M> Supply<S, M> {
    /// Create a `Future` for the `StateMachine` by providing a mutable
    /// state to it
    pub fn new(supply: S, state: M) -> Supply<S, M> {
        Supply(supply, Some(state))
    }
}

// We could implement futures::IntoFuture by using FutureWrapper
impl<S, M: StateMachine<Supply=S>> futures::Future for Supply<S, M> {
    type Item = M::Item;
    type Error = M::Error;
    fn poll(&mut self) -> Result<futures::Async<Self::Item>, Self::Error> {
        let val = self.1.take()
            .expect("finished future called again");
        match val.poll(&mut self.0) {
            Ok(Async::Ready(x)) => Ok(futures::Async::Ready(x)),
            Ok(Async::NotReady(f)) => {
                self.1 = Some(f);
                Ok(futures::Async::NotReady)
            }
            Err(e) => Err(e),
        }
    }
}
