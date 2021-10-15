//! frontend/ui/mod.rs
//!
//! Provides a web-view UI.

use web_view::Content;

use crate::logging::LoggingErrors;

use log::Level;

#[derive(Deserialize, Debug)]
enum CallbackType {
    SelectInstallDir { callback_name: String },
    Log { msg: String, kind: String },
    Test {},
}

/// Starts the main web UI. Will return when UI is closed.
pub fn start_ui(app_name: &str, http_address: &str, is_launcher: bool) {
    let size = if is_launcher { (600, 300) } else { (1024, 600) };

    info!("Spawning web view instance");

    web_view::builder()
        .title(&format!("{} Installer", app_name))
        .content(Content::Url(http_address))
        .size(size.0, size.1)
        .resizable(false)
        .debug(cfg!(debug_assertions))
        .user_data(())
        .invoke_handler(|wv, msg| {
            let mut cb_result = Ok(());
            let command: CallbackType =
                serde_json::from_str(msg).log_expect(&format!("Unable to parse string: {:?}", msg));

            debug!("Incoming payload: {:?}", command);

            match command {
                CallbackType::SelectInstallDir { callback_name } => {
                    let result =
                        tinyfiledialogs::select_folder_dialog("Select a install directory...", "");

                    if let Some(new_path) = result {
                        if !new_path.is_empty() {
                            let result = serde_json::to_string(&new_path)
                                .log_expect("Unable to serialize response");
                            let command = format!("window.{}({});", callback_name, result);
                            debug!("Injecting response: {}", command);
                            cb_result = wv.eval(&command);
                        }
                    }
                }
                CallbackType::Log { msg, kind } => {
                    let kind = match kind.as_ref() {
                        "info" | "log" => Level::Info,
                        "warn" => Level::Warn,
                        "error" => Level::Error,
                        _ => Level::Error,
                    };

                    log!(target: "liftinstall::frontend::js", kind, "{}", msg);
                }
                CallbackType::Test {} => {}
            }

            cb_result
        })
        .run()
        .log_expect("Unable to launch Web UI!");
}
