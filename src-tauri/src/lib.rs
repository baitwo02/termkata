//! termkata 桌面壳：Tauri 宿主。
//!
//! 宿主持有权威状态（`App<FakeApi>`）；前端只通过命令读状态、发事件。
//! 命令的返回值就是最新状态快照（IPC 会序列化为 JSON）。

use std::sync::Mutex;

use tauri::State;

use termkata_core::round::RoundState;
use termkata_host::app::App;
use termkata_host::exec;
use termkata_wordgen::fake::FakeApi;

/// 供 Tauri 命令共享的宿主编排器：`App`（权威状态 + 词汇来源）包在互斥锁里。
struct SharedApp {
    app: Mutex<App<FakeApi>>,
}

impl SharedApp {
    fn new() -> Self {
        Self {
            app: Mutex::new(App::new(FakeApi::new())),
        }
    }
}

/// 读取当前状态。
#[tauri::command]
fn get_state(state: State<'_, SharedApp>) -> RoundState {
    eprintln!("[ipc] get_state"); // 临时诊断日志，定位后删除
    state.app.lock().unwrap().state().clone()
}

/// 发起抽取：就绪 → 生成中。
#[tauri::command]
fn begin_draw(state: State<'_, SharedApp>) -> RoundState {
    eprintln!("[ipc] begin_draw"); // 临时诊断日志，定位后删除
    let mut app = state.app.lock().unwrap();
    let _ = app.begin_draw();
    app.state().clone()
}

/// 等待生成结果并应用：生成中 → 准备（或失败回到就绪）。
///
/// FakeApi 立即就绪，用极简执行器驱动；真实异步 API 接入时改为 async 命令 + 正式运行时。
#[tauri::command]
fn finish_draw(state: State<'_, SharedApp>) -> RoundState {
    eprintln!("[ipc] finish_draw"); // 临时诊断日志，定位后删除
    let mut app = state.app.lock().unwrap();
    exec::block_on(app.finish_draw());
    app.state().clone()
}

/// 开始口播：准备 → 口播。
#[tauri::command]
fn start_speaking(state: State<'_, SharedApp>) -> RoundState {
    eprintln!("[ipc] start_speaking"); // 临时诊断日志，定位后删除
    let mut app = state.app.lock().unwrap();
    let _ = app.start_speaking();
    app.state().clone()
}

/// 结束口播：口播 → 就绪。
#[tauri::command]
fn end_speaking(state: State<'_, SharedApp>) -> RoundState {
    eprintln!("[ipc] end_speaking"); // 临时诊断日志，定位后删除
    let mut app = state.app.lock().unwrap();
    let _ = app.end_speaking();
    app.state().clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(SharedApp::new())
        .invoke_handler(tauri::generate_handler![
            get_state,
            begin_draw,
            finish_draw,
            start_speaking,
            end_speaking
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
