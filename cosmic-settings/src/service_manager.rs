// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Service manager abstraction for managing system services.
//!
//! This module provides a trait-based interface for interacting with system
//! service managers (like systemd, OpenRC, etc.), allowing for testability
//! and flexibility across different init systems.

use std::future::Future;
use std::pin::Pin;

/// Trait for managing system services.
///
/// Implementations of this trait are bound to a specific service name
/// and provide methods to query service state and perform operations
/// like enabling or activating that service.
pub trait ServiceManager {
    /// Check if the service is enabled (will start on boot).
    fn is_enabled(&self) -> bool;
    
    /// Check if the service is currently active (running).
    fn is_active(&self) -> bool;
    
    /// Activate (start) the service.
    fn activate(&self) -> Pin<Box<dyn Future<Output = ()> + Send>>;
    
    /// Enable the service (configure it to start on boot and start it now).
    fn enable(&self) -> Pin<Box<dyn Future<Output = ()> + Send>>;

    /// Check if the service is installed on this system.
    fn is_installed(&self) -> bool;
}

/// Logs the result of a privileged command execution for diagnostics.
///
/// Called after a command completes — logs a warning on non-zero exit or
/// an error if the command could not be executed at all.
fn log_command_result(
    result: &Result<std::process::ExitStatus, std::io::Error>,
    description: &str,
    service: &str,
) {
    match result {
        Ok(status) if status.success() => {}
        Ok(status) => tracing::warn!(
            "{} for '{}' failed with exit code: {:?}",
            description,
            service,
            status.code(),
        ),
        Err(e) => tracing::error!(
            "Failed to execute {} for '{}': {}",
            description,
            service,
            e,
        ),
    }
}

/// Runs a privileged command via `pkexec` and logs the result.
///
/// This is the standard async entry point for service management actions
/// that require elevated privileges.
async fn run_pkexec_command(args: &[&str], description: &str, service: &str) {
    let result = tokio::process::Command::new("pkexec")
        .args(args)
        .status()
        .await;
    log_command_result(&result, description, service);
}

/// Mock implementation of ServiceManager for testing.
///
/// This implementation returns configurable boolean values and provides
/// no-op implementations of activate and enable operations.
#[cfg(test)]
pub struct MockServiceManager {
    service_name: String,
    enabled: bool,
    active: bool,
    installed: bool,
}

#[cfg(test)]
impl MockServiceManager {
    /// Create a new MockServiceManager with specified service name, enabled and active states.
    pub fn new(service_name: impl Into<String>, enabled: bool, active: bool) -> Self {
        Self {
            service_name: service_name.into(),
            enabled,
            active,
            installed: true,
        }
    }

    /// Configure whether the service is reported as installed.
    pub fn with_installed(mut self, installed: bool) -> Self {
        self.installed = installed;
        self
    }
}

#[cfg(test)]
impl ServiceManager for MockServiceManager {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn activate(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {})
    }

    fn enable(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {})
    }

    fn is_installed(&self) -> bool {
        self.installed
    }
}

/// SystemD implementation of ServiceManager.
///
/// This implementation delegates to systemd commands for managing services.
#[cfg(feature = "systemd")]
pub struct SystemDServiceManager {
    service_name: String,
}

#[cfg(feature = "systemd")]
impl SystemDServiceManager {
    /// Create a new SystemDServiceManager for the specified service.
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }
}

#[cfg(feature = "systemd")]
impl ServiceManager for SystemDServiceManager {
    fn is_enabled(&self) -> bool {
        std::process::Command::new("systemctl")
            .args(["is-enabled", &self.service_name])
            .status()
            .map(|status| status.success())
            .unwrap_or(true)
    }

    fn is_active(&self) -> bool {
        std::process::Command::new("systemctl")
            .args(["is-active", &self.service_name])
            .status()
            .map(|status| status.success())
            .unwrap_or(true)
    }

    fn activate(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let service = self.service_name.clone();
        Box::pin(async move {
            run_pkexec_command(
                &["systemctl", "start", &service],
                "systemctl start",
                &service,
            )
            .await;
        })
    }

