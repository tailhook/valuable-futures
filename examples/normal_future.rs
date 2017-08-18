extern crate futures;
extern crate tokio_core;
extern crate tk_easyloop;

use std::time::Duration;

use futures::{Future, Async};
use tokio_core::reactor::Timeout;
use tk_easyloop::{run, timeout};


pub struct WaitAndPrint {
    num: u32,
    timeo: Option<Timeout>,
}

impl Future for WaitAndPrint {
    type Item = ();
    type Error = ();
    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        if let Some(ref mut timeo) = self.timeo {
            match timeo.poll().unwrap() {
                Async::NotReady => return Ok(Async::NotReady),
                Async::Ready(()) => {}
            }
        }
        loop {
            println!("Seconds left: {}", self.num);
            if self.num > 0 {
                self.num -= 1;
                let mut timeo = timeout(Duration::new(1, 0));
                // Note 1: we must poll to schedule wakeup
                // Note 2: if we poll, nobody guarantees that
                //         timer is not ready
                match timeo.poll().unwrap() {
                    Async::Ready(()) => continue,
                    Async::NotReady => {
                        self.timeo = Some(timeo);
                        return Ok(Async::NotReady)
                    }
                }
            } else {
                return Ok(Async::Ready(()));
            }
        }
    }
}

fn main() {
    run(|| WaitAndPrint {
        num: 5,
        timeo: None,
    }).unwrap();
}
