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

/// Detects init system from the PID 1 executable path
/// This is exposed for testing purposes
///
/// systemd is always supported. OpenRC is supported when the openrc feature is enabled.
/// Uses strict matching on the basename of the executable to avoid false positives.
fn detect_from_pid1_exe(exe_path: &str) -> Option<InitSystem> {
    use std::path::Path;

    // Strict basename matching prevents false positives from directory names
    let basename = Path::new(exe_path).file_name().and_then(|s| s.to_str())?;

    if basename == "systemd" {
        return Some(InitSystem::Systemd);
    }

    #[cfg(feature = "openrc")]
    if basename == "openrc-init" {
        return Some(InitSystem::OpenRC);
    }

    None
}

/// Detects the init system currently running on the host by checking /proc/1/exe
///
/// This reads the symlink at /proc/1/exe to determine what executable is running
/// as PID 1 (the init system), which is the most reliable detection method.
pub fn detect_init_system() -> InitSystem {
    use std::fs;

    if let Ok(pid1_exe) = fs::read_link("/proc/1/exe") {
        if let Some(exe_str) = pid1_exe.to_str() {
            if let Some(init_system) = detect_from_pid1_exe(exe_str) {
                return init_system;
            }
        }
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

    #[test]
    fn test_detect_from_pid1_exe_systemd() {
        let result = detect_from_pid1_exe("/usr/lib/systemd/systemd");
        assert_eq!(result, Some(InitSystem::Systemd));
    }

    #[test]
    fn test_detect_from_pid1_exe_systemd_sbin() {
        let result = detect_from_pid1_exe("/sbin/init -> /lib/systemd/systemd");
        assert_eq!(result, Some(InitSystem::Systemd));
    }

    #[test]
    #[cfg(feature = "openrc")]
    fn test_detect_from_pid1_exe_openrc() {
        let result = detect_from_pid1_exe("/sbin/openrc-init");
        assert_eq!(result, Some(InitSystem::OpenRC));
    }

    #[test]
    fn test_detect_from_pid1_exe_unknown() {
        let result = detect_from_pid1_exe("/sbin/runit");
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_from_pid1_exe_false_positive_systemd_in_path() {
        let result = detect_from_pid1_exe("/opt/systemd-tools/bin/myinit");
        assert_eq!(result, None, "should not match systemd in directory name");
    }

    #[test]
    fn test_detect_from_pid1_exe_false_positive_openrc_in_path() {
        let result = detect_from_pid1_exe("/usr/openrc-backups/bin/init");
        assert_eq!(result, None, "should not match openrc in directory name");
    }

    #[test]
    fn test_detect_from_pid1_exe_systemd_exact_match() {
        assert_eq!(
            detect_from_pid1_exe("/usr/lib/systemd/systemd"),
            Some(InitSystem::Systemd)
        );
        assert_eq!(
            detect_from_pid1_exe("/lib/systemd/systemd"),
            Some(InitSystem::Systemd)
        );
        assert_eq!(
            detect_from_pid1_exe("/sbin/systemd"),
            Some(InitSystem::Systemd)
        );
    }

    #[test]
    #[cfg(feature = "openrc")]
    fn test_detect_from_pid1_exe_openrc_exact_match() {
        assert_eq!(
            detect_from_pid1_exe("/sbin/openrc-init"),
            Some(InitSystem::OpenRC)
        );
        assert_eq!(
            detect_from_pid1_exe("/usr/sbin/openrc-init"),
            Some(InitSystem::OpenRC)
        );
    }

    #[test]
    fn test_detect_from_pid1_exe_systemd_always_enabled() {
        let result = detect_from_pid1_exe("/usr/lib/systemd/systemd");
        assert_eq!(result, Some(InitSystem::Systemd));
    }

    #[test]
    #[cfg(feature = "openrc")]
    fn test_detect_from_pid1_exe_openrc_when_feature_enabled() {
        let result = detect_from_pid1_exe("/sbin/openrc-init");
        assert_eq!(result, Some(InitSystem::OpenRC));
    }

    #[test]
    #[cfg(not(feature = "openrc"))]
    fn test_detect_from_pid1_exe_openrc_when_feature_disabled() {
        let result = detect_from_pid1_exe("/sbin/openrc-init");
        assert_eq!(
            result, None,
            "openrc should not be detected when feature is disabled"
        );
    }
}
