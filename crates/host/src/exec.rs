//! 极简执行器：驱动"立即就绪"的 future（假实现 / 单测用）。
//!
//! 真实异步 I/O 由宿主的正式运行时驱动（CLI 用 tokio；Tauri 用 tauri::async_runtime）。

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
