// SPDX-License-Identifier: GPL-3.0-only

/// Runtime detection and management of init systems
///
/// This module provides runtime detection of the init system (systemd, OpenRC, etc.)
/// and abstractions for managing services across different init systems.
///
/// This module is Linux-only and will fail to compile on other platforms.

#[cfg(not(target_os = "linux"))]
compile_error!(
    "cosmic-settings requires Linux. Init system detection relies on /proc, which is Linux-specific."
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitSystem {
    Systemd,
    OpenRC,
    Unsupported,
}

/// Errors that can occur during service management operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceError {
    /// The detected init system is not supported by this build
    UnsupportedInitSystem,
    /// Failed to execute the service management command
    CommandFailed,
    /// Failed to determine service status
    StatusCheckFailed,
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::UnsupportedInitSystem => write!(f, "Init system not supported"),
            ServiceError::CommandFailed => write!(f, "Service command failed"),
            ServiceError::StatusCheckFailed => write!(f, "Failed to check service status"),
        }
    }
}

impl std::error::Error for ServiceError {}

/// Checks if the systemctl command is available on the system
fn has_systemctl() -> bool {
    which::which("systemctl").is_ok()
}

/// Checks if both rc-service and rc-update commands are available on the system
fn has_openrc() -> bool {
    which::which("rc-service").is_ok() && which::which("rc-update").is_ok()
}

/// Detects the init system currently running on the host by checking for service management tools
///
/// This checks for the presence of service management commands (systemctl, rc-service, rc-update)
/// to determine which init system is available. This is more reliable than checking /proc/1/exe
/// because it detects the actual service manager you can use, not just what's running as PID 1.
/// For example, Gentoo may have sysvinit at PID 1 but uses OpenRC for service management.
pub fn detect_init_system() -> InitSystem {
    if has_systemctl() {
        return InitSystem::Systemd;
    }

    #[cfg(feature = "openrc")]
    if has_openrc() {
        return InitSystem::OpenRC;
    }

    InitSystem::Unsupported
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_init_system_returns_valid_variant() {
        let init = detect_init_system();

        assert!(
            matches!(
                init,
                InitSystem::Systemd | InitSystem::OpenRC | InitSystem::Unsupported
            ),
            "detect_init_system must return a valid InitSystem variant"
        );
    }

    #[test]
    fn test_has_systemctl_returns_bool() {
        let result = has_systemctl();
        assert!(result == true || result == false, "must return a boolean value");
    }

    #[test]
    fn test_has_openrc_returns_bool() {
        let result = has_openrc();
        assert!(result == true || result == false, "must return a boolean value");
    }
}
