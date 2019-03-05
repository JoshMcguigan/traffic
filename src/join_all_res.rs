// this is a slightly modified version of join_all from futures-rs

use std::prelude::v1::*;

use std::mem;

use futures::{Future, IntoFuture, Poll, Async};

#[derive(Debug)]
enum ElemState<T> where T: Future {
    Pending(T),
    Done(Result<T::Item, T::Error>),
}

#[must_use = "futures do nothing unless polled"]
pub struct JoinAll<I>
    where I: IntoIterator,
          I::Item: IntoFuture,
{
    elems: Vec<ElemState<<I::Item as IntoFuture>::Future>>,
}

pub fn join_all<I>(i: I) -> JoinAll<I>
    where I: IntoIterator,
          I::Item: IntoFuture,
{
    let elems = i.into_iter().map(|f| {
        ElemState::Pending(f.into_future())
    }).collect();
    JoinAll { elems: elems }
}

impl<I> Future for JoinAll<I>
    where I: IntoIterator,
          I::Item: IntoFuture,
{
    type Item = Vec<Result<<I::Item as IntoFuture>::Item, <I::Item as IntoFuture>::Error>>;
    type Error = (); // never type would be better here, but it is not stabilized


    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut all_done = true;

        for idx in 0 .. self.elems.len() {
            let done = match self.elems[idx] {
                ElemState::Pending(ref mut t) => {
                    match t.poll() {
                        Ok(Async::Ready(v)) => {
                            ElemState::Done(Ok(v))
                        },
                        Ok(Async::NotReady) => {
                            all_done = false;
                            continue
                        }
                        Err(e) => {
                            ElemState::Done(Err(e))
                        },
                    }
                }
                ElemState::Done(ref mut _v) => continue,
            };
            self.elems[idx] = done;
        }

        if all_done {
            let elems = mem::replace(&mut self.elems, Vec::new());
            let result = elems.into_iter().map(|e| {
                match e {
                    ElemState::Done(t) => t,
                    _ => unreachable!(),
                }
            }).collect();
            Ok(Async::Ready(result))
        } else {
            Ok(Async::NotReady)
        }
    }
}
