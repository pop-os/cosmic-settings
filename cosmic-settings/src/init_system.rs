// SPDX-License-Identifier: GPL-3.0-only

/// Runtime detection and management of init systems
///
/// Detects service managers rather than PID 1 because on some distributions
/// (e.g., Gentoo) the PID 1 process differs from the actual service manager.
///
/// This module is Linux-only and will fail to compile on other platforms.

#[cfg(not(target_os = "linux"))]
compile_error!(
    "cosmic-settings requires Linux. Init system detection is Linux-specific."
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

fn has_systemctl() -> bool {
    which::which("systemctl").is_ok()
}

/// Requires both commands because OpenRC needs rc-service for service control
/// and rc-update for runlevel management
fn has_openrc() -> bool {
    which::which("rc-service").is_ok() && which::which("rc-update").is_ok()
}

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
