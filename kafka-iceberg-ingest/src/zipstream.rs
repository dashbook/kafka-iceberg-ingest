use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{stream::FuturesUnordered, Future, Stream};

pub struct ZipStream<S, T> {
    inner: Option<ZipStreamInner<S, T>>,
}
impl<S: Unpin, T> Unpin for ZipStream<S, T> {}

struct ZipStreamInner<S, T> {
    pending: FuturesUnordered<StreamFuture<S>>,
    waiting: Vec<Slot<S, T>>,
}

enum Slot<S, T> {
    Empty,
    Filled(S, T),
}

impl<S, T> Slot<S, T> {
    fn take(&mut self) -> (S, T) {
        match std::mem::replace(self, Slot::Empty) {
            Slot::Filled(s, t) => (s, t),
            Slot::Empty => unreachable!(),
        }
    }
}

impl<S, T> ZipStream<S, T>
where
    S: Stream<Item = T> + Unpin,
{
    pub fn new(streams: impl IntoIterator<Item = S>) -> Self {
        let pending: FuturesUnordered<_> = streams
            .into_iter()
            .enumerate()
            .map(|(i, s)| StreamFuture::new(i, s))
            .collect();

        let mut waiting = Vec::with_capacity(pending.len());
        for _ in 0..pending.len() {
            waiting.push(Slot::Empty);
        }

        Self {
            inner: Some(ZipStreamInner { waiting, pending }),
        }
    }
}

impl<S, T> Stream for ZipStream<S, T>
where
    S: Stream<Item = T> + Unpin,
{
    type Item = Vec<T>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Vec<T>>> {
        let inner = match &mut self.inner {
            Some(inner) => inner,
            None => return Poll::Ready(None),
        };

        loop {
            match Pin::new(&mut inner.pending).poll_next(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Some(Some((i, item, stream)))) => {
                    inner.waiting[i] = Slot::Filled(stream, item);
                    // go around loop
                }
                Poll::Ready(Some(None)) => {
                    // A stream has finished
                    self.inner = None;
                    return Poll::Ready(None);
                }
                Poll::Ready(None) => {
                    // All streams have yielded an item.
                    let mut vec = Vec::with_capacity(inner.waiting.len());
                    for (i, slot) in inner.waiting.iter_mut().enumerate() {
                        let (s, t) = slot.take();
                        inner.pending.push(StreamFuture::new(i, s));
                        vec.push(t);
                    }
                    return Poll::Ready(Some(vec));
                }
            }
        }
    }
}

struct StreamFuture<S> {
    index: usize,
    stream: Option<S>,
}
impl<S: Unpin> StreamFuture<S> {
    fn new(index: usize, stream: S) -> Self {
        Self {
            index,
            stream: Some(stream),
        }
    }
    fn project(&mut self) -> Pin<&mut S> {
        match self.stream {
            Some(ref mut stream) => Pin::new(stream),
            None => unreachable!(),
        }
    }
}

impl<S: Stream + Unpin> Future for StreamFuture<S> {
    type Output = Option<(usize, S::Item, S)>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project().poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(out)) => {
                Poll::Ready(Some((self.index, out, self.stream.take().unwrap())))
            }
            Poll::Ready(None) => Poll::Ready(None),
        }
    }
}
