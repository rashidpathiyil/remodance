# Remodance Cross-Platform Testing Guide

This document provides guidelines for testing the Remodance application across different platforms to ensure consistent functionality before release.

## Target Platforms

- **Windows:** Windows 10, Windows 11
- **macOS:** Latest three versions (Monterey, Ventura, Sonoma)
- **Linux:** Lubuntu, Fedora

## Test Environment Setup

### Windows Testing

1. **Virtual Machine Setup:**
   - Download Windows 10/11 VM images from [Microsoft's Developer site](https://developer.microsoft.com/en-us/windows/downloads/virtual-machines/)
   - Use VirtualBox, VMware, or Hyper-V to run the VM

2. **Required Software:**
   - Install Git
   - Install Node.js and npm
   - Install Rust and Cargo

### macOS Testing

1. **Virtual Machine Setup (if not testing on physical hardware):**
   - Create macOS VMs using [UTM](https://mac.getutm.app/) (Apple Silicon) or VMware (Intel)

2. **Required Software:**
   - Install Xcode Command Line Tools
   - Install Node.js and npm
   - Install Rust and Cargo

### Linux Testing

1. **Virtual Machine Setup:**
   - Download Lubuntu and Fedora ISOs
   - Create VMs using VirtualBox or QEMU

2. **Required Software:**
   - Install build dependencies: `sudo apt-get install libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev` (Ubuntu/Lubuntu)
   - Install build dependencies: `sudo dnf install gtk3-devel webkit2gtk3-devel libappindicator-gtk3-devel librsvg2-devel` (Fedora)
   - Install Node.js and npm
   - Install Rust and Cargo

## Testing Checklist

### Installation Testing

- [ ] Application installs correctly without errors
- [ ] Application appears in the Applications/Programs list
- [ ] Uninstallation works correctly

### Core Functionality Testing

- [ ] Application launches properly
- [ ] Auto-launch at system startup works correctly
- [ ] System tray icon appears and shows correct status
- [ ] Menu items in system tray work correctly
- [ ] Manual check-in/check-out functionality works
- [ ] Auto check-in/check-out based on system activity works
- [ ] Settings are saved and persisted between app restarts
- [ ] API communication works correctly when online

### Offline Mode Testing

- [ ] Application detects when network is unavailable
- [ ] Events are queued when offline
- [ ] Queued events are sent when network is restored
- [ ] Offline indicator is visible when in offline mode
- [ ] Offline queue is persisted between app restarts

### UI Testing

- [ ] UI renders correctly (no visual glitches)
- [ ] Windows are properly sized and positioned
- [ ] Settings dialog opens and displays correctly
- [ ] All form inputs work correctly
- [ ] Buttons and controls are responsive
- [ ] Status indicators update properly

### Platform-Specific Testing

#### Windows

- [ ] Application runs correctly on Windows 10
- [ ] Application runs correctly on Windows 11
- [ ] Auto-launch registry entry is created correctly
- [ ] System tray integration works properly
- [ ] User idle detection is accurate

#### macOS

- [ ] Application runs correctly on all targeted macOS versions
- [ ] Menu bar icon appears correctly
- [ ] Auto-launch LaunchAgent is created correctly
- [ ] Notification permissions work correctly
- [ ] User idle detection is accurate

#### Linux

- [ ] Application runs correctly on Lubuntu
- [ ] Application runs correctly on Fedora
- [ ] System tray integration works with different desktop environments (GNOME, KDE, etc.)
- [ ] Auto-launch desktop entry is created correctly
- [ ] X11 idle detection is accurate

### Performance Testing

- [ ] Application startup time is reasonable
- [ ] Memory usage is within acceptable limits
- [ ] CPU usage is minimal when idle
- [ ] No memory leaks during extended usage

### Security Testing

- [ ] API credentials are stored securely
- [ ] Network communication is secure
- [ ] No sensitive data is logged or exposed

## Testing Procedure

1. **Build the application locally using:**
   ```bash
   npm run tauri build
   ```

2. **Install the application on each target platform**

3. **Follow the testing checklist for each platform**

4. **Report any issues found in GitHub Issues with:**
   - Platform details (OS name and version)
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Screenshots (if applicable)

## Automated Testing

For CI/CD pipeline testing, refer to the GitHub Actions workflow in `.github/workflows/test.yml` which automatically builds and tests the application on Windows, macOS, and Linux platforms.

## Testing Matrix

| Feature                 | Windows 10 | Windows 11 | macOS Monterey | macOS Ventura | macOS Sonoma | Lubuntu | Fedora |
|-------------------------|------------|------------|----------------|---------------|--------------|---------|--------|
| Installation            |            |            |                |               |              |         |        |
| Launching               |            |            |                |               |              |         |        |
| Auto-launch             |            |            |                |               |              |         |        |
| System tray             |            |            |                |               |              |         |        |
| Check-in/out            |            |            |                |               |              |         |        |
| Idle detection          |            |            |                |               |              |         |        |
| Settings saving         |            |            |                |               |              |         |        |
| API communication       |            |            |                |               |              |         |        |
| Offline mode            |            |            |                |               |              |         |        |
| UI rendering            |            |            |                |               |              |         |        | 
