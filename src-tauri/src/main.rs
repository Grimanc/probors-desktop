#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(unused_imports)]
use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind, MessageDialogButtons};
use std::time::Duration;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // Check for updates in the background after app loads
            tauri::async_runtime::spawn(async move {
                // Wait for the app to fully load
                tokio::time::sleep(Duration::from_secs(3)).await;
                
                if let Err(e) = check_for_updates(app_handle).await {
                    eprintln!("Update check failed: {}", e);
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn check_for_updates(app: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let updater = app.updater()?;
    
    if let Some(update) = updater.check().await? {
        let current_version = update.current_version.to_string();
        let new_version = update.version.clone();
        let notes = update.body.clone().unwrap_or_default();
        
        let message = if notes.is_empty() {
            format!(
                "A new version of ProBors is available!\n\nCurrent version: v{}\nNew version: v{}\n\nWould you like to update now?",
                current_version, new_version
            )
        } else {
            format!(
                "A new version of ProBors is available!\n\nCurrent version: v{}\nNew version: v{}\n\nWhat's new:\n{}\n\nWould you like to update now?",
                current_version, new_version, notes
            )
        };
        
        // Show update confirmation dialog
        let should_update = app
            .dialog()
            .message(message)
            .title("Update Available")
            .kind(MessageDialogKind::Info)
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Update Now".to_string(),
                "Later".to_string(),
            ))
            .blocking_show();
        
        if should_update {
            // Show downloading message
            let _ = app
                .dialog()
                .message("Downloading update... Please wait.\n\nThe app will restart automatically when complete.")
                .title("Updating ProBors")
                .kind(MessageDialogKind::Info)
                .buttons(MessageDialogButtons::Ok)
                .blocking_show();
            
            // Download and install the update
            update.download_and_install(
                |_chunk_len, _content_len| {
                    // Progress callback - could emit events here
                },
                || {
                    // Download complete, installing
                },
            ).await?;
            
            // Restart the app to apply update
            app.restart();
        }
    }
    
    Ok(())
}
