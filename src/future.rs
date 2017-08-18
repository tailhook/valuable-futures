use futures;

use {Async};

pub struct FutureWrapper<F>(Option<F>);

pub trait Future: Sized {
    type Item;
    type Error;
    fn poll(self) -> Result<Async<Self::Item, Self>, Self::Error>;
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
