//! frontend/ui/mod.rs
//!
//! Provides a web-view UI.

use anyhow::Result;
use wry::{
    application::{
        dpi::LogicalSize,
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::{RpcResponse, WebViewBuilder},
};

use log::Level;

#[derive(Deserialize, Debug)]
enum CallbackType {
    SelectInstallDir { callback_name: String },
    Log { msg: String, kind: String },
    Test {},
}

/// Starts the main web UI. Will return when UI is closed.
pub fn start_ui(app_name: &str, http_address: &str, is_launcher: bool) -> Result<()> {
    let size = if is_launcher { (600, 300) } else { (1024, 600) };
    info!("Spawning web view instance");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(format!("{} Installer", app_name))
        .with_inner_size(LogicalSize::new(size.0, size.1))
        .with_resizable(false)
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
        .with_url(http_address)?
        .with_rpc_handler(|_, mut event| {
            debug!("Incoming payload: {:?}", event);
            match event.method.as_str() {
                "Test" => (),
                "Log" => {
                    if let Some(msg) = event.params.take() {
                        if let Ok(msg) = serde_json::from_value::<(String, String)>(msg) {
                            let kind = match msg.0.as_str() {
                                "info" | "log" => Level::Info,
                                "warn" => Level::Warn,
                                _ => Level::Error,
                            };
                            log!(target: "liftinstall::frontend::js", kind, "{}", msg.1);
                        }
                    }
                }
                "SelectInstallDir" => {
                    let result =
                        tinyfiledialogs::select_folder_dialog("Select a install directory...", "")
                            .and_then(|v| serde_json::to_value(v).ok());
                    return Some(RpcResponse::new_result(event.id, result));
                }
                _ => warn!("Unknown RPC method: {}", event.method),
            }
            None
        })
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => info!("Webview started"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
