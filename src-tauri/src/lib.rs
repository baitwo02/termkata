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

/// 宿主状态：编排器（权威状态 + 词汇来源）包在互斥锁里。
struct AppState(Mutex<App<FakeApi>>);

impl AppState {
    fn new() -> Self {
        Self(Mutex::new(App::new(FakeApi::new())))
    }
}

/// 读取当前状态。
#[tauri::command]
fn get_state(state: State<'_, AppState>) -> RoundState {
    state.0.lock().unwrap().state().clone()
}

/// 发起抽取：就绪 → 生成中。
#[tauri::command]
fn begin_draw(state: State<'_, AppState>) -> RoundState {
    let mut app = state.0.lock().unwrap();
    let _ = app.begin_draw();
    app.state().clone()
}

/// 等待生成结果并应用：生成中 → 准备（或失败回到就绪）。
///
/// FakeApi 立即就绪，用极简执行器驱动；真实异步 API 接入时改为 async 命令 + 正式运行时。
#[tauri::command]
fn finish_draw(state: State<'_, AppState>) -> RoundState {
    let mut app = state.0.lock().unwrap();
    exec::block_on(app.finish_draw());
    app.state().clone()
}

/// 开始口播：准备 → 口播。
#[tauri::command]
fn start_speaking(state: State<'_, AppState>) -> RoundState {
    let mut app = state.0.lock().unwrap();
    let _ = app.start_speaking();
    app.state().clone()
}

/// 结束口播：口播 → 就绪。
#[tauri::command]
fn end_speaking(state: State<'_, AppState>) -> RoundState {
    let mut app = state.0.lock().unwrap();
    let _ = app.end_speaking();
    app.state().clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
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
