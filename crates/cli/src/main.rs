//! termkata CLI 宿主：组装各层并驱动主循环（MVU 中的 Runtime）。
//!
//! 层级：`command`（输入解析）→ 主循环（编排）→ `render`（文本）；
//! `main` 只做 wiring 与 I/O，功能逻辑全部在 core / wordgen。
//!
//! 词汇来源：默认 DeepSeek API（key 来自 `.env` 或环境变量）；
//! `--fake` 使用固定词表假实现（开发调试）。

mod command;
mod render;

use std::io::{self, BufRead, Write};

use termkata_host::app::{App, TransitionStatus};
use termkata_wordgen::any::AnySource;
use termkata_wordgen::WordSource;

use crate::command::{Command, Input};

#[tokio::main]
async fn main() {
    termkata_host::env::load_dotenv();

    if has_flag("--fake") {
        run(AnySource::fake(), "假实现（--fake，固定词表轮换）").await;
        return;
    }

    match AnySource::deepseek_from_env() {
        Ok(source) => run(source, "DeepSeek API（deepseek-chat）").await,
        Err(message) => {
            eprintln!("{message}");
            eprintln!("开发调试可用：cargo run -p termkata-cli -- --fake");
            std::process::exit(1);
        }
    }
}

/// 是否有某个命令行开关。
fn has_flag(flag: &str) -> bool {
    std::env::args().skip(1).any(|arg| arg == flag)
}

/// 用给定词汇来源驱动交互循环。
async fn run(source: AnySource, source_label: &str) {
    let mut app = App::new(source);

    println!("termkata CLI —— 一轮练习：抽取 → 准备 → 口播");
    println!("词汇来源：{source_label}");
    println!("命令：draw 抽取 / speak 开始口播 / end 结束口播 / help 帮助 / q 退出");
    println!("{}", render::state_text(app.state()));

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        print!("\n> ");
        io::stdout().flush().ok();

        let Some(Ok(line)) = lines.next() else {
            break; // EOF 或读取错误：退出
        };

        match command::parse(&line) {
            Input::Empty => {}
            Input::Unknown(text) => println!("未知命令：{text}（输入 help 查看可用命令）"),
            Input::Command(Command::Help) => println!("{}", render::help_text()),
            Input::Command(Command::Quit) => break,
            Input::Command(Command::Draw) => {
                match app.begin_draw() {
                    TransitionStatus::Rejected(phase) => {
                        println!("{}", render::rejected_text(phase))
                    }
                    TransitionStatus::Applied => {
                        println!("{}", render::GENERATING_HINT);
                        app.finish_draw().await;
                    }
                }
                println!("{}", render::state_text(app.state()));
            }
            Input::Command(Command::Speak) => report(app.start_speaking(), &app),
            Input::Command(Command::End) => report(app.end_speaking(), &app),
        }
    }

    println!("再见。");
}

/// 打印普通命令的结果：被拒绝时先提示，随后总是打印当前状态。
fn report<S: WordSource>(status: TransitionStatus, app: &App<S>) {
    if let TransitionStatus::Rejected(phase) = status {
        println!("{}", render::rejected_text(phase));
    }
    println!("{}", render::state_text(app.state()));
}
