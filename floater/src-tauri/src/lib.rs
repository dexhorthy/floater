use std::sync::{Arc, Mutex};
use std::path::Path;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Emitter};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use anyhow::Result;

#[derive(Debug, Deserialize)]
#[serde(tag = "action")]
enum Command {
    #[serde(rename = "show")]
    Show {
        content: String,
        #[serde(default)]
        timer: bool,
    },
    #[serde(rename = "hide")]
    Hide,
}

#[derive(Debug, Serialize)]
struct Response {
    status: String,
    message: String,
}

// Shared state for the application content
type AppState = Arc<Mutex<String>>;

#[tauri::command]
fn get_content(state: tauri::State<AppState>) -> String {
    let content = state.lock().unwrap();
    content.clone()
}

#[tauri::command]
fn set_content(content: String, state: tauri::State<AppState>, app: AppHandle) -> Result<(), String> {
    {
        let mut app_content = state.lock().unwrap();
        *app_content = content;
    }

    // Emit event to frontend to update content
    app.emit("content-updated", &*state.lock().unwrap())
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn handle_client(mut stream: UnixStream, app_handle: AppHandle) -> Result<()> {
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    while buf_reader.read_line(&mut line).await? > 0 {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            line.clear();
            continue;
        }

        let response = match serde_json::from_str::<Command>(trimmed) {
            Ok(command) => {
                match command {
                    Command::Show { content, timer } => {
                        match handle_show_command(&app_handle, content, timer).await {
                            Ok(_) => Response {
                                status: "success".to_string(),
                                message: "Window shown with content".to_string(),
                            },
                            Err(e) => Response {
                                status: "error".to_string(),
                                message: format!("Failed to show window: {}", e),
                            },
                        }
                    }
                    Command::Hide => {
                        match handle_hide_command(&app_handle).await {
                            Ok(_) => Response {
                                status: "success".to_string(),
                                message: "Window hidden".to_string(),
                            },
                            Err(e) => Response {
                                status: "error".to_string(),
                                message: format!("Failed to hide window: {}", e),
                            },
                        }
                    }
                }
            }
            Err(e) => Response {
                status: "error".to_string(),
                message: format!("Invalid JSON command: {}", e),
            },
        };

        let response_json = serde_json::to_string(&response)?;
        writer.write_all(format!("{}\n", response_json).as_bytes()).await?;
        writer.flush().await?;

        line.clear();
    }

    Ok(())
}

async fn handle_show_command(app_handle: &AppHandle, content: String, timer: bool) -> Result<()> {
    let window = app_handle.get_webview_window("main")
        .ok_or_else(|| anyhow::anyhow!("Main window not found"))?;

    // Update content state and emit event
    let state: tauri::State<AppState> = app_handle.state();
    {
        let mut app_content = state.lock().unwrap();
        *app_content = content;
    }

    let payload = serde_json::json!({
        "content": &*state.lock().unwrap(),
        "timer": timer
    });
    app_handle.emit("content-updated", payload)?;

    // Show the window
    window.show()?;
    window.set_focus()?;

    Ok(())
}

async fn handle_hide_command(app_handle: &AppHandle) -> Result<()> {
    let window = app_handle.get_webview_window("main")
        .ok_or_else(|| anyhow::anyhow!("Main window not found"))?;

    window.hide()?;

    Ok(())
}

async fn start_socket_server(app_handle: AppHandle) -> Result<()> {
    let socket_path = "/tmp/floater.sock";

    // Remove existing socket file if it exists
    if Path::new(socket_path).exists() {
        std::fs::remove_file(socket_path)?;
    }

    let listener = UnixListener::bind(socket_path)?;
    println!("Unix domain socket server listening on {}", socket_path);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let app_handle_clone = app_handle.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream, app_handle_clone).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::default();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![get_content, set_content])
        .setup(|app| {
            let app_handle = app.app_handle().clone();

            // Start the socket server in a background task
            tauri::async_runtime::spawn(async move {
                if let Err(e) = start_socket_server(app_handle).await {
                    eprintln!("Socket server error: {}", e);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
