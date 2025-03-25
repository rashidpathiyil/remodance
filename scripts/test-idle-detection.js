#!/usr/bin/env node

/**
 * Test script for validating idle detection
 * 
 * This script simulates user inactivity by not interacting with the system for a configurable
 * amount of time, then verifies that the application detected the idle state correctly.
 */

const { exec } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

// Configuration
const IDLE_TIMEOUT_MINS = 2; // Set this lower than the app's setting for testing
const TEST_DURATION_MINS = IDLE_TIMEOUT_MINS + 1;

console.log(`Idle Detection Test Script`);
console.log(`=========================`);
console.log(`Platform: ${os.platform()}`);
console.log(`Testing idle detection for ${TEST_DURATION_MINS} minutes...`);
console.log(`\nThis test will:`);
console.log(`1. Monitor the application's attendance status`);
console.log(`2. Simulate user inactivity for ${TEST_DURATION_MINS} minutes`);
console.log(`3. Verify that the application detects idle state and checks out`);

// Platform-specific log file paths
let logPath;
switch (os.platform()) {
  case 'win32':
    logPath = path.join(os.homedir(), 'AppData', 'Roaming', 'remodance', 'logs', 'app.log');
    break;
  case 'darwin':
    logPath = path.join(os.homedir(), 'Library', 'Logs', 'remodance', 'app.log');
    break;
  default: // Linux
    logPath = path.join(os.homedir(), '.local', 'share', 'remodance', 'logs', 'app.log');
    break;
}

// Function to check the log file for idle detection events
function checkIdleDetection() {
  try {
    if (fs.existsSync(logPath)) {
      const logs = fs.readFileSync(logPath, 'utf8');
      
      // Look for idle detection events in the logs
      const idleDetected = logs.includes('User is idle');
      const checkedOut = logs.includes('Automatically checking out');
      
      console.log('\nTest Results:');
      console.log(`- Idle detected: ${idleDetected ? '✅' : '❌'}`);
      console.log(`- Auto check-out: ${checkedOut ? '✅' : '❌'}`);
      
      if (idleDetected && checkedOut) {
        console.log('\n✅ TEST PASSED: Idle detection is working correctly');
        return true;
      } else {
        console.log('\n❌ TEST FAILED: Idle detection issues detected');
        console.log('Check the application logs for more details');
        return false;
      }
    } else {
      console.log(`\n❌ Could not find log file at ${logPath}`);
      return false;
    }
  } catch (error) {
    console.error(`Error checking idle detection: ${error}`);
    return false;
  }
}

// Main test function
async function runTest() {
  console.log(`\nStarting test, please do not move mouse or type for ${TEST_DURATION_MINS} minutes...`);
  
  // Create countdown
  let remainingMins = TEST_DURATION_MINS;
  const countdownInterval = setInterval(() => {
    remainingMins--;
    console.log(`${remainingMins} minute${remainingMins !== 1 ? 's' : ''} remaining...`);
    
    if (remainingMins <= 0) {
      clearInterval(countdownInterval);
      
      // Check the results
      const passed = checkIdleDetection();
      process.exit(passed ? 0 : 1);
    }
  }, 60000); // Update every minute
}

// Run the test
runTest(); 
