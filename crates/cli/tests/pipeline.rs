use std::io::Write;
use std::process::{Command, Stdio};

/// 启动 CLI，喂入命令，收集输出。
fn run_cli(input: &str) -> (String, bool) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_termkata"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("启动 CLI 失败");

    child
        .stdin
        .as_mut()
        .expect("stdin 不可用")
        .write_all(input.as_bytes())
        .expect("写入 stdin 失败");

    let output = child.wait_with_output().expect("等待 CLI 退出失败");
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        output.status.success(),
    )
}

#[test]
fn full_round_pipeline() {
    let (stdout, success) = run_cli("draw\nspeak\nend\ndraw\nquit\n");

    assert!(success, "CLI 应正常退出：{stdout}");
    assert!(stdout.contains("生成中"), "应显示生成中状态：{stdout}");
    assert!(
        stdout.contains("SSA（静态单赋值）"),
        "第一次抽取应得到默认词表第一个词：{stdout}"
    );
    assert!(stdout.contains("[准备]"), "应进入准备阶段：{stdout}");
    assert!(stdout.contains("[口播]"), "应进入口播阶段：{stdout}");
    assert!(stdout.contains("[就绪]"), "结束口播后应回到就绪：{stdout}");
    assert!(
        stdout.contains("逃逸分析"),
        "第二次抽取应轮换到第二个词：{stdout}"
    );
}

#[test]
fn rejects_command_out_of_phase() {
    let (stdout, success) = run_cli("speak\nquit\n");

    assert!(success, "CLI 应正常退出：{stdout}");
    assert!(
        stdout.contains("当前阶段不能执行该操作"),
        "就绪阶段直接 speak 应被拒绝：{stdout}"
    );
}
