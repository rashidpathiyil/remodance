# Remodance Development Notes

## Issues and Fixes for Tauri 2 Implementation

This document outlines the issues encountered during the implementation of the Remodance virtual attendance application using Tauri 2, along with the solutions applied.

## Key Issues

### 1. Capability File Configuration

**Problem**: Tauri 2 has completely changed how permissions are configured in capability files.

**Error**:
```
Permission path:default not found, expected one of core:default, core:app:default...
```

**Solution**: 
- Permissions in Tauri 2 must use the `core:` prefix (e.g., `core:path:default` instead of `path:default`)
- A minimal working configuration:

```json
{
  "$schema": "https://raw.githubusercontent.com/tauri-apps/tauri/dev/core/tauri-config-schema/schema.json",
  "identifier": "main-capability",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:path:default",
    "core:event:default", 
    "core:window:default",
    "core:app:default",
    "core:tray:default"
  ]
}
```

### 2. SystemTray API Changes

**Problem**: The system tray APIs have changed in Tauri 2, causing compilation errors.

**Error**:
```
error[E0599]: no method named `system_tray` found for struct `tauri::Builder` in the current scope
```

**Solution**:
- Remove trayIcon from tauri.conf.json completely and implement it programmatically
- Use the TrayIconBuilder API from Tauri 2.0

```rust
// Create tray icon
let tray_icon = TrayIconBuilder::new()
    .tooltip("Remodance")
    .build(app)?;
```

### 3. Emitter Trait Not in Scope

**Problem**: The `emit` method on `AppHandle` was not found because the trait wasn't in scope.

**Error**:
```
error[E0599]: no method named `emit` found for struct `AppHandle` in the current scope
```

**Solution**:
- Import the `Emitter` trait explicitly:
```rust
use tauri::{AppHandle, Emitter};
```

### 4. User-Idle API Usage

**Problem**: The `UserIdle` crate's API was being used incorrectly.

**Error**:
```
error[E0599]: no function or associated item named `get_idle_time` found for struct `UserIdle`
```

**Initial Solution**:
- Use the correct API method:
```rust
let idle_duration = match UserIdle::get_time() {
    Ok(idle) => idle.idle, // Note: need to access the `idle` field
    Err(_) => continue,
};
```

**Updated Solution**:
- The correct usage for getting duration is to use the `duration()` method, not accessing a field directly:
```rust
let idle_duration = match UserIdle::get_time() {
    Ok(idle_info) => idle_info.duration(), // Use the method, not direct field access
    Err(_) => continue,
};
```

### 5. Build Script Errors

**Problem**: The build script wasn't properly setting environment variables.

**Error**:
```
OUT_DIR env var is not set, do you have a build script?
```

**Solution**:
- Update the build.rs file to include:
```rust
fn main() {
    println!("cargo:rerun-if-changed=tauri.conf.json");
    println!("cargo:rerun-if-changed=capabilities");
    tauri_build::build()
}
```

### 6. Configuration File Structure

**Problem**: Configuration file structure in Tauri 2 differs from Tauri 1.x.

**Solution**:
- Updated tauri.conf.json to use the correct format:
  - Changed `systemTray` to `trayIcon`
  - Removed `withGlobalShortcut` and used `withGlobalTauri` instead
  - Removed `autoStart` configuration (handle programmatically instead)

### 7. Deprecated Functions

**Warning**:
```
warning: use of deprecated function `whoami::hostname`: use `fallible::hostname()` instead
```

**Solution**:
- Update to use the recommended APIs:
```rust
device_name: whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string()),
```

### 8. Rust Lifetime Issues with AppHandle in Async Tasks

**Problem**: When using `app_handle.state()` outside the async block and trying to use that state reference in the async task, Rust reports a lifetime error.

**Error**:
```
error[E0597]: `app_handle` does not live long enough
```

