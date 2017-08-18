use futures;

use {Async};


pub struct Supply<S, M>(S, Option<M>);

pub trait StateMachine: Sized {
    type Supply;
    type Item;
    type Error;
    fn poll(self, &mut Self::Supply)
        -> Result<Async<Self::Item, Self>, Self::Error>;
}

impl<S, M> Supply<S, M> {
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