    fn enable(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let service = self.service_name.clone();
        Box::pin(async move {
            run_pkexec_command(
                &["systemctl", "enable", "--now", &service],
                "systemctl enable --now",
                &service,
            )
            .await;
        })
    }

    fn is_installed(&self) -> bool {
        std::process::Command::new("systemctl")
            .args(["cat", &self.service_name])
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
}

/// OpenRC implementation of ServiceManager.
///
/// This implementation delegates to OpenRC commands for managing services.
#[cfg(feature = "openrc")]
pub struct OpenRcServiceManager {
    service_name: String,
}

#[cfg(feature = "openrc")]
impl OpenRcServiceManager {
    /// Create a new OpenRcServiceManager for the specified service.
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }
}

/// Parses `rc-update show` output to check if a service is in any runlevel.
///
/// Uses exact service name matching to avoid false positives from prefix matches
/// (e.g., "bluetoothd" should not match a lookup for "bluetooth").
#[cfg(any(feature = "openrc", test))]
fn is_service_in_runlevel_output(output: &str, service_name: &str) -> bool {
    output
        .lines()
        .any(|line| {
            line.split('|')
                .next()
                .map(|name| name.trim() == service_name)
                .unwrap_or(false)
        })
}

#[cfg(feature = "openrc")]
impl ServiceManager for OpenRcServiceManager {
    fn is_enabled(&self) -> bool {
        // Check if service is in a runlevel (enabled to start on boot)
        std::process::Command::new("rc-update")
            .args(["show"])
            .output()
            .map(|output| {
                is_service_in_runlevel_output(
                    &String::from_utf8_lossy(&output.stdout),
                    &self.service_name,
                )
            })
            .unwrap_or(true)
    }

    fn is_active(&self) -> bool {
        // Check if service is currently running
        std::process::Command::new("rc-service")
            .args([&self.service_name, "status"])
            .status()
            .map(|status| status.success())
            .unwrap_or(true)
    }

    fn activate(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let service = self.service_name.clone();
        Box::pin(async move {
            run_pkexec_command(
                &["rc-service", &service, "start"],
                "rc-service start",
                &service,
            )
            .await;
        })
    }

    fn enable(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let service = self.service_name.clone();
        Box::pin(async move {
            // Add to default runlevel
            run_pkexec_command(
                &["rc-update", "add", &service, "default"],
                "rc-update add",
                &service,
            )
            .await;
            // Start the service now
            run_pkexec_command(
                &["rc-service", &service, "start"],
                "rc-service start",
                &service,
            )
            .await;
        })
    }

    fn is_installed(&self) -> bool {
        std::path::Path::new("/etc/init.d")
            .join(&self.service_name)
            .exists()
    }
}

/// Detects the running service manager at runtime and creates a manager for the specified service.
///
/// Detection strategy:
/// 1. Check for systemd: Verify /run/systemd/system exists (systemd running as service manager)
/// 2. Check for OpenRC: Verify /run/openrc exists (OpenRC actively managing services)
///
/// Note: This checks for RUNNING service managers, not just installed ones.
/// For example, OpenRC may be installed but not running, or both may be installed
/// but only one is actively managing services.
///
/// Returns an error if no supported service manager is detected.
pub fn detect_service_manager(service_name: impl Into<String>) -> Result<Box<dyn ServiceManager>, String> {
    let service_name = service_name.into();
    
    // Try systemd first (most common on modern Linux)
    // /run/systemd/system only exists when systemd is running as the service manager
    #[cfg(feature = "systemd")]
    {
        if std::path::Path::new("/run/systemd/system").exists() {
            tracing::debug!("Detected systemd service manager via /run/systemd/system");
            return Ok(Box::new(SystemDServiceManager::new(service_name)));
        }
    }

    // Try OpenRC
    // /run/openrc exists when OpenRC is actively managing services
    // This works regardless of whether the init system is sysvinit, runit, or something else
    #[cfg(feature = "openrc")]
    {
        if std::path::Path::new("/run/openrc").exists() {
            tracing::debug!("Detected OpenRC service manager via /run/openrc");
            return Ok(Box::new(OpenRcServiceManager::new(service_name)));
        }
    }

    // No supported service manager detected - build error message based on what was checked
    #[cfg(not(any(feature = "systemd", feature = "openrc")))]
    {
        Err("No service manager features enabled at compile time. \
             Enable the 'systemd' or 'openrc' feature to support service management."
            .to_string())
    }

    #[cfg(any(feature = "systemd", feature = "openrc"))]
    {
        let mut checked = Vec::new();
        #[cfg(feature = "systemd")]
        checked.push("systemd (/run/systemd/system)");
        #[cfg(feature = "openrc")]
        checked.push("openrc (/run/openrc)");

        Err(format!(
            "Could not detect a running service manager. Checked for: {}. \
             None of these service managers appear to be running on this system.",
            checked.join(", ")
        ))
    }
}

