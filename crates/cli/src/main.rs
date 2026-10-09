//! termkata CLI 宿主：组装各层并驱动主循环（MVU 中的 Runtime）。
//!
//! 层级：`command`（输入解析）→ `app`（编排）→ `render`（文本）；
//! `main` 只做 wiring 与 I/O，功能逻辑全部在 core / wordgen。

mod app;
mod command;
mod exec;
mod render;

use std::io::{self, BufRead, Write};

use termkata_wordgen::fake::FakeApi;
use termkata_wordgen::WordSource;

use crate::app::{App, Outcome};
use crate::command::{Command, Input};

fn main() {
    let mut app = App::new(FakeApi::new());

    println!("termkata CLI —— 一轮练习：抽取 → 准备 → 口播");
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
                    Outcome::Rejected(phase) => println!("{}", render::rejected_text(phase)),
                    Outcome::Updated => {
                        println!("{}", render::GENERATING_HINT);
                        exec::block_on(app.finish_draw());
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
fn report<S: WordSource>(outcome: Outcome, app: &App<S>) {
    if let Outcome::Rejected(phase) = outcome {
        println!("{}", render::rejected_text(phase));
    }
    println!("{}", render::state_text(app.state()));
}
