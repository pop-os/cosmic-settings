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
/// Implementations of this trait provide methods to query service state
/// and perform operations like enabling or activating services.
pub trait ServiceManager {
    /// Check if a service is enabled (will start on boot).
    fn is_enabled(&self, service: &str) -> bool;
    
    /// Check if a service is currently active (running).
    fn is_active(&self, service: &str) -> bool;
    
    /// Activate (start) a service.
    fn activate(&self, service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>>;
    
    /// Enable a service (configure it to start on boot and start it now).
    fn enable(&self, service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

/// Mock implementation of ServiceManager for testing.
///
/// This implementation returns configurable boolean values and provides
/// no-op implementations of activate and enable operations.
#[cfg(test)]
pub struct MockServiceManager {
    enabled: bool,
    active: bool,
}

#[cfg(test)]
impl MockServiceManager {
    /// Create a new MockServiceManager with specified enabled and active states.
    pub fn new(enabled: bool, active: bool) -> Self {
        Self { enabled, active }
    }
}

#[cfg(test)]
impl ServiceManager for MockServiceManager {
    fn is_enabled(&self, _service: &str) -> bool {
        self.enabled
    }

    fn is_active(&self, _service: &str) -> bool {
        self.active
    }

    fn activate(&self, _service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {})
    }

    fn enable(&self, _service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async {})
    }
}

/// SystemD implementation of ServiceManager.
///
/// This implementation delegates to systemd commands for managing services.
#[cfg(feature = "systemd")]
pub struct SystemDServiceManager;

#[cfg(feature = "systemd")]
impl SystemDServiceManager {
    /// Create a new SystemDServiceManager.
    pub fn new() -> Self {
        Self
    }
}

#[cfg(feature = "systemd")]
impl Default for SystemDServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "systemd")]
impl ServiceManager for SystemDServiceManager {
    fn is_enabled(&self, service: &str) -> bool {
        std::process::Command::new("systemctl")
            .args(["is-enabled", service])
            .status()
            .map(|status| status.success())
            .unwrap_or(true)
    }

    fn is_active(&self, service: &str) -> bool {
        std::process::Command::new("systemctl")
            .args(["is-active", service])
            .status()
            .map(|status| status.success())
            .unwrap_or(true)
    }

    fn activate(&self, service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let service = service.to_string();
        Box::pin(async move {
            let _ = tokio::process::Command::new("pkexec")
                .args(["systemctl", "start", &service])
                .status()
                .await;
        })
    }

    fn enable(&self, service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let service = service.to_string();
        Box::pin(async move {
            let _ = tokio::process::Command::new("pkexec")
                .args(["systemctl", "enable", "--now", &service])
                .status()
                .await;
        })
    }
}

/// OpenRC implementation of ServiceManager.
///
/// This implementation delegates to OpenRC commands for managing services.
#[cfg(feature = "openrc")]
pub struct OpenRcServiceManager;

#[cfg(feature = "openrc")]
impl OpenRcServiceManager {
    /// Create a new OpenRcServiceManager.
    pub fn new() -> Self {
        Self
    }
}

#[cfg(feature = "openrc")]
impl Default for OpenRcServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "openrc")]
impl ServiceManager for OpenRcServiceManager {
    fn is_enabled(&self, service: &str) -> bool {
        // Check if service is in a runlevel (enabled to start on boot)
        std::process::Command::new("rc-update")
            .args(["show"])
            .output()
            .map(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .any(|line| line.trim().starts_with(service))
            })
            .unwrap_or(true)
    }

    fn is_active(&self, service: &str) -> bool {
        // Check if service is currently running
        std::process::Command::new("rc-service")
            .args([service, "status"])
            .status()
            .map(|status| status.success())
            .unwrap_or(true)
    }

    fn activate(&self, service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let service = service.to_string();
        Box::pin(async move {
            let _ = tokio::process::Command::new("pkexec")
                .args(["rc-service", &service, "start"])
                .status()
                .await;
        })
    }

    fn enable(&self, service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let service = service.to_string();
        Box::pin(async move {
            // Add to default runlevel
            let _ = tokio::process::Command::new("pkexec")
                .args(["rc-update", "add", &service, "default"])
                .status()
                .await;
            // Start the service now
            let _ = tokio::process::Command::new("pkexec")
                .args(["rc-service", &service, "start"])
                .status()
                .await;
        })
    }
}

/// Creates the appropriate service manager based on the build configuration.
///
/// Priority order:
/// 1. In test mode: Returns MockServiceManager
/// 2. If openrc feature is enabled: Returns OpenRcServiceManager
/// 3. If systemd feature is enabled (default): Returns SystemDServiceManager
#[cfg(test)]
pub fn create_default_service_manager() -> Box<dyn ServiceManager> {
    Box::new(MockServiceManager::new(false, false))
}

#[cfg(all(not(test), feature = "openrc"))]
pub fn create_default_service_manager() -> Box<dyn ServiceManager> {
    Box::new(OpenRcServiceManager::new())
}

