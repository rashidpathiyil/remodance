<script setup lang="ts">
import { ref, onMounted, reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/api/shell";

// State variables
const isCheckedIn = ref(false);
const isAutoMode = ref(true);
const lastActivityTime = ref(new Date());
const appVersion = ref("");
const showSettings = ref(false);

// Settings form
const settings = reactive({
  apiEndpoint: "",
  username: "",
  deviceName: "",
  idleTimeoutMins: 10,
  autoMode: true,
  developerMode: false
});

// Toggle check-in/check-out status manually
async function toggleAttendance() {
  const newStatus = !isCheckedIn.value;
  isCheckedIn.value = newStatus;
  
  // Send API request with check-in/check-out event
  await sendAttendanceEvent(newStatus ? "check-in" : "check-out");
}

// Send attendance event to API
async function sendAttendanceEvent(eventType: "check-in" | "check-out") {
  try {
    await invoke("send_attendance_event", { eventType });
  } catch (error) {
    console.error("Failed to send attendance event:", error);
  }
}

// Initialize app
async function initApp() {
  try {
    // Get app version
    appVersion.value = await invoke("get_app_version");
    
    // Listen for auto check-in/check-out events from Rust
    await listen("attendance_changed", (event) => {
      isCheckedIn.value = event.payload === "check-in";
    });
    
    // Listen for activity updates
    await listen("activity_update", (event) => {
      lastActivityTime.value = new Date();
    });
    
    // Initialize app state
    const config = await invoke("get_app_config");
    isAutoMode.value = config.autoMode;
    
    // Initialize settings
    settings.apiEndpoint = config.api_endpoint;
    settings.username = config.username;
    settings.deviceName = config.device_name;
    settings.idleTimeoutMins = config.idle_timeout_mins;
    settings.autoMode = config.auto_mode;
    settings.developerMode = config.developer_mode;
    
    // Check initial status
    const status = await invoke("get_attendance_status");
    isCheckedIn.value = status === "checked-in";
  } catch (error) {
    console.error("Failed to initialize app:", error);
  }
}

// Open settings window
function openSettings() {
  showSettings.value = true;
}

// Close settings window
function closeSettings() {
  showSettings.value = false;
}

// Save settings
async function saveSettings() {
  try {
    await invoke("save_settings", {
      settings: {
        api_endpoint: settings.apiEndpoint,
        username: settings.username,
        device_name: settings.deviceName,
        idle_timeout_mins: settings.idleTimeoutMins,
        auto_mode: settings.autoMode,
        developer_mode: settings.developerMode
      }
    });
    
    // Update local state
    isAutoMode.value = settings.autoMode;
    
    // Close settings
    closeSettings();
  } catch (error) {
    console.error("Failed to save settings:", error);
  }
}

onMounted(() => {
  initApp();
  
  // Listen for settings open event
  invoke("open_settings").then(() => {
    openSettings();
  }).catch(() => {
    // Ignore errors
  });
});
</script>

<template>
  <main class="container">
    <h1>Remodance</h1>
    <p class="subtitle">Virtual Attendance System</p>

    <div class="status-card">
      <div class="status-indicator" :class="{ active: isCheckedIn }"></div>
      <p class="status-text">{{ isCheckedIn ? 'Checked In' : 'Checked Out' }}</p>
      <p class="mode-text">{{ isAutoMode ? 'Auto Mode Enabled' : 'Manual Mode' }}</p>
      
      <button class="attendance-btn" :class="{ 'checked-in': isCheckedIn }" @click="toggleAttendance">
        {{ isCheckedIn ? 'Check Out' : 'Check In' }}
      </button>
    </div>

    <div class="settings-row">
      <button @click="openSettings" class="settings-btn">
        Settings
      </button>
    </div>

    <footer>
      <p>Version: {{ appVersion }}</p>
    </footer>
    
    <!-- Settings Modal -->
    <div v-if="showSettings" class="settings-modal">
      <div class="settings-content">
        <h2>Settings</h2>
        
        <div class="form-group">
          <label for="apiEndpoint">API Endpoint URL</label>
          <input id="apiEndpoint" v-model="settings.apiEndpoint" type="text" placeholder="https://example.com/attendance" />
        </div>
        
        <div class="form-group">
          <label for="username">Username</label>
          <input id="username" v-model="settings.username" type="text" />
        </div>
        
        <div class="form-group">
          <label for="deviceName">Device Name</label>
          <input id="deviceName" v-model="settings.deviceName" type="text" />
        </div>
        
        <div class="form-group form-checkbox">
          <input id="developerMode" v-model="settings.developerMode" type="checkbox" />
          <label for="developerMode">Enable Developer Mode</label>
        </div>
        
        <div v-if="settings.developerMode" class="developer-settings">
          <h3>Developer Options</h3>
          <div class="form-group">
            <label for="idleTimeout">Idle Timeout (minutes)</label>
            <input id="idleTimeout" v-model="settings.idleTimeoutMins" type="number" min="1" />
          </div>
          
          <div class="form-group form-checkbox">
            <input id="autoMode" v-model="settings.autoMode" type="checkbox" />
            <label for="autoMode">Enable Auto Mode</label>
          </div>
        </div>
        
        <div class="form-actions">
          <button @click="closeSettings" class="cancel-btn">Cancel</button>
          <button @click="saveSettings" class="save-btn">Save</button>
        </div>
      </div>
    </div>
  </main>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding: 2rem;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  max-width: 500px;
  margin: 0 auto;
}

