use futures;

use {Async};

/// A wrapper that converts type-safe future into normal future
pub struct FutureWrapper<F>(Option<F>);

/// A type-safe future trait
pub trait Future: Sized {

    /// The type of value that this future will resolved with
    /// if it is successful
    type Item;

    /// The type of error that this future will resolve with
    /// if it fails in a normal fashion.
    type Error;

    /// Query this future to see if its value has become available,
    /// registering interest if it is not.
    ///
    /// The only difference of this method to the `futures::Future::poll`
    /// is that `self` is passed by value.
    ///
    /// See [documentation of futures](https://docs.rs/futures/0.1.14/futures/future/trait.Future.html#tymethod.poll)
    /// for more more information on how the method should be implemented.
    fn poll(self) -> Result<Async<Self::Item, Self>, Self::Error>;

    /// Convert this object into `futures::Future`
    fn into_future(self) -> FutureWrapper<Self> {
        FutureWrapper(Some(self))
    }
}

impl<F: Future> futures::Future for FutureWrapper<F> {
    type Item = F::Item;
    type Error = F::Error;
    fn poll(&mut self) -> Result<futures::Async<Self::Item>, Self::Error> {
        let val = self.0.take()
            .expect("finished future called again");
        match val.poll() {
            Ok(Async::Ready(x)) => Ok(futures::Async::Ready(x)),
            Ok(Async::NotReady(f)) => {
                self.0 = Some(f);
                Ok(futures::Async::NotReady)
            }
            Err(e) => Err(e),
        }
    }
}
