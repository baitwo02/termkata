//! 开发用假实现：按固定词表轮换返回。

use std::future::Future;

use crate::WordSource;

/// 假实现：轮换返回固定词表；词表为空时返回错误。
#[derive(Debug, Clone)]
pub struct FakeApi {
    words: Vec<String>,
    next: usize,
}

impl FakeApi {
    /// 默认词表（5 个词轮换）。
    pub fn new() -> Self {
        Self::with_words([
            "SSA（静态单赋值）",
            "逃逸分析",
            "写时复制（CoW）",
            "KV 缓存",
            "光栅化",
        ])
    }

    /// 使用自定义词表。
    pub fn with_words<I, S>(words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            words: words.into_iter().map(Into::into).collect(),
            next: 0,
        }
    }

    /// 轮换取下一个词（同步逻辑；trait 方法只是包一层立即就绪的 future）。
    fn next_word(&mut self) -> Result<String, String> {
        if self.words.is_empty() {
            return Err("词表为空".to_string());
        }
        let word = self.words[self.next % self.words.len()].clone();
        self.next += 1;
        Ok(word)
    }
}

impl Default for FakeApi {
    fn default() -> Self {
        Self::new()
    }
}

impl WordSource for FakeApi {
    fn request_word(&mut self) -> impl Future<Output = Result<String, String>> + Send {
        let result = self.next_word();
        async move { result }
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    use super::*;

    /// 测试用极简 block_on：假实现的 future 立即就绪，无需真正的 executor。
    fn block_on<F: Future>(fut: F) -> F::Output {
        let mut fut = pin!(fut);
        let mut cx = Context::from_waker(Waker::noop());
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("fake future 应当立即就绪"),
        }
    }

    #[test]
    fn rotates_through_default_words() {
        let mut api = FakeApi::new();

        let first = block_on(api.request_word()).unwrap();
        let second = block_on(api.request_word()).unwrap();
        assert_eq!(first, "SSA（静态单赋值）");
        assert_eq!(second, "逃逸分析");

        // 抽满一轮后回到开头
        for _ in 0..3 {
            block_on(api.request_word()).unwrap();
        }
        assert_eq!(block_on(api.request_word()).unwrap(), first);
    }

    #[test]
    fn uses_custom_word_list() {
        let mut api = FakeApi::with_words(["甲", "乙"]);
        assert_eq!(block_on(api.request_word()).unwrap(), "甲");
        assert_eq!(block_on(api.request_word()).unwrap(), "乙");
        assert_eq!(block_on(api.request_word()).unwrap(), "甲");
    }

    #[test]
    fn errors_on_empty_word_list() {
        let mut api = FakeApi::with_words(Vec::<String>::new());
        assert!(block_on(api.request_word()).is_err());
    }
}
