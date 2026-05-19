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
/// Implementations are bound to a specific service name and provide
/// methods to query service state and perform lifecycle operations.
pub trait ServiceManager {
    /// Whether the service is configured to start on boot.
    fn is_enabled(&self) -> bool;

    /// Whether the service is currently running.
    fn is_active(&self) -> bool;

    /// Start the service.
    fn activate(&self) -> Pin<Box<dyn Future<Output = ()> + Send>>;

    /// Enable the service to start on boot and start it now.
    fn enable(&self) -> Pin<Box<dyn Future<Output = ()> + Send>>;

    /// Whether the service is installed on this system.
    fn is_installed(&self) -> bool;
}

/// A newtype around `Box<dyn ServiceManager>` that enables `#[derive(Default)]`
/// on containing structs. Methods on `ServiceManager` are accessible via `Deref`.
///
/// Each consuming module provides its own `Default` impl with the appropriate
/// service name.
pub struct ServiceManagerHandle(Box<dyn ServiceManager>);

impl ServiceManagerHandle {
    pub fn new(manager: Box<dyn ServiceManager>) -> Self {
        Self(manager)
    }
}

impl std::ops::Deref for ServiceManagerHandle {
    type Target = dyn ServiceManager;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

/// Log a warning or error after a privileged command completes.
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
        Err(e) => tracing::error!("Failed to execute {} for '{}': {}", description, service, e,),
    }
}

/// Run a privileged command via `pkexec`.
///
/// This is the common async entry point for service management actions
/// that require elevated privileges.
async fn run_pkexec_command(args: &[&str], description: &str, service: &str) {
    let result = tokio::process::Command::new("pkexec")
        .args(args)
        .status()
        .await;
    log_command_result(&result, description, service);
}

/// Mock ServiceManager that returns configurable boolean values.
#[cfg(test)]
pub struct MockServiceManager {
    enabled: bool,
    active: bool,
    installed: bool,
}

#[cfg(test)]
impl MockServiceManager {
    pub fn new(enabled: bool, active: bool) -> Self {
        Self {
            enabled,
            active,
            installed: true,
        }
    }

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
#[cfg(feature = "systemd")]
pub struct SystemDServiceManager {
    service_name: String,
}

#[cfg(feature = "systemd")]
impl SystemDServiceManager {
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
#[cfg(feature = "openrc")]
pub struct OpenRcServiceManager {
    service_name: String,
}

#[cfg(feature = "openrc")]
impl OpenRcServiceManager {
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
    output.lines().any(|line| {
        line.split('|')
            .next()
            .map(|name| name.trim() == service_name)
            .unwrap_or(false)
    })
}

#[cfg(feature = "openrc")]
impl ServiceManager for OpenRcServiceManager {
    fn is_enabled(&self) -> bool {
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
            run_pkexec_command(
                &["rc-update", "add", &service, "default"],
                "rc-update add",
                &service,
            )
            .await;
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

/// Detect the running service manager and create a manager for the named service.
///
/// Checks runtime directories (`/run/systemd/system`, `/run/openrc`) to determine
/// which service manager is actively running, rather than relying on which binaries
/// are installed.  This avoids false detections when multiple service managers are
/// installed but only one is actively managing services.
pub fn detect_service_manager(
    service_name: impl Into<String>,
) -> Result<Box<dyn ServiceManager>, String> {
    let service_name = service_name.into();

    #[cfg(feature = "systemd")]
    {
        if std::path::Path::new("/run/systemd/system").exists() {
            tracing::debug!("Detected systemd service manager via /run/systemd/system");
            return Ok(Box::new(SystemDServiceManager::new(service_name)));
        }
    }

    #[cfg(feature = "openrc")]
    {
        if std::path::Path::new("/run/openrc").exists() {
            tracing::debug!("Detected OpenRC service manager via /run/openrc");
            return Ok(Box::new(OpenRcServiceManager::new(service_name)));
        }
    }

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

/// Factory: returns `MockServiceManager` in tests, detects the running service manager in production.
#[cfg(test)]
pub fn create_default_service_manager(_service_name: impl Into<String>) -> Box<dyn ServiceManager> {
    Box::new(MockServiceManager::new(false, false))
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
        // Graceful degradation — app continues but service management won't work
        Box::new(NoOpServiceManager)
    })
}

/// Fallback when no service manager is detected — reports everything as active but does nothing.
struct NoOpServiceManager;

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
    use tracing_subscriber::util::SubscriberInitExt;

    #[test]
    fn test_service_manager_trait_is_enabled() {
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

        let test_service = TestServiceManager {
            enabled: true,
            installed: true,
        };

        assert!(test_service.is_enabled());

        let disabled_service = TestServiceManager {
            enabled: false,
            installed: true,
        };
        assert!(!disabled_service.is_enabled());
    }

    #[test]
    fn test_service_manager_trait_is_active() {
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

        let test_service = TestServiceManager {
            active: true,
            installed: true,
        };

        assert!(test_service.is_active());

        let inactive_service = TestServiceManager {
            active: false,
            installed: true,
        };
        assert!(!inactive_service.is_active());
    }