#[cfg(all(not(test), not(feature = "openrc"), feature = "systemd"))]
pub fn create_default_service_manager() -> Box<dyn ServiceManager> {
    Box::new(SystemDServiceManager::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_manager_trait_is_enabled() {
        // Arrange: Create a mock service manager
        struct TestServiceManager {
            enabled: bool,
        }
        
        impl ServiceManager for TestServiceManager {
            fn is_enabled(&self, _service: &str) -> bool {
                self.enabled
            }
            
            fn is_active(&self, _service: &str) -> bool {
                true
            }

            fn activate(&self, _service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>> {
                Box::pin(async {})
            }

            fn enable(&self, _service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>> {
                Box::pin(async {})
            }
        }
        
        let manager = TestServiceManager { enabled: true };
        
        // Act & Assert
        assert!(manager.is_enabled("bluetooth"));
        
        let disabled_manager = TestServiceManager { enabled: false };
        assert!(!disabled_manager.is_enabled("bluetooth"));
    }

    #[test]
    fn test_service_manager_trait_is_active() {
        // Arrange: Create a mock service manager
        struct TestServiceManager {
            active: bool,
        }
        
        impl ServiceManager for TestServiceManager {
            fn is_enabled(&self, _service: &str) -> bool {
                true
            }
            
            fn is_active(&self, _service: &str) -> bool {
                self.active
            }

            fn activate(&self, _service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>> {
                Box::pin(async {})
            }

            fn enable(&self, _service: &str) -> Pin<Box<dyn Future<Output = ()> + Send>> {
                Box::pin(async {})
            }
        }
        
        let manager = TestServiceManager { active: true };
        
        // Act & Assert
        assert!(manager.is_active("bluetooth"));
        
        let inactive_manager = TestServiceManager { active: false };
        assert!(!inactive_manager.is_active("bluetooth"));
    }

    #[test]
    fn test_mock_service_manager_returns_configured_values() {
        // Arrange: Create a MockServiceManager with known state
        let mock = MockServiceManager::new(true, true);
        
        // Act & Assert: Verify it returns the configured values
        assert!(mock.is_enabled("bluetooth"));
        assert!(mock.is_active("bluetooth"));
        
        // Arrange: Create a MockServiceManager with different state
        let mock_disabled = MockServiceManager::new(false, false);
        
        // Act & Assert: Verify it returns the configured values
        assert!(!mock_disabled.is_enabled("bluetooth"));
        assert!(!mock_disabled.is_active("bluetooth"));
    }

    #[test]
    #[cfg(feature = "systemd")]
    fn test_systemd_service_manager_implements_trait() {
        // Arrange: Create a SystemDServiceManager
        let manager = SystemDServiceManager::new();
        
        // Act & Assert: Verify it implements ServiceManager trait
        // Note: We can't test the actual systemd calls in unit tests,
        // but we can verify the struct exists and implements the trait
        let _enabled: bool = manager.is_enabled("bluetooth");
        let _active: bool = manager.is_active("bluetooth");
    }

    #[tokio::test]
    async fn test_service_manager_trait_has_activate_method() {
        // Arrange: Create a mock service manager
        let mock = MockServiceManager::new(false, false);
        
        // Act: Call activate on the service manager
        mock.activate("bluetooth").await;
        
        // Assert: The method should exist and be callable
    }

    #[tokio::test]
    async fn test_service_manager_trait_has_enable_method() {
        // Arrange: Create a mock service manager
        let mock = MockServiceManager::new(false, false);
        
        // Act: Call enable on the service manager
        mock.enable("bluetooth").await;
        
        // Assert: The method should exist and be callable
    }

    #[tokio::test]
    async fn test_service_manager_activate_returns_future() {
        // Arrange: Create a mock service manager
        let mock = MockServiceManager::new(false, false);
        
        // Act: Call activate and await the future
        let future = mock.activate("bluetooth");
        future.await;
        
        // Assert: The method should return a Future that can be awaited
    }

    #[tokio::test]
    async fn test_service_manager_enable_returns_future() {
        // Arrange: Create a mock service manager
        let mock = MockServiceManager::new(false, false);
        
        // Act: Call enable and await the future
        let future = mock.enable("bluetooth");
        future.await;
        
        // Assert: The method should return a Future that can be awaited
    }

    #[test]
    fn test_default_service_manager_uses_mock_in_tests() {
        // Arrange & Act: Create a service manager using the factory function
        let manager = create_default_service_manager();
        
        // Assert: The service manager should be a MockServiceManager
        // We verify this by checking that it returns the mock's default values (false, false)
        assert!(!manager.is_enabled("bluetooth"), 
            "Default service manager in test mode should use MockServiceManager which returns false for is_enabled");
        assert!(!manager.is_active("bluetooth"),
            "Default service manager in test mode should use MockServiceManager which returns false for is_active");
    }

    #[test]
    #[cfg(feature = "openrc")]
    fn test_openrc_service_manager_implements_trait() {
        // Arrange: Create an OpenRcServiceManager
        let manager = OpenRcServiceManager::new();
        
        // Act & Assert: Verify it implements ServiceManager trait
        // Note: We can't test the actual OpenRC calls in unit tests,
        // but we can verify the struct exists and implements the trait
        let _enabled: bool = manager.is_enabled("bluetooth");
        let _active: bool = manager.is_active("bluetooth");
    }
}
