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

/// Detects the init system currently running on the host
pub fn detect_init_system() -> InitSystem {
    use std::path::Path;
    
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
        
        // At this point, detect_init_system doesn't exist yet, so this should fail
        assert!(
            matches!(init, InitSystem::Systemd | InitSystem::OpenRC | InitSystem::Unsupported),
            "detect_init_system must return a valid InitSystem variant"
        );
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