**Solution**:
- Move the state acquisition inside the async block:
```rust
fn start_idle_monitor(app_handle: AppHandle) {
    let app_handle_clone = app_handle.clone();
    
    tauri::async_runtime::spawn(async move {
        // Get state inside the async block using the cloned handle
        let state: State<Arc<AppState>> = app_handle_clone.state();
        
        // Rest of the code...
    });
}
```

### 9. TrayIcon Configuration in Tauri 2.0

**Problem**: The `trayIcon` configuration in tauri.conf.json causes schema validation errors.

**Error**:
```
Error `tauri.conf.json` error on `app > trayIcon`: {"icon":"icons/icon.png",...} is not valid under any of the schemas listed in the 'anyOf' keyword
```

**Solution**:
- Remove the `trayIcon` from tauri.conf.json completely
- Add the `tray-icon` feature to the Tauri dependency in Cargo.toml:
```toml
tauri = { version = "2", features = ["tray-icon"] }
```
- Implement tray icon programmatically in the Rust code using TrayIconBuilder

### 10. Application State Management

**Problem**: Safely sharing state between different parts of the application, including async tasks.

**Solution**:
- Use Arc<Mutex<T>> pattern for threadsafe state:
```rust
struct AppState {
    status: Mutex<AttendanceStatus>,
    last_activity: Mutex<Instant>,
    settings: Mutex<Settings>,
}
```
- Register state with Tauri:
```rust
.manage(app_state)
```
- Access state in commands:
```rust
fn get_attendance_status(state: State<Arc<AppState>>) -> String {
    let status = state.status.lock().unwrap();
    // ...
}
```

## Automatic Check-In/Check-Out Implementation

The automatic check-in/check-out functionality has been implemented using the following approach:

1. **Idle Time Detection**:
   - Created a background task that runs every second to check user activity
   - Used the `user-idle` crate with proper API usage to detect system inactivity
   - Configured the idle timeout based on user settings

2. **State Management**:
   - Created a thread-safe application state with mutex-protected fields
   - Stored the current attendance status, last activity time, and user settings
   - Used Arc for safe sharing between threads

3. **Event Processing**:
   - Automatically checks out the user when idle time exceeds the threshold
   - Automatically checks in when activity is detected after being checked out
   - Emits events to the frontend to update the UI

4. **API Integration**:
   - Formats API requests according to the required JSON structure
   - Currently uses a mock implementation for testing
   - Will be replaced with actual HTTP requests in the future

## Next Steps for Development

1. **Re-implement system tray functionality** using Tauri 2's new API ✅
2. **Add idle monitoring** with correct `user-idle` usage ✅
3. **Implement auto-launch** using Tauri 2's capabilities
4. **Add offline functionality** (storing events for later sending)
5. **Enhance error handling** ✅
6. **Fix all deprecated function warnings** ✅

## Lessons Learned

1. **Rust Ownership and Lifetimes** - Be especially careful with async tasks that outlive the function they're created in. Use cloning and move semantics to ensure proper ownership.

2. **Tauri 2.0 API Changes** - Many APIs have changed in Tauri 2.0. Consult the documentation before implementing features.

3. **Schema Validation** - Tauri 2.0 is more strict about configuration files. Remove configurations entirely if unsure about the correct format, and implement the feature programmatically.

4. **Error Messages** - Rust's error messages, while sometimes verbose, provide valuable information. Pay attention to suggestions like "consider using method X instead."

## Useful Resources

- [Tauri 2 Migration Guide](https://tauri.app/v2/migration/from-v1/)
- [Tauri 2 Capabilities Documentation](https://tauri.app/v2/dev/capabilities/)
- [user-idle crate documentation](https://docs.rs/user-idle)
- [Tauri 2.0 System Tray Documentation](https://v2.tauri.app/learn/system-tray/)

## Development Environment

- MacOS
- Tauri 2
- Cargo/Rust
- Vue.js frontend 
