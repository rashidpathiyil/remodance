use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time;
use user_idle::UserIdle;
use chrono::{DateTime, Utc};
use serde_json;

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

// Start the idle monitoring thread
fn start_idle_monitor(app_handle: AppHandle) {
    let app_handle_clone = app_handle.clone();
    
    // Spawn a background task to monitor idle time
    tauri::async_runtime::spawn(async move {
        // Get state inside the async block, using the cloned handle
        let state: State<Arc<AppState>> = app_handle_clone.state();
        let mut interval = time::interval(Duration::from_secs(1));
        
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
                Err(_) => continue,
            };
            
            // Get current status
            let current_status = {
                state.status.lock().unwrap().clone()
            };
            
            // Convert idle timeout to milliseconds
            let idle_timeout = Duration::from_secs(settings.idle_timeout_mins * 60);
            
            // Check if the user is idle
            if idle_duration > idle_timeout && current_status == AttendanceStatus::CheckedIn {
                // User is idle beyond threshold, check out
                if let Err(e) = process_attendance_change(&app_handle_clone, "check-out", &settings).await {
                    println!("Error processing check-out: {}", e);
                }
                
                // Update status in state
                {
                    let mut status = state.status.lock().unwrap();
                    *status = AttendanceStatus::CheckedOut;
                }
            } else if idle_duration < Duration::from_secs(1) && current_status == AttendanceStatus::CheckedOut {
                // User is active, check in
                if let Err(e) = process_attendance_change(&app_handle_clone, "check-in", &settings).await {
                    println!("Error processing check-in: {}", e);
                }
                
                // Update status in state
                {
                    let mut status = state.status.lock().unwrap();
                    *status = AttendanceStatus::CheckedIn;
                }
                
                // Update last activity time
                {
                    let mut last_activity = state.last_activity.lock().unwrap();
                    *last_activity = Instant::now();
                }
            }
        }
    });
}

// Process attendance change (check-in or check-out)
async fn process_attendance_change(app_handle: &AppHandle, event_type: &str, settings: &Settings) -> Result<(), String> {
    // Emit event to frontend
    let _ = app_handle.emit("attendance_changed", event_type);
    
    // Get current time
    let now: DateTime<Utc> = Utc::now();
    
    // Prepare payload
    let payload = AttendancePayload {
        event_type: event_type.to_string(),
        user_id: settings.username.clone(),
        payload: AttendanceData {
            time: now.format("%H:%M:%S").to_string(),
            date: now.format("%Y-%m-%d").to_string(),
            device_id: settings.device_name.clone(),
            config: if settings.developer_mode {
                Some(ConfigData {
                    idle_timeout_mins: settings.idle_timeout_mins,
                    auto_mode: settings.auto_mode,
                })
            } else {
                None
            },
        },
        timestamp: now.to_rfc3339(),
    };
    
    // Send to API (in a real implementation, handle errors and offline mode)
    if let Err(e) = send_to_api(&settings.api_endpoint, &payload).await {
        println!("Error sending to API: {}", e);
        // In a real implementation, store for later sending
    }
    
    Ok(())
}

// Send payload to API
async fn send_to_api(endpoint: &str, payload: &AttendancePayload) -> Result<(), String> {
    // In a real implementation, use HTTP client to send request
    // For now, just log the attempt to the console for testing
    println!("Would send to API: {} with event: {}", endpoint, payload.event_type);
    println!("Payload: {}", serde_json::to_string(payload).unwrap_or_default());
    
    // Mock successful response
    Ok(())
}

/// Check-in or check-out manually
#[tauri::command]
async fn send_attendance_event(event_type: String, app_handle: AppHandle) -> Result<(), String> {
    let state: State<Arc<AppState>> = app_handle.state();
    
    // Get settings
    let settings = {
        state.settings.lock().unwrap().clone()
    };
    
    // Process the attendance change
    process_attendance_change(&app_handle, &event_type, &settings).await?;
    
    // Update state
    {
        let mut status = state.status.lock().unwrap();
        *status = if event_type == "check-in" {
            AttendanceStatus::CheckedIn
        } else {
            AttendanceStatus::CheckedOut
        };
    }
    
    Ok(())
}

/// Get the current attendance status
#[tauri::command]
fn get_attendance_status(state: State<Arc<AppState>>) -> String {
    let status = state.status.lock().unwrap();
    match *status {
        AttendanceStatus::CheckedIn => "checked-in".to_string(),
        AttendanceStatus::CheckedOut => "checked-out".to_string(),
    }
}

/// Get application configuration
#[tauri::command]
fn get_app_config(state: State<Arc<AppState>>) -> Settings {
    state.settings.lock().unwrap().clone()
}

/// Get application version
#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Open settings window
#[tauri::command]
fn open_settings(_app_handle: AppHandle) -> Result<(), String> {
    Ok(())
}

/// Save application settings
#[tauri::command]
fn save_settings(settings: Settings, state: State<Arc<AppState>>) -> Result<(), String> {
    // Update settings in state
    {
        let mut current_settings = state.settings.lock().unwrap();
        *current_settings = settings;
    }
    
    // In a real implementation, save settings to disk
    
    Ok(())
}

// Application entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Create app state
    let app_state = Arc::new(AppState::default());
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Start idle monitor
            let app_handle = app.handle().clone(); // Clone to get owned AppHandle
            start_idle_monitor(app_handle);
            
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 