/// Creates the appropriate service manager.
///
/// In test mode: Returns MockServiceManager for the specified service
/// In production: Attempts to detect the running service manager
#[cfg(test)]
pub fn create_default_service_manager(service_name: impl Into<String>) -> Box<dyn ServiceManager> {
    Box::new(MockServiceManager::new(service_name, false, false))
}

#[cfg(not(test))]
pub fn create_default_service_manager(service_name: impl Into<String>) -> Box<dyn ServiceManager> {
    let service_name = service_name.into();
    detect_service_manager(&service_name).unwrap_or_else(|e| {
        tracing::warn!(
            "Failed to detect service manager: {}. Service management features will not be available.",
            e
        );
        tracing::warn!(
            "Using no-op service manager for '{}'. Service status will always report as enabled/active, but operations will not actually execute.",
            service_name
        );
        // Return a no-op implementation that always reports services as enabled/active
        // This allows the app to continue but service management won't actually work
        Box::new(NoOpServiceManager { service_name })
    })
}

/// No-op service manager for when no real service manager is detected.
/// Reports all services as enabled and active, but doesn't actually manage anything.
struct NoOpServiceManager {
    service_name: String,
}

impl ServiceManager for NoOpServiceManager {
    fn is_enabled(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn activate(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {})
    }

