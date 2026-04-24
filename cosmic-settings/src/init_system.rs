/// Runtime detection and management of init systems
/// 
/// This module provides runtime detection of the init system (systemd, OpenRC, etc.)
/// and abstractions for managing services across different init systems.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitSystem {
    Systemd,
    OpenRC,
    Unsupported,
}

/// Detects init system from the PID 1 executable path
/// This is exposed for testing purposes
fn detect_from_pid1_exe(exe_path: &str) -> Option<InitSystem> {
    let exe_lower = exe_path.to_lowercase();
    
    if exe_lower.contains("systemd") {
        Some(InitSystem::Systemd)
    } else if exe_lower.contains("openrc") {
        Some(InitSystem::OpenRC)
    } else {
        None
    }
}

/// Detects the init system currently running on the host
pub fn detect_init_system() -> InitSystem {
    use std::fs;
    use std::path::Path;
    
    // Method 1: Check /proc/1/exe to see what's actually running as PID 1
    // This is the most reliable method
    if let Ok(pid1_exe) = fs::read_link("/proc/1/exe") {
        if let Some(exe_str) = pid1_exe.to_str() {
            if let Some(init_system) = detect_from_pid1_exe(exe_str) {
                return init_system;
            }
        }
    }
    
    // Method 2: Fall back to checking for characteristic runtime directories
    // This handles cases where /proc/1/exe can't be read or is ambiguous
    
    // Check for systemd - look for /run/systemd/system directory
    if Path::new("/run/systemd/system").exists() {
        return InitSystem::Systemd;
    }
    
    // Check for OpenRC - look for /run/openrc directory
    if Path::new("/run/openrc").exists() {
        return InitSystem::OpenRC;
    }
    
    // No supported init system detected
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
}
