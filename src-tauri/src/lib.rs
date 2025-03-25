use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time;
use user_idle::UserIdle;
use chrono::Local;
use serde_json;
use log::{info, error, debug};
use reqwest;

// Attendance status
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum AttendanceStatus {
    CheckedIn,
    CheckedOut,
}

impl Default for AttendanceStatus {
    fn default() -> Self {
        Self::CheckedOut
    }
}

// Store application state
#[derive(Debug)]
struct AppState {
    status: Mutex<AttendanceStatus>,
    last_activity: Mutex<Instant>,
    settings: Mutex<Settings>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            status: Mutex::new(AttendanceStatus::default()),
            last_activity: Mutex::new(Instant::now()),
            settings: Mutex::new(Settings::default()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Settings {
    api_endpoint: String,
    username: String,
    device_name: String,
    idle_timeout_mins: u64,
    auto_mode: bool,
    developer_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_endpoint: "https://example.com/attendance".to_string(),
            username: whoami::username(),
            device_name: whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string()),
            idle_timeout_mins: 10,
            auto_mode: true,
            developer_mode: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AttendancePayload {
    event_type: String,
    user_id: String,
    payload: AttendanceData,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AttendanceData {
    time: String,
    date: String,
    device_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<ConfigData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigData {
    idle_timeout_mins: u64,
    auto_mode: bool,
}

// Helper to create the current ISO timestamp
fn iso_timestamp() -> String {
    use chrono::Utc;
    Utc::now().to_rfc3339()
}

// Format current time as HH:MM:SS
fn format_current_time() -> String {
    use chrono::Local;
    Local::now().format("%H:%M:%S").to_string()
}

// Format current date as YYYY-MM-DD
fn format_current_date() -> String {
    use chrono::Local;
    Local::now().format("%Y-%m-%d").to_string()
}

// Create attendance payload from settings
fn create_attendance_payload(event_type: &str, settings: &Settings) -> AttendancePayload {
    let config = if settings.developer_mode {
        Some(ConfigData {
            idle_timeout_mins: settings.idle_timeout_mins,
            auto_mode: settings.auto_mode,
        })
    } else {
        None
    };

    AttendancePayload {
        event_type: event_type.to_string(),
        user_id: settings.username.clone(),
        payload: AttendanceData {
            time: format_current_time(),
            date: format_current_date(),
            device_id: settings.device_name.clone(),
            config,
        },
        timestamp: iso_timestamp(),
    }
}

// Start the idle monitoring thread
fn start_idle_monitor(app_handle: AppHandle) {
    let app_handle_clone = app_handle.clone();
    
    // Spawn a background task to monitor idle time
    tauri::async_runtime::spawn(async move {
        // Get state inside the async block, using the cloned handle
        let state: State<'_, Arc<AppState>> = app_handle_clone.state();
        let mut interval = time::interval(Duration::from_secs(1));
        
        debug!("Idle monitor thread started");
        
        loop {
            interval.tick().await;
            
            // Get the current settings
            let settings = {
                state.settings.lock().unwrap().clone()
            };
            
            // Skip if auto-mode is disabled
            if !settings.auto_mode {
                continue;
            }
            
            // Get the idle time using the correct API
            let idle_duration = match UserIdle::get_time() {
                Ok(idle_info) => idle_info.duration(),
                Err(e) => {
                    error!("Failed to get idle time: {}", e);
                    continue;
                }
            };
            
            // Get current status
            let current_status = {
                state.status.lock().unwrap().clone()
            };
            
            // Convert idle timeout to milliseconds
            let idle_timeout = Duration::from_secs(settings.idle_timeout_mins * 60);
            
            // Check if the user is idle
            if idle_duration >= idle_timeout {
                if current_status == AttendanceStatus::CheckedIn {
                    info!("User is idle for {} seconds. Automatically checking out", idle_duration.as_secs());
                    
                    // Update status in state
                    {
                        let mut status = state.status.lock().unwrap();
                        *status = AttendanceStatus::CheckedOut;
                    }
                    
                    // Create payload and send check-out event to the API
                    let payload = create_attendance_payload("check-out", &settings);
                    if let Err(err) = send_to_api("check-out", &payload, &settings).await {
                        error!("Failed to send check-out event: {}", err);
                    }
                    
                    // Notify the frontend
                    let _ = app_handle_clone.emit("attendance_changed", "check-out");
                }
            } else {
                // User is active
                if current_status == AttendanceStatus::CheckedOut {
                    info!("User activity detected after being idle. Automatically checking in");
                    
                    // Update status in state
                    {
                        let mut status = state.status.lock().unwrap();
                        *status = AttendanceStatus::CheckedIn;
                    }
                    
                    // Create payload and send check-in event to the API
                    let payload = create_attendance_payload("check-in", &settings);
                    if let Err(err) = send_to_api("check-in", &payload, &settings).await {
                        error!("Failed to send check-in event: {}", err);
                    }
                    
                    // Notify the frontend
                    let _ = app_handle_clone.emit("attendance_changed", "check-in");
                }
                
                // Update last activity time
                {
                    let mut last_activity = state.last_activity.lock().unwrap();
                    *last_activity = Instant::now();
                    
                    // Emit activity update event every 60 seconds
                    let elapsed = last_activity.elapsed();
                    if elapsed.as_secs() > 60 {
                        debug!("Emitting activity update");
                        let _ = app_handle_clone.emit("activity_update", "");
                    }
                }
            }
        }
    });
}

// Send attendance event to API
async fn send_to_api(event_type: &str, payload: &AttendancePayload, settings: &Settings) -> Result<(), String> {
    // Serialize the payload to JSON
    let payload_str = match serde_json::to_string(payload) {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to serialize payload: {}", e))
    };
    
    info!("Sending {} event to API: {}", event_type, payload_str);
    
    // Get API endpoint from settings
    let api_endpoint = &settings.api_endpoint;
    
    // Send the actual HTTP request
    let client = reqwest::Client::new();
    let response = client.post(api_endpoint)
        .header("Content-Type", "application/json")
        .body(payload_str)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;
    
    // Check if the request was successful
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await
            .unwrap_or_else(|_| "Failed to get error details".to_string());
        
        error!("API request failed with status {}: {}", status, error_text);
        return Err(format!("API request failed with status {}", status));
    }
    
    info!("Successfully sent {} event to API", event_type);
    Ok(())
}

// Send attendance event
#[tauri::command]
async fn send_attendance_event(event_type: String, app_handle: AppHandle, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    // Get settings
    let settings = {
        state.settings.lock().unwrap().clone()
    };
    
    // Update status in state
    {
        let mut status = state.status.lock().unwrap();
        *status = if event_type == "check-in" {
            AttendanceStatus::CheckedIn
        } else {
            AttendanceStatus::CheckedOut
        };
    }
    
    // Create payload and send to API
    let payload = create_attendance_payload(&event_type, &settings);
    send_to_api(&event_type, &payload, &settings).await?;
    
    // Notify the frontend
    let _ = app_handle.emit("attendance_changed", &event_type);
    
    Ok(())
}

// Get current attendance status
#[tauri::command]
fn get_attendance_status(state: State<'_, Arc<AppState>>) -> String {
    let status = state.status.lock().unwrap();
    match *status {
        AttendanceStatus::CheckedIn => "checked-in".to_string(),
        AttendanceStatus::CheckedOut => "checked-out".to_string(),
    }
}

// Get app configuration
#[tauri::command]
fn get_app_config(state: State<'_, Arc<AppState>>) -> Settings {
    state.settings.lock().unwrap().clone()
}

// Get app version
#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// Open settings window
#[tauri::command]
fn open_settings() -> Result<(), String> {
    Ok(())
}

// Save settings
#[tauri::command]
fn save_settings(settings: Settings, state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let mut settings_lock = state.settings.lock().unwrap();
    *settings_lock = settings;
    
    Ok(())
}

// Configure auto launch
fn configure_auto_launch(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri_plugin_autostart::ManagerExt;
    
    let autostart_manager = app.autolaunch();
    
    // Enable auto-launch by default
    if let Ok(false) = autostart_manager.is_enabled() {
        info!("Enabling auto-launch at startup");
        let _ = autostart_manager.enable();
    }
    
    Ok(())
}

// Check if auto-launch is enabled
#[tauri::command]
fn is_auto_launch_enabled(app_handle: AppHandle) -> Result<bool, String> {
    use tauri_plugin_autostart::ManagerExt;
    
    let autostart_manager = app_handle.autolaunch();
    
    autostart_manager.is_enabled()
        .map_err(|err| format!("Failed to check auto-launch status: {}", err))
}

// Toggle auto-launch
#[tauri::command]
fn toggle_auto_launch(app_handle: AppHandle, enable: bool) -> Result<(), String> {
    use tauri_plugin_autostart::ManagerExt;
    
    let autostart_manager = app_handle.autolaunch();
    
    if enable {
        autostart_manager.enable()
            .map_err(|err| format!("Failed to enable auto-launch: {}", err))
    } else {
        autostart_manager.disable()
            .map_err(|err| format!("Failed to disable auto-launch: {}", err))
    }
}

// Application entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Create app state
    let app_state = Arc::new(AppState::default());
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None // No extra args
        ))
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(|app| {
            info!("Starting Remodance v{}", env!("CARGO_PKG_VERSION"));
            
            // Start idle monitor
            let app_handle = app.handle().clone(); // Clone to get owned AppHandle
            start_idle_monitor(app_handle);
            
            // Configure auto-launch
            if let Err(err) = configure_auto_launch(app) {
                error!("Failed to configure auto-launch: {}", err);
            }
            
            Ok(())
        })
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            send_attendance_event,
            get_attendance_status,
            get_app_config,
            get_app_version,
            open_settings,
            save_settings,
            is_auto_launch_enabled,
            toggle_auto_launch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert_eq!(*state.status.lock().unwrap(), AttendanceStatus::CheckedOut);
    }

    #[test]
    fn test_create_attendance_payload() {
        let settings = Settings {
            api_endpoint: "https://example.com/api".to_string(),
            username: "testuser".to_string(),
            device_name: "testdevice".to_string(),
            idle_timeout_mins: 10,
            auto_mode: true,
            developer_mode: false,
        };

        let payload = create_attendance_payload("check-in", &settings);
        
        assert_eq!(payload.user_id, "testuser");
        assert_eq!(payload.payload.device_id, "testdevice");
        
        // Validate time format (HH:MM:SS)
        let time_parts: Vec<&str> = payload.payload.time.split(':').collect();
        assert_eq!(time_parts.len(), 3);
        
        // Validate date format (YYYY-MM-DD)
        let date_parts: Vec<&str> = payload.payload.date.split('-').collect();
        assert_eq!(date_parts.len(), 3);
    }

    #[test]
    fn test_format_current_time() {
        let now = Local::now();
        let formatted = format_current_time();
        assert_eq!(formatted, now.format("%H:%M:%S").to_string());
    }

    #[test]
    fn test_format_current_date() {
        let now = Local::now();
        let formatted = format_current_date();
        assert_eq!(formatted, now.format("%Y-%m-%d").to_string());
    }
} 
