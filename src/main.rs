//! main.rs
//!
//! The main entrypoint for the application. Orchestrates the building of the installation
//! framework, and opens necessary HTTP servers/frontends.

//#![windows_subsystem = "windows"]

#[cfg(windows)]
extern crate nfd;

extern crate web_view;

extern crate futures;
extern crate hyper;
extern crate url;

extern crate number_prefix;
extern crate reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;

extern crate regex;
extern crate semver;

extern crate zip;

mod assets;
mod config;
mod http;
mod installer;
mod rest;
mod sources;
mod tasks;

use web_view::*;

use config::Config;

use installer::InstallerFramework;

#[cfg(windows)]
use nfd::Response;

use rest::WebServer;

// TODO: Fetch this over a HTTP request?
static RAW_CONFIG: &'static str = include_str!("../config.toml");

#[derive(Deserialize, Debug)]
enum CallbackType {
    SelectInstallDir { callback_name: String },
}

fn main() {
    let config = Config::from_toml_str(RAW_CONFIG).unwrap();

    let app_name = config.general.name.clone();

    println!("{} installer", app_name);

    let current_exe = std::env::current_exe().unwrap();
    let current_path = current_exe.parent().unwrap();
    let metadata_file = current_path.join("metadata.json");
    let framework = if metadata_file.exists() {
        println!("Using pre-existing metadata file: {:?}", metadata_file);
        InstallerFramework::new_with_db(config, current_path).unwrap()
    } else {
        println!("Starting fresh install");
        InstallerFramework::new(config)
    };

    let server = WebServer::new(framework).unwrap();

    // Startup HTTP server for handling the web view
    let http_address = format!("http://{}", server.get_addr());
    println!("Server: {:?}", http_address);

    // Init the web view
    let size = (1024, 500);
    let resizable = false;
    let debug = true;

    run(
        &format!("{} Installer", app_name),
        Content::Url(http_address),
        Some(size),
        resizable,
        debug,
        |_| {},
        |wv, msg, _| {
            let command: CallbackType =
                serde_json::from_str(msg).expect(&format!("Unable to parse string: {:?}", msg));

            println!("Incoming payload: {:?}", command);

            match command {
                CallbackType::SelectInstallDir { callback_name } => {
                    #[cfg(windows)]
                    let result =
                        match nfd::open_pick_folder(None).expect("Unable to open folder dialog") {
                            Response::Okay(v) => v,
                            _ => return,
                        };

                    #[cfg(not(windows))]
                    let result =
                        wv.dialog(Dialog::ChooseDirectory, "Select a install directory...", "");

                    if result.len() > 0 {
                        let result =
                            serde_json::to_string(&result).expect("Unable to serialize response");
                        let command = format!("{}({});", callback_name, result);
                        println!("Injecting response: {}", command);
                        wv.eval(&command);
                    }
                }
            }
        },
        (),
    );
}
