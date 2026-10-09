//! 极简执行器：驱动"立即就绪"的 future（当前只有 FakeApi）。
//!
//! 接入真实异步 API 时，替换为正式运行时（如 tokio / pollster）。

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

pub fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("当前 block_on 只支持立即就绪的 future"),
    }
}
