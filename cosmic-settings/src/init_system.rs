// SPDX-License-Identifier: GPL-3.0-only

/// Runtime detection and management of init systems
/// 
/// This module provides runtime detection of the init system (systemd, OpenRC, etc.)
/// and abstractions for managing services across different init systems.
/// 
/// This module is Linux-only and will fail to compile on other platforms.

#[cfg(not(target_os = "linux"))]
compile_error!("cosmic-settings requires Linux. Init system detection relies on /proc, which is Linux-specific.");

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

/// Detects init system from the PID 1 executable path
/// This is exposed for testing purposes
/// 
/// Only returns init systems that are enabled via cargo features.
/// Uses strict matching on the basename of the executable to avoid false positives.
fn detect_from_pid1_exe(exe_path: &str) -> Option<InitSystem> {
    use std::path::Path;
    
    // Extract the basename (file name) from the path
    let basename = Path::new(exe_path)
        .file_name()
        .and_then(|s| s.to_str())?;
    
    #[cfg(feature = "systemd")]
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
    
    // Check /proc/1/exe to see what's actually running as PID 1
    if let Ok(pid1_exe) = fs::read_link("/proc/1/exe") {
        if let Some(exe_str) = pid1_exe.to_str() {
            if let Some(init_system) = detect_from_pid1_exe(exe_str) {
                return init_system;
            }
        }
    }
    
    // If we can't read /proc/1/exe or don't recognize the init system
    InitSystem::Unsupported
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_init_system_returns_valid_variant() {
        // This test verifies that detect_init_system returns one of the valid InitSystem variants
        let init = detect_init_system();
        
        assert!(
            matches!(init, InitSystem::Systemd | InitSystem::OpenRC | InitSystem::Unsupported),
            "detect_init_system must return a valid InitSystem variant"
        );
    }

    #[test]
    #[cfg(feature = "systemd")]
    fn test_detect_from_pid1_exe_systemd() {
        // Test detection when /proc/1/exe points to systemd
        let result = detect_from_pid1_exe("/usr/lib/systemd/systemd");
        assert_eq!(result, Some(InitSystem::Systemd));
    }

    #[test]
    #[cfg(feature = "systemd")]
    fn test_detect_from_pid1_exe_systemd_sbin() {
        // Test detection when systemd is in /sbin
        let result = detect_from_pid1_exe("/sbin/init -> /lib/systemd/systemd");
        assert_eq!(result, Some(InitSystem::Systemd));
    }

    #[test]
    #[cfg(feature = "openrc")]
    fn test_detect_from_pid1_exe_openrc() {
        // Test detection when /proc/1/exe points to openrc-init
        let result = detect_from_pid1_exe("/sbin/openrc-init");
        assert_eq!(result, Some(InitSystem::OpenRC));
    }

    #[test]
    fn test_detect_from_pid1_exe_unknown() {
        // Test detection when PID 1 is something else (e.g., runit, s6, etc.)
        let result = detect_from_pid1_exe("/sbin/runit");
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_from_pid1_exe_false_positive_systemd_in_path() {
        // Should NOT match if "systemd" appears in a directory but not as the executable
        let result = detect_from_pid1_exe("/opt/systemd-tools/bin/myinit");
        assert_eq!(result, None, "should not match systemd in directory name");
    }

    #[test]
    fn test_detect_from_pid1_exe_false_positive_openrc_in_path() {
        // Should NOT match if "openrc" appears in a directory but not as the executable
        let result = detect_from_pid1_exe("/usr/openrc-backups/bin/init");
        assert_eq!(result, None, "should not match openrc in directory name");
    }

    #[test]
    #[cfg(feature = "systemd")]
    fn test_detect_from_pid1_exe_systemd_exact_match() {
        // Should match exact systemd binary names
        assert_eq!(detect_from_pid1_exe("/usr/lib/systemd/systemd"), Some(InitSystem::Systemd));
        assert_eq!(detect_from_pid1_exe("/lib/systemd/systemd"), Some(InitSystem::Systemd));
        assert_eq!(detect_from_pid1_exe("/sbin/systemd"), Some(InitSystem::Systemd));
    }

    #[test]
    #[cfg(feature = "openrc")]
    fn test_detect_from_pid1_exe_openrc_exact_match() {
        // Should match exact OpenRC binary names
        assert_eq!(detect_from_pid1_exe("/sbin/openrc-init"), Some(InitSystem::OpenRC));
        assert_eq!(detect_from_pid1_exe("/usr/sbin/openrc-init"), Some(InitSystem::OpenRC));
    }

    #[test]
    #[cfg(feature = "systemd")]
    fn test_detect_from_pid1_exe_systemd_when_feature_enabled() {
        // When systemd feature is enabled, systemd should be detected
        let result = detect_from_pid1_exe("/usr/lib/systemd/systemd");
        assert_eq!(result, Some(InitSystem::Systemd));
    }

    #[test]
    #[cfg(feature = "openrc")]
    fn test_detect_from_pid1_exe_openrc_when_feature_enabled() {
        // When openrc feature is enabled, OpenRC should be detected
        let result = detect_from_pid1_exe("/sbin/openrc-init");
        assert_eq!(result, Some(InitSystem::OpenRC));
    }

    #[test]
    #[cfg(all(not(feature = "systemd"), feature = "openrc"))]
    fn test_detect_from_pid1_exe_systemd_when_feature_disabled() {
        // When systemd feature is NOT enabled, systemd should NOT be detected
        let result = detect_from_pid1_exe("/usr/lib/systemd/systemd");
        assert_eq!(result, None, "systemd should not be detected when feature is disabled");
    }

    #[test]
    #[cfg(all(feature = "systemd", not(feature = "openrc")))]
    fn test_detect_from_pid1_exe_openrc_when_feature_disabled() {
        // When openrc feature is NOT enabled, OpenRC should NOT be detected
        let result = detect_from_pid1_exe("/sbin/openrc-init");
        assert_eq!(result, None, "openrc should not be detected when feature is disabled");
    }

    #[test]
    #[cfg(all(not(feature = "systemd"), not(feature = "openrc")))]
    fn test_detect_from_pid1_exe_no_features_enabled() {
        // When no init system features are enabled, nothing should be detected
        let systemd_result = detect_from_pid1_exe("/usr/lib/systemd/systemd");
        assert_eq!(systemd_result, None, "systemd should not be detected when no features enabled");
        
        let openrc_result = detect_from_pid1_exe("/sbin/openrc-init");
        assert_eq!(openrc_result, None, "openrc should not be detected when no features enabled");
    }
}
