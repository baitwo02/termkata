//! termkata CLI 宿主：在终端驱动一轮练习。
//!
//! 本程序只做三件事：读命令 → 发事件 → 打印状态。
//! 所有功能逻辑在 `termkata-core`（状态机）与 `termkata-wordgen`（词汇生成）中。
//! 当前使用 `FakeApi`；真实 LLM API 接入后只替换 word source。

use std::future::Future;
use std::io::{self, BufRead, Write};
use std::pin::pin;
use std::task::{Context, Poll, Waker};

use termkata_core::round::{transition, Phase, RoundEvent, RoundState};
use termkata_wordgen::fake::FakeApi;
use termkata_wordgen::WordSource;

fn main() {
    let mut state = RoundState::new();
    let mut source = FakeApi::new();

    println!("termkata CLI —— 一轮练习：抽取 → 准备 → 口播");
    println!("命令：draw 抽取 / speak 开始口播 / end 结束口播 / help 帮助 / q 退出");
    print_state(&state);

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        print!("\n> ");
        io::stdout().flush().ok();

        let Some(Ok(line)) = lines.next() else {
            break; // EOF 或读取错误：退出
        };

        match line.trim() {
            "" => {}
            "help" => print_help(),
            "q" | "quit" => break,
            "draw" => {
                state = draw(state, &mut source);
                print_state(&state);
            }
            "speak" => {
                state = dispatch(state, RoundEvent::StartSpeaking);
                print_state(&state);
            }
            "end" => {
                state = dispatch(state, RoundEvent::EndSpeaking);
                print_state(&state);
            }
            other => println!("未知命令：{other}（输入 help 查看可用命令）"),
        }
    }

    println!("再见。");
}

/// CLI 显示层：把核心值翻译为界面文案（UI 文案不进 core）。
trait RoundDisplay {
    /// 当前词汇的显示文本（无词时给出占位符）。
    fn word_label(&self) -> &str;
}

impl RoundDisplay for RoundState {
    fn word_label(&self) -> &str {
        self.word().unwrap_or("(无)")
    }
}

/// 抽取：发 `RequestWord` → 等生成器 → 按结果发 `WordReceived` / `RequestFailed`。
fn draw(state: RoundState, source: &mut impl WordSource) -> RoundState {
    let state = dispatch(state, RoundEvent::RequestWord);
    if state.phase() != Phase::Generating {
        return state; // 当前阶段不允许抽取
    }

    println!("生成中…");
    match block_on(source.request_word()) {
        Ok(word) => transition(state, RoundEvent::WordReceived(word)),
        Err(message) => transition(state, RoundEvent::RequestFailed(message)),
    }
}

/// 把事件派发给状态机；无效事件被忽略并提示（返回原状态）。
fn dispatch(state: RoundState, event: RoundEvent) -> RoundState {
    let before = state.phase();
    let next = transition(state, event);
    if next.phase() == before {
        println!("当前阶段不能执行该操作（{}）", phase_label(before));
    }
    next
}

fn print_state(state: &RoundState) {
    match state.phase() {
        Phase::Ready => match state.error() {
            Some(error) => println!("[就绪] 上次抽取失败：{error}"),
            None => println!("[就绪] 等待抽取"),
        },
        Phase::Generating => println!("[生成中] 正在请求词汇…"),
        Phase::Preparing => println!("[准备] 当前词汇：{}", state.word_label()),
        Phase::Speaking => println!("[口播] 当前词汇：{}", state.word_label()),
    }
}

fn phase_label(phase: Phase) -> &'static str {
    match phase {
        Phase::Ready => "就绪",
        Phase::Generating => "生成中",
        Phase::Preparing => "准备",
        Phase::Speaking => "口播",
    }
}

fn print_help() {
    println!("命令：");
    println!("  draw   抽取一个词汇（FakeApi 轮换生成）");
    println!("  speak  准备结束，开始口播");
    println!("  end    结束口播，回到就绪");
    println!("  help   显示本帮助");
    println!("  q      退出");
}

/// 极简 block_on：FakeApi 的 future 立即就绪，因此只 poll 一次。
/// 接入真实异步 API 时，替换为正式运行时（如 tokio / pollster）。
fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    match fut.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("当前 block_on 只支持立即就绪的 future"),
    }
}