h1 {
  font-size: 2rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.subtitle {
  font-size: 1rem;
  color: #64748b;
  margin-top: 0;
  margin-bottom: 2rem;
}

.status-card {
  background-color: white;
  border-radius: 12px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  padding: 2rem;
  margin-bottom: 2rem;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.status-indicator {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background-color: #94a3b8;
  margin-bottom: 1rem;
}

.status-indicator.active {
  background-color: #22c55e;
}

.status-text {
  font-size: 1.5rem;
  font-weight: 600;
  margin: 0;
}

.mode-text {
  font-size: 0.875rem;
  color: #64748b;
  margin-top: 0.5rem;
  margin-bottom: 1.5rem;
}

.attendance-btn {
  background-color: #3b82f6;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 0.8rem 2rem;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  width: 100%;
  max-width: 200px;
}

.attendance-btn:hover {
  background-color: #2563eb;
}

.attendance-btn.checked-in {
  background-color: #ef4444;
}

.attendance-btn.checked-in:hover {
  background-color: #dc2626;
}

.settings-row {
  display: flex;
  justify-content: center;
}

.settings-btn {
  background-color: transparent;
  color: #64748b;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  padding: 0.6rem 1.2rem;
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s;
}

.settings-btn:hover {
  background-color: #f8fafc;
  color: #334155;
}

footer {
  margin-top: 2rem;
  font-size: 0.75rem;
  color: #94a3b8;
}

/* Settings Modal */
.settings-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 1rem;
  overflow-y: auto;
}

.settings-content {
  background-color: white;
  border-radius: 12px;
  width: 90%;
  max-width: 500px;
  padding: 1.5rem;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  max-height: 90vh;
  overflow-y: auto;
}

.settings-content h2 {
  margin-top: 0;
  margin-bottom: 1.5rem;
  font-size: 1.5rem;
  color: #334155;
}

.form-group {
  margin-bottom: 1.25rem;
  text-align: left;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.875rem;
  color: #64748b;
  font-weight: 500;
}

.form-group input[type="text"],
.form-group input[type="number"] {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  color: #334155;
  box-sizing: border-box;
}

.form-checkbox {
  display: flex;
  align-items: center;
}

.form-checkbox input {
  margin-right: 0.5rem;
  min-width: 18px;
  min-height: 18px;
}

.form-checkbox label {
  margin-bottom: 0;
}

.developer-settings {
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid #e2e8f0;
}

.developer-settings h3 {
  font-size: 1.1rem;
  color: #64748b;
  margin-top: 0;
  margin-bottom: 1rem;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 2rem;
  gap: 1rem;
}

.cancel-btn {
  background-color: transparent;
  color: #64748b;
  border: 1px solid #e2e8f0;
  border-radius: 8px;
  padding: 0.6rem 1.2rem;
  font-size: 0.875rem;
  cursor: pointer;
}

.save-btn {
  background-color: #3b82f6;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 0.6rem 1.2rem;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
}

.save-btn:hover {
  background-color: #2563eb;
}

/* Responsive styles */
@media (max-width: 600px) {
  .settings-content {
    width: 95%;
    padding: 1rem;
  }
  
  .form-group label {
    font-size: 0.8rem;
  }
  
  .form-group input[type="text"],
  .form-group input[type="number"] {
    padding: 0.6rem;
    font-size: 0.9rem;
  }
  
  .form-actions {
    flex-direction: column-reverse;
    gap: 0.5rem;
  }
  
  .save-btn, .cancel-btn {
    width: 100%;
    padding: 0.75rem;
  }
}
</style>
