//! termkata 桌面壳：Tauri 宿主。
//!
//! 宿主持有权威状态（`App<AnySource>`）；前端只通过命令读状态、发意向。
//! 命令返回值是状态快照（IPC 序列化为 JSON）。
//!
//! 命令为 async：`finish_draw` 会等待真实网络（LLM API）。
//! 状态锁用 `tauri::async_runtime::Mutex`（guard 可跨 await，见 ADR 0005）。

use tauri::async_runtime::Mutex;
use tauri::State;

use termkata_core::round::RoundState;
use termkata_host::app::App;
use termkata_wordgen::any::AnySource;

/// 供 Tauri 命令共享的宿主编排器：`App`（权威状态 + 词汇来源）包在互斥锁里。
struct SharedApp {
    app: Mutex<App<AnySource>>,
}

impl SharedApp {
    /// 从环境装配词汇来源（`.env` 已由 [`run`] 加载）。
    ///
    /// 缺少 API key 时不崩溃：抽取时经 `RequestFailed` 给出清晰错误，
    /// 不静默回退假实现。
    fn new() -> Self {
        let source = AnySource::deepseek_from_env().unwrap_or_else(AnySource::unconfigured);
        Self {
            app: Mutex::new(App::new(source)),
        }
    }
}

/// 读取当前状态。
#[tauri::command]
async fn get_state(shared: State<'_, SharedApp>) -> Result<RoundState, String> {
    let app = shared.app.lock().await;
    Ok(app.state().clone())
}

/// 发起抽取：就绪 → 生成中。
#[tauri::command]
async fn begin_draw(shared: State<'_, SharedApp>) -> Result<RoundState, String> {
    let mut app = shared.app.lock().await;
    let _ = app.begin_draw();
    Ok(app.state().clone())
}

/// 等待生成结果并应用：生成中 → 准备（或失败回到就绪）。
///
/// 持锁等待网络：命令语义即"一次一个"（生成中 UI 禁用抽取按钮）。
/// 若将来需要并发或取消，再把 I/O 移到锁外（ADR 0005 记录该演进方向）。
#[tauri::command]
async fn finish_draw(shared: State<'_, SharedApp>) -> Result<RoundState, String> {
    let mut app = shared.app.lock().await;
    app.finish_draw().await;
    Ok(app.state().clone())
}

/// 开始口播：准备 → 口播。
#[tauri::command]
async fn start_speaking(shared: State<'_, SharedApp>) -> Result<RoundState, String> {
    let mut app = shared.app.lock().await;
    let _ = app.start_speaking();
    Ok(app.state().clone())
}

/// 结束口播：口播 → 就绪。
#[tauri::command]
async fn end_speaking(shared: State<'_, SharedApp>) -> Result<RoundState, String> {
    let mut app = shared.app.lock().await;
    let _ = app.end_speaking();
    Ok(app.state().clone())
}

/// 启动应用：加载 `.env` → 装配宿主 → 注册命令。
pub fn run() {
    termkata_host::env::load_dotenv();

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
