extern crate futures;
extern crate tokio_core;
extern crate valuable_futures;
extern crate tk_easyloop;

use std::time::Duration;

use futures::{Future as OriginalFuture};
use valuable_futures::{Future, Async};
use tokio_core::reactor::Timeout;
use tk_easyloop::{run, timeout};


pub enum WaitAndPrint {
    Wait(u32, Timeout),
    Print(u32),
}

impl Future for WaitAndPrint {
    type Item = ();
    type Error = ();
    fn poll(self) -> Result<Async<Self::Item, Self>, Self::Error> {
        use self::WaitAndPrint::*;
        let mut state = self;
        loop {
            state = match state {
                Wait(n, mut t) => {
                    if t.poll().unwrap().is_ready() {
                        Print(n)
                    } else {
                        return Ok(Async::NotReady(Wait(n, t)));
                    }
                }
                Print(n) => {
                    println!("Seconds left: {}", n);
                    if n > 0 {
                        Wait(n-1, timeout(Duration::new(1, 0)))
                    } else {
                        return Ok(Async::Ready(()));
                    }
                }
            }
        }
    }
}

fn main() {
    run(|| WaitAndPrint::Print(5).into_future()).unwrap();
}
