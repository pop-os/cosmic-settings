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

/// Detects init system from the PID 1 executable path
/// This is exposed for testing purposes
/// 
/// Only returns init systems that are enabled via cargo features.
fn detect_from_pid1_exe(exe_path: &str) -> Option<InitSystem> {
    let exe_lower = exe_path.to_lowercase();
    
    #[cfg(feature = "systemd")]
    if exe_lower.contains("systemd") {
        return Some(InitSystem::Systemd);
    }
    
    #[cfg(feature = "openrc")]
    if exe_lower.contains("openrc") {
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

/// Returns the command and arguments to start a service for the given init system
pub fn get_service_start_command(init: InitSystem, service: &str) -> (&'static str, Vec<&str>) {
    match init {
        InitSystem::Systemd => ("systemctl", vec!["start", service]),
        InitSystem::OpenRC => ("rc-service", vec![service, "start"]),
        InitSystem::Unsupported => ("", vec![]),
    }
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
    fn test_detect_from_pid1_exe_systemd() {
        // Test detection when /proc/1/exe points to systemd
        let result = detect_from_pid1_exe("/usr/lib/systemd/systemd");
        assert_eq!(result, Some(InitSystem::Systemd));
    }

    #[test]
    fn test_detect_from_pid1_exe_systemd_sbin() {
        // Test detection when systemd is in /sbin
        let result = detect_from_pid1_exe("/sbin/init -> /lib/systemd/systemd");
        assert_eq!(result, Some(InitSystem::Systemd));
    }

    #[test]
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
    fn test_get_service_start_command_systemd() {
        // Test that systemd returns correct service start command
        let (program, args) = get_service_start_command(InitSystem::Systemd, "bluetooth");
        assert_eq!(program, "systemctl");
        assert_eq!(args, vec!["start", "bluetooth"]);
    }

    #[test]
    fn test_get_service_start_command_openrc() {
        // Test that OpenRC returns correct service start command
        let (program, args) = get_service_start_command(InitSystem::OpenRC, "bluetooth");
        assert_eq!(program, "rc-service");
        assert_eq!(args, vec!["bluetooth", "start"]);
    }

    #[test]
    fn test_get_service_start_command_unsupported() {
        // Test that Unsupported returns empty command (should not be called in practice)
        let (program, args) = get_service_start_command(InitSystem::Unsupported, "bluetooth");
        assert_eq!(program, "");
        assert!(args.is_empty());
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
}
