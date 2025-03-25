**Virtual Attendance System Application**

**1\. Project Overview**

* **Application Name:** Remodance  
* **Description:** A cross-platform desktop application designed to automate attendance tracking for virtual environments. The application monitors user activity, detects idle time, and automatically manages check-in/check-out events by sending data to a designated API. The application also provides a manual check-in/check-out option.  
* **Target Platforms:**  
  * Linux (Lubuntu, Fedora)  
  * Windows (Windows 10, Windows 11\)  
  * macOS (Latest three versions)  
* **Open Source:**  
  * Repository: [https://github.com/rashidpathiyil](https://github.com/rashidpathiyil)  
  * License: MIT License  
* **Primary Goal:** To provide a seamless, efficient, and accurate way for users to track their attendance in virtual settings, minimizing manual effort.  
* **Secondary Goals:**  
  * Offer a user-friendly interface with clear settings and feedback.  
  * Ensure reliable operation, even in offline conditions.  
  * Facilitate easy configuration and customization.  
  * Adhere to software development best practices for maintainability and scalability.

**2\. Core Functionality**

* **2.1. Attendance Management**  
  * **2.1.1. Automatic Check-In/Check-Out (Auto-Mode):**  
    * Enabled by default.  
    * The application automatically determines the user's attendance status (present/absent) based on system activity.  
    * Check-in occurs when the system becomes active after an idle period.  
    * Check-out occurs when the system is idle for a configurable duration.  
    * The application should use an event-driven approach.  
  * **2.1.2. Manual Check-In/Check-Out:**  
    * A prominent toggle button in the main UI window.  
    * Label: "Check In / Check Out" (Label should toggle based on current state).  
    * Functionality:  
      * When clicked, the button sends a "check-in" or "check-out" event to the API.  
      * The UI updates to reflect the new status.  
      * Manual actions override auto-mode until the next system activity change (e.g., if a user manually checks out, auto-mode should not check them back in until they become active again).  
* **2.2. System Activity Monitoring**  
  * **2.2.1. Idle Time Detection:**  
    * Cross-platform implementation using native OS APIs.  
    * Accurate and reliable detection of user inactivity (keyboard, mouse, screen activity).  
    * Configurable idle timeout in developer mode (see 2.4).  
    * Use appropriate platform-specific APIs:  
      * Windows: GetLastInputInfo  
      * macOS: CGEventSourceSecondsSinceLastEventType  
      * Linux (X11): XScreenSaverQueryInfo (or similar)  
  * **2.2.2. Activity Threshold:**  
    * Consider a small activity threshold. (e.g., a single key press or mouse movement within a short period)  
    * This will prevent false check-in/check-out events for very brief pauses.  
* **2.3. API Communication**  
  * **2.3.1. API Endpoint:**  
    * User-configurable in the application settings.  
    * Default value: https://example.com/attendance (This MUST be changed to a real example, or a placeholder that indicates it *must* be changed).  
    * Support for HTTPS.  
    * Error handling for invalid or unreachable endpoints (with user feedback).  
  * **2.3.2. Authentication:**  
    * (Optional, but Highly Recommended) If the API requires authentication, implement a secure method. Consider these options in order of preference:  
      * **Option 1: API Key:** A user-configurable API key in settings. Store securely (e.g., using the OS's credential manager).  
      * **Option 2: OAuth 2.0:** If the API supports it, this is the most secure option.  
      * If no authentication is used, provide a VERY CLEAR warning in the settings UI.  
    * The application should handle authentication errors gracefully (e.g., invalid key, token expired) and provide informative messages to the user.  
  * **2.3.3. Payload Formatting:**  
    * Adhere strictly to the specified JSON payload structure (see section 4).  
    * Ensure correct data types and formatting.  
    * Include a UTC timestamp for all API requests.  
  * **2.3.4. Offline Handling:**  
    * When the application detects a network disconnection:  
      * Queue API requests in a persistent storage (e.g., a local database or file).  
      * Provide visual feedback to the user (e.g., a small indicator in the UI).  
    * When the network connection is restored:  
      * Process the queued requests in the order they were generated.  
      * Implement a retry mechanism for failed requests (with a maximum number of retries and exponential backoff).  
  * **2.3.5. Error Handling:**  
    * Handle API errors (e.g., 400, 401, 500\) gracefully.  
    * Log errors for debugging.  
    * Provide informative error messages to the user (without exposing sensitive information).  
* **2.4. Application Settings**  
  * **2.4.1. Access:**  
    * Context menu (system tray icon).  
    * Menu item in the main application window (if applicable).  
    * Consistent design across all platforms.  
  * **2.4.2. Basic Settings:**  
    * API Endpoint URL (text input).  
    * Username (text input, pre-filled with system username).  
    * Device Name (text input, pre-filled with hostname).  
  * **2.4.3. Developer Mode (Hidden/Toggleable):**  
    * A way to enable/disable developer mode (e.g., a specific menu option, a hidden button, or a command-line flag). The method should be documented.  
    * When enabled, the following settings become visible/editable:  
      * Idle Timeout (minutes) (numeric input).  
      * Auto-Mode (toggle switch).  
  * **2.4.4. Persistence:**  
    * Store settings persistently across application sessions (e.g., using a configuration file or a local database). Consider using a library that handles this.  
  * **2.4.5. Validation:**  
    * Validate user input in the settings (e.g., ensure the API endpoint is a valid URL, the idle timeout is a positive number).  
    * Provide clear error messages for invalid input.  
* **2.5. Application Lifecycle**  
  * **2.5.1. Auto-Start on Startup:**  
    * The application should automatically launch when the user logs in to their operating system.  
    * Implement platform-specific mechanisms for achieving this.  
  * **2.5.2. Background Operation:**  
    * The application should run primarily in the background (e.g., in the system tray) and minimize its visual footprint.  
    * Provide a system tray icon with a context menu for accessing settings, checking status, and exiting the application.  
  * **2.5.3. Application Exit:**  
    * Provide a clean way to exit the application (e.g., a "Quit" option in the system tray context menu).  
    * Ensure that all pending API requests are handled or queued before the application exits.  
* **2.6. Updates and Versioning**  
  * **2.6.1. Update Mechanism:**  
    * Implement a way for the application to check for new versions.  
    * Use a reliable update mechanism (e.g., connecting to a GitHub releases page).  
    * Support automatic updates (optional, but preferred) or prompt the user to download and install updates manually.  
  * **2.6.2. Version Display:**  
    * Display the current application version in the settings window or an "About" section.  
    * Use a consistent versioning scheme (e.g., Semantic Versioning).

**3\. User Interface (UI) Design**

* **3.1. Overall Design Principles:**  
  * Clean, modern, and intuitive.  
  * Platform-consistent look and feel.  
  * Minimalist design to reduce clutter.  
  * Clear visual feedback for user actions.  
  * Accessibility: Ensure the application is usable by people with disabilities (e.g., keyboard navigation, screen reader compatibility).  
* **3.2. Key UI Elements:**  
  * **System Tray Icon:**  
    * A subtle icon in the system tray to indicate the application's running status.  
    * Context menu with options:  
      * "Settings"  
      * "About" (displays version information)  
      * "Check for Updates"  
      * "Quit"  
  * **Main Window with** Check-In/Check-Out Button\*\*:\*\*  
    * Main window is needed, it should be simple and focused.  
    * Displaying the "Check In" / "Check Out" Button (reflects current state)  
    * A prominent toggle button, clearly labeled.  
    * Visual indication of the current state (e.g., different colors or icons for "Checked In" and "Checked Out").  
  * **Settings Window:**  
    * Organized layout with clear labels and input fields.  
    * Use appropriate UI controls (e.g., text boxes, dropdowns, checkboxes, toggle switches).  
    * Validation messages for incorrect input.  
  * **Notifications(only if enabled):**  
    * Use OS native notifications to inform the user of check-in/check-out events.  
    * Provide an option to disable notifications in the settings.

**4\. Data Format**

* **4.1. API Payload Structure:**

{  
  "event\_type": "check-in", // "check-out"  
  "user\_id": "user123", //  Dynamically populated  
  "payload": {  
    "time": "08:45:00", // HH:MM:SS (24-hour format)  
    "date": "2023-06-20", // YYYY-MM-DD  
    "device\_id": "device-001", // Dynamically populated  
    "config": { // Only included when developer mode is enabled  
      "idle\_timeout\_mins": 10, //  Only when developer mode is enabled  
      "auto\_mode": true //   Only when developer mode is enabled  
    }  
  },  
  "timestamp": "2024-07-24T10:00:00Z" // UTC timestamp (ISO 8601 format)  
}

* **4.2. Data Notes:**  
  * event\_type: "check-in" or "check-out" (lowercase).  
  * user\_id: The username of the user.  
  * time: Current time in 24-hour format (HH:MM:SS).  
  * date: Current date in YYYY-MM-DD format.  
  * device\_id: The unique identifier of the device.  
  * timestamp: Current timestamp in UTC, ISO 8601 format (e.g., "2024-07-24T10:00:00Z").  
  * idle\_timeout\_mins: Include this ONLY when developer mode is enabled.  
  * auto\_mode: Include this ONLY when developer mode is enabled.

**5\. Technology Stack**

* **Recommended:**  
  * Backend: Rust  
  * UI Framework: Tauri/Wry  
* **Rationale:**  
  * Rust:  
    * Performance: Excellent for system-level operations and responsiveness.  
    * Memory Safety: Prevents common programming errors.  
    * Cross-Platform: Well-suited for building applications for multiple operating systems.  
  * Tauri/Wry:  
    * Cross-Platform: Builds desktop apps using web technologies.  
    * Small Size: Smaller application bundles compared to Electron.  
    * Native Integration: Allows calling Rust functions for native functionality.  
    * Security: More secure than other web-based desktop frameworks.  
* **Alternative (If justified):** If there is a *strong* reason to deviate, please explain.

**6\. Development Guidelines**

* **6.1. Code Quality:**  
  * Adhere to Rust best practices (e.g., effective use of the borrow checker, proper error handling).  
  * Write clean, well-documented code with consistent formatting.  
  * Follow the principles of SOLID (Single Responsibility, Open/Closed, Liskov Substitution, Interface Segregation, Dependency Inversion).  
  * Use descriptive variable and function names.  
  * Keep functions short and focused.  
* **6.2. Error Handling:**  
  * Implement robust error handling for all operations (e.g., API calls, file operations, system API calls).  
  * Use Rust's Result type effectively.  
  * Provide informative error messages to the user (without exposing sensitive information).  
  * Log errors for debugging and troubleshooting.  
* **6.3. Concurrency:**  
  * Use Rust's concurrency features (e.g., threads, async/await) appropriately to prevent blocking the UI and ensure responsiveness.  
  * Avoid race conditions and data corruption.  
* **6.4. Testing:**  
  * Write unit tests for individual components and functions.  
  * Write integration tests to verify the interaction between different parts of the application.  
  * Test on all target platforms.  
  * Use a testing framework.  
* **6.5. Documentation:**  
  * Write clear and comprehensive documentation:  
    * **README.md:** Project overview, setup instructions, build instructions, contribution guidelines, code of conduct.  
    * **Code comments:** Explain complex logic, non-obvious behavior, and public APIs.  
    * **API documentation:** Document the format of the API requests and responses.  
* **6.6. Version Control:**  
  * Use Git for version control.  
  * Follow a consistent branching strategy (e.g., Gitflow or a simplified version).  
  * Write clear and concise commit messages.  
  * Use pull requests for code review.  
* **6.7. Build Automation:**  
  * Use GitHub Actions for continuous integration and continuous deployment (CI/CD).  
  * Automate the build process for all target platforms.  
  * Generate application bundles for distribution.  
  * Automate the release process.  
* **6.8. Security:**  
  * Follow security best practices to protect user data and prevent vulnerabilities.  
  * Sanitize user inputs.  
  * Store sensitive data securely (e.g., using the OS's credential manager).  
  * Be aware of common desktop application vulnerabilities.

**7\. Contributor Guidance**

* **7.1. Getting Started:**  
  * Provide a clear and easy-to-follow README.md file.  
  * Include detailed instructions on how to set up the development environment.  
  * Explain the project's architecture and how the different components interact.  
* **7.2. Contribution Workflow:**  
  * Define a clear contribution workflow (e.g., how to submit pull requests, how to report bugs).  
  * Use a code of conduct to ensure a welcoming and respectful community.  
  * Label issues clearly (e.g., "bug," "enhancement," "help wanted").  
* **7.3. Code Style:**  
  * Define a consistent code style (e.g., using a Rust formatter like rustfmt).  
  * Enforce the code style using a linter (e.g., clippy).  
* **7.4. Communication:**  
  * Use GitHub issues and pull requests for communication.  
  * Be responsive to questions and feedback from contributors.  
  * Provide a way for contributors to ask questions (e.g., a forum or chat channel).  
* **7.5. Code Reviews:**  
  * Conduct thorough code reviews to ensure code quality and prevent bugs.  
  * Provide constructive feedback to contributors.  
  * Be patient and helpful with new contributors.
