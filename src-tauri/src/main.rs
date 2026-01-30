#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use std::process::{Child, Command};
use std::sync::Mutex;

static NEXT_CHILD: Lazy<Mutex<Option<Child>>> = Lazy::new(|| Mutex::new(None));

fn spawn_next_server() {
    #[cfg(target_os = "windows")]
    let npm_cmd = "npm.cmd";
    #[cfg(not(target_os = "windows"))]
    let npm_cmd = "npm";

    // Start Next.js production server from the sibling repo
    let child = Command::new(npm_cmd)
        .args(&["run", "start"])
        .current_dir("../probors-front/probors-react-2.0")
        .spawn();

    if let Ok(c) = child {
        *NEXT_CHILD.lock().unwrap() = Some(c);
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|_app| {
            // Spawn local Next server only when explicitly requested via env var.
            // Production desktop builds will load the remote site (https://dashboard.probors.com)
            // and should not attempt to start a local server.
            let spawn_local = std::env::var("PROBORS_LOCAL_NEXT").unwrap_or_default();
            if std::env::var("TAURI_DEV").is_ok() || spawn_local == "true" {
                // In dev mode or when PROBORS_LOCAL_NEXT=true, spawn the Next server.
                spawn_next_server();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