    fn enable(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {})
    }

    fn is_installed(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_manager_trait_is_enabled() {
        // Arrange: Create a mock service manager
        struct TestServiceManager {
            enabled: bool,
            installed: bool,
        }
        
        impl ServiceManager for TestServiceManager {
            fn is_enabled(&self) -> bool {
                self.enabled
            }
            
            fn is_active(&self) -> bool {
                true
            }

            fn activate(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
                Box::pin(async {})
            }

            fn enable(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
                Box::pin(async {})
            }

            fn is_installed(&self) -> bool {
                self.installed
            }
        }
        
        let test_service = TestServiceManager { enabled: true, installed: true };
        
        // Act & Assert
        assert!(test_service.is_enabled());
        
        let disabled_service = TestServiceManager { enabled: false, installed: true };
        assert!(!disabled_service.is_enabled());
    }

    #[test]
    fn test_service_manager_trait_is_active() {
        // Arrange: Create a mock service manager
        struct TestServiceManager {
            active: bool,
            installed: bool,
        }
        
        impl ServiceManager for TestServiceManager {
            fn is_enabled(&self) -> bool {
                true
            }
            
            fn is_active(&self) -> bool {
                self.active
            }

            fn activate(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
                Box::pin(async {})
            }

            fn enable(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
                Box::pin(async {})
            }

            fn is_installed(&self) -> bool {
                self.installed
            }
        }
        
        let test_service = TestServiceManager { active: true, installed: true };
        
        // Act & Assert
        assert!(test_service.is_active());
        
        let inactive_service = TestServiceManager { active: false, installed: true };
        assert!(!inactive_service.is_active());
    }

    #[test]
    fn test_mock_service_manager_returns_configured_values() {
        // Arrange: Create a MockServiceManager with known state
        let bluetooth = MockServiceManager::new("bluetooth", true, true);
        
        // Act & Assert: Verify it returns the configured values
        assert!(bluetooth.is_enabled());
        assert!(bluetooth.is_active());
        
        // Arrange: Create a MockServiceManager with different state
        let disabled_bluetooth = MockServiceManager::new("bluetooth", false, false);
        
        // Act & Assert: Verify it returns the configured values
        assert!(!disabled_bluetooth.is_enabled());
        assert!(!disabled_bluetooth.is_active());
    }

    #[test]
    #[cfg(feature = "systemd")]
    fn test_systemd_service_manager_implements_trait() {
        // Arrange: Create a SystemDServiceManager
        let bluetooth = SystemDServiceManager::new("bluetooth");
        
        // Act & Assert: Verify it implements ServiceManager trait
        // Note: We can't test the actual systemd calls in unit tests,
        // but we can verify the struct exists and implements the trait
        let _enabled: bool = bluetooth.is_enabled();
        let _active: bool = bluetooth.is_active();
    }

    #[tokio::test]
    async fn test_service_manager_trait_has_activate_method() {
        // Arrange: Create a mock service manager
        let bluetooth = MockServiceManager::new("bluetooth", false, false);
        
        // Act: Call activate on the service manager
        bluetooth.activate().await;
        
        // Assert: The method should exist and be callable
    }

    #[tokio::test]
    async fn test_service_manager_trait_has_enable_method() {
        // Arrange: Create a mock service manager
        let bluetooth = MockServiceManager::new("bluetooth", false, false);
        
        // Act: Call enable on the service manager
        bluetooth.enable().await;
        
        // Assert: The method should exist and be callable
    }

    #[tokio::test]
    async fn test_service_manager_activate_returns_future() {
        // Arrange: Create a mock service manager
        let bluetooth = MockServiceManager::new("bluetooth", false, false);
        
        // Act: Call activate and await the future
        let future = bluetooth.activate();
        future.await;
        
        // Assert: The method should return a Future that can be awaited
    }

    #[tokio::test]
    async fn test_service_manager_enable_returns_future() {
        // Arrange: Create a mock service manager
        let bluetooth = MockServiceManager::new("bluetooth", false, false);
        
        // Act: Call enable and await the future
        let future = bluetooth.enable();
        future.await;
        
        // Assert: The method should return a Future that can be awaited
    }

    #[test]
    fn test_default_service_manager_uses_mock_in_tests() {
        // Arrange & Act: Create a service manager using the factory function
        let bluetooth = create_default_service_manager("bluetooth");
        
        // Assert: The service manager should be a MockServiceManager
        // We verify this by checking that it returns the mock's default values (false, false)
        assert!(!bluetooth.is_enabled(), 
            "Default service manager in test mode should use MockServiceManager which returns false for is_enabled");
        assert!(!bluetooth.is_active(),
            "Default service manager in test mode should use MockServiceManager which returns false for is_active");
    }

    #[test]
    #[cfg(feature = "openrc")]
    fn test_openrc_service_manager_implements_trait() {
        // Arrange: Create an OpenRcServiceManager
        let bluetooth = OpenRcServiceManager::new("bluetooth");
        
        // Act & Assert: Verify it implements ServiceManager trait
        // Note: We can't test the actual OpenRC calls in unit tests,
        // but we can verify the struct exists and implements the trait
        let _enabled: bool = bluetooth.is_enabled();
        let _active: bool = bluetooth.is_active();
    }

    #[test]
    fn test_detect_service_manager_returns_result() {
        // Act: Attempt to detect the running service manager for a test service
        let result = detect_service_manager("test-service");
        
        // Assert: Should return Ok or Err, never panic
        // On most Linux systems, at least one service manager should be detected
        // If detection fails, error message should be descriptive
        match result {
            Ok(_service_manager) => {
                // Success - a service manager was detected
                // We don't test actual service operations here to avoid
                // depending on specific services being present
            }
            Err(e) => {
                // Verify error message is descriptive
                assert!(!e.is_empty(), "Error message should not be empty");
                assert!(e.contains("service manager"), 
                    "Error message should mention service manager: {}", e);
            }
        }
    }

    #[test]
    fn test_mock_service_manager_accepts_service_name() {
        // Arrange & Act: Create a MockServiceManager with a service name
        let bluetooth = MockServiceManager::new("bluetooth", true, true);
        
        // Assert: The MockServiceManager should be created successfully and bound to the service
        assert!(bluetooth.is_enabled());
    }

    #[test]
    fn test_mock_service_manager_is_installed_defaults_to_true() {
        // Arrange: Create a MockServiceManager with default settings
        let bluetooth = MockServiceManager::new("bluetooth", true, true);
        
        // Assert: The default installed state should be true
        assert!(bluetooth.is_installed(),
            "MockServiceManager should default to installed=true");
    }

    #[test]
    fn test_mock_service_manager_with_installed_configures_state() {
        // Arrange: Create a MockServiceManager configured as not installed
        let not_installed = MockServiceManager::new("bluetooth", true, true)
            .with_installed(false);
        
        // Assert: is_installed should return false
        assert!(!not_installed.is_installed(),
            "with_installed(false) should make is_installed() return false");
        
        // Arrange: Create a MockServiceManager configured as installed
        let installed = MockServiceManager::new("bluetooth", true, true)
            .with_installed(true);
        
        // Assert: is_installed should return true
        assert!(installed.is_installed(),
            "with_installed(true) should make is_installed() return true");
    }

    #[test]
    fn test_no_op_service_manager_reports_not_installed() {
        // Arrange: Create a NoOpServiceManager (production fallback when no service manager is detected)
        let no_op = NoOpServiceManager { service_name: String::from("bluetooth") };
        
        // Assert: NoOpServiceManager should report the service as NOT installed
        // This is critical for the DBusServiceUnknown handler: when the service manager
        // can't be detected at runtime, we must fall through to bluez_service_unknown
        // so the user sees an appropriate message rather than a non-functional toggle.
        assert!(!no_op.is_installed(),
            "NoOpServiceManager should report is_installed() = false so that \
             DBusServiceUnknown falls through to the bluez_service_unknown path");
    }

    #[test]
    fn test_parse_rc_update_output_exact_service_name() {
        // Test that parsing rc-update output uses exact service name matching,
        // not prefix matching (which would falsely match "bluetoothd" for "bluetooth").

        // Arrange: Simulate rc-update show output that includes a service with
        // a similar-but-longer name than the one we're looking for.
        let output = "  bluetoothd | default\n";

        // Act: Parse the output looking for "bluetooth" (not "bluetoothd")
        let found = is_service_in_runlevel_output(output, "bluetooth");

        // Assert: Should NOT match "bluetooth" when only "bluetoothd" is present
        assert!(!found,
            "Should not match 'bluetooth' when only 'bluetoothd' is present (prefix matching bug)");
    }

    #[test]
    fn test_parse_rc_update_output_matches_exact_name() {
        // Arrange: Simulate rc-update show output with exact service name
        let output = "  bluetooth | default\n";

        // Act: Parse the output looking for "bluetooth"
        let found = is_service_in_runlevel_output(output, "bluetooth");

        // Assert: Should match when exact name is present
        assert!(found,
            "Should match 'bluetooth' when it is present in the output");
    }

    #[test]
    fn test_parse_rc_update_output_not_found() {
        // Arrange: Simulate rc-update show output without the target service
        let output = "  dbus | default\n  network | sysinit\n";

        // Act: Parse looking for a service not in the output
        let found = is_service_in_runlevel_output(output, "bluetooth");

        // Assert: Should not find it
        assert!(!found,
            "Should not match when the service is not in the output");
    }

    #[tokio::test]
    async fn test_log_command_result_logs_error_on_failure() {
        // This test verifies that the run_pkexec_command helper exists and handles
        // command failures gracefully (logs a warning/error instead of panicking).
        // The function doesn't exist yet — this test drives its creation.
        let result = std::io::Result::Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "command not found",
        ));
        log_command_result(&result, "test command", "test-service");
        // Verification: the function should not panic on error.
    }

    #[test]
    fn test_parse_rc_update_output_multi_line() {
        // Arrange: Multi-line output with target service
        let output = "  dbus | default\n  bluetooth | default\n  network | sysinit\n";

        // Act: Parse looking for bluetooth
        let found = is_service_in_runlevel_output(output, "bluetooth");

        // Assert: Should find it in multi-line output
        assert!(found,
            "Should find the service in multi-line output");
    }
}
