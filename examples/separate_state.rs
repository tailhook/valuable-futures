extern crate futures;
extern crate tokio_core;
extern crate valuable_futures;
extern crate tk_easyloop;

use std::time::Duration;

use futures::{Future as OriginalFuture};
use valuable_futures::{StateMachine, Supply, Async};
use tokio_core::reactor::Timeout;
use tk_easyloop::{run, timeout};

pub enum WaitAndPrint {
    Wait(Timeout),
    Print,
}


impl StateMachine for WaitAndPrint {
    type Supply = u32;
    type Item = ();
    type Error = ();
    fn poll(self, n: &mut u32)
        -> Result<Async<Self::Item, Self>, Self::Error>
    {
        use self::WaitAndPrint::*;
        let mut state = self;
        loop {
            state = match state {
                Wait(mut t) => {
                    if t.poll().unwrap().is_ready() {
                        Print
                    } else {
                        return Ok(Async::NotReady(Wait(t)));
                    }
                }
                Print => {
                    println!("Seconds left: {}", n);
                    if *n > 0 {
                        *n -= 1;
                        Wait(timeout(Duration::new(1, 0)))
                    } else {
                        return Ok(Async::Ready(()));
                    }
                }
            }
        }
    }
}

fn main() {
    run(|| Supply::new(5, WaitAndPrint::Print)).unwrap();
}