    #[test]
    fn test_mock_service_manager_returns_configured_values() {
        let bluetooth = MockServiceManager::new(true, true);
        assert!(bluetooth.is_enabled());
        assert!(bluetooth.is_active());

        let disabled_bluetooth = MockServiceManager::new(false, false);
        assert!(!disabled_bluetooth.is_enabled());
        assert!(!disabled_bluetooth.is_active());
    }

    #[test]
    #[cfg(feature = "systemd")]
    fn test_systemd_service_manager_implements_trait() {
        let bluetooth = SystemDServiceManager::new("bluetooth");

        // Can't test actual systemd calls in unit tests, but verify the struct exists
        let _enabled: bool = bluetooth.is_enabled();
        let _active: bool = bluetooth.is_active();
    }

    #[tokio::test]
    async fn test_service_manager_trait_has_activate_method() {
        let bluetooth = MockServiceManager::new(false, false);
        bluetooth.activate().await;
    }

    #[tokio::test]
    async fn test_service_manager_trait_has_enable_method() {
        let bluetooth = MockServiceManager::new(false, false);
        bluetooth.enable().await;
    }

    #[tokio::test]
    async fn test_service_manager_activate_returns_future() {
        let bluetooth = MockServiceManager::new(false, false);
        let future = bluetooth.activate();
        future.await;
    }

    #[tokio::test]
    async fn test_service_manager_enable_returns_future() {
        let bluetooth = MockServiceManager::new(false, false);
        let future = bluetooth.enable();
        future.await;
    }

    #[test]
    fn test_default_service_manager_uses_mock_in_tests() {
        let bluetooth = create_default_service_manager("bluetooth");

        assert!(!bluetooth.is_enabled());
        assert!(!bluetooth.is_active());
    }

    #[test]
    #[cfg(feature = "openrc")]
    fn test_openrc_service_manager_implements_trait() {
        let bluetooth = OpenRcServiceManager::new("bluetooth");

        // Can't test actual OpenRC calls in unit tests, but verify the struct exists
        let _enabled: bool = bluetooth.is_enabled();
        let _active: bool = bluetooth.is_active();
    }

    #[test]
    fn test_detect_service_manager_returns_result() {
        let result = detect_service_manager("test-service");

        match result {
            Ok(_) => {}
            Err(e) => {
                assert!(!e.is_empty());
                assert!(
                    e.contains("service manager"),
                    "Error message should mention service manager: {}",
                    e
                );
            }
        }
    }

    #[test]
    fn test_mock_service_manager_accepts_service_name() {
        let bluetooth = MockServiceManager::new(true, true);
        assert!(bluetooth.is_enabled());
    }

    #[test]
    fn test_mock_service_manager_is_installed_defaults_to_true() {
        let bluetooth = MockServiceManager::new(true, true);
        assert!(bluetooth.is_installed());
    }

    #[test]
    fn test_mock_service_manager_with_installed_configures_state() {
        let not_installed = MockServiceManager::new(true, true).with_installed(false);
        assert!(!not_installed.is_installed());

        let installed = MockServiceManager::new(true, true).with_installed(true);
        assert!(installed.is_installed());
    }

    #[test]
    fn test_no_op_service_manager_reports_not_installed() {
        let no_op = NoOpServiceManager;

        // When no service manager is detected at runtime, the fallback must report
        // not-installed so DBusServiceUnknown falls through to the unknown-service path
        // instead of showing a non-functional toggle.
        assert!(!no_op.is_installed());
    }

    #[test]
    fn test_parse_rc_update_output_exact_service_name() {
        // Must use exact matching, not prefix matching — "bluetoothd" must not match "bluetooth".
        let output = "  bluetoothd | default\n";
        let found = is_service_in_runlevel_output(output, "bluetooth");
        assert!(!found);
    }

    #[test]
    fn test_parse_rc_update_output_matches_exact_name() {
        let output = "  bluetooth | default\n";
        let found = is_service_in_runlevel_output(output, "bluetooth");
        assert!(found);
    }

    #[test]
    fn test_parse_rc_update_output_not_found() {
        let output = "  dbus | default\n  network | sysinit\n";
        let found = is_service_in_runlevel_output(output, "bluetooth");
        assert!(!found);
    }

    #[test]
    fn test_log_command_result_logs_all_branches() {
        let _guard = tracing_subscriber::fmt().with_test_writer().set_default();

        let err_result: Result<std::process::ExitStatus, std::io::Error> = Err(
            std::io::Error::new(std::io::ErrorKind::NotFound, "command not found"),
        );
        log_command_result(&err_result, "test command", "test-service");

        let fail_result = std::process::Command::new("false").status();
        log_command_result(&fail_result, "test false", "test-service");

        let ok_result = std::process::Command::new("true").status();
        log_command_result(&ok_result, "test true", "test-service");
    }

    #[test]
    fn test_parse_rc_update_output_multi_line() {
        let output = "  dbus | default\n  bluetooth | default\n  network | sysinit\n";
        let found = is_service_in_runlevel_output(output, "bluetooth");
        assert!(found);
    }
}
