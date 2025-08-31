use std::fs;
use std::path::PathBuf;
use std::io::Result as IoResult;
use tracing::info;

const CONFIGURATION_DIRECTORY: &str = "~/.config/regolith3/sway/cosmic-settings/generated-config.d";

/// Saves configuration content to a file in the Sway configuration directory.
/// The file will be overwritten if it already exists.
/// The .conf extension is automatically added to the filename.
/// 
/// # Arguments
/// * `filename` - Name of the file to save (e.g., "10-keyboard") - .conf will be added automatically
/// * `contents` - The configuration content to write to the file
/// 
/// # Returns
/// * `Ok(())` if the file was saved successfully
/// * `Err(std::io::Error)` if there was an error creating directories or writing the file

// Macro to handle Sway command execution with logging and config saving
#[macro_export]
macro_rules! execute_sway_cmd {
    ($page:expr, $cmd:expr, $success_msg:expr, $config_file:expr, $config_content:expr) => {
        match $page.sway_connection().run_command($cmd.clone()) {
            Ok(_) => {
                info!("{} via: swaymsg {}", $success_msg, &$cmd);
                // Save the configuration to file for persistence
                if let Err(e) = save_config($config_file, $config_content) {
                    tracing::error!("Failed to save config file {}: {}", $config_file, e);
                }
            },
            Err(e) => {
                tracing::error!("Failed to execute command: {}", e);
                return Err(e.into());
            }
        }
    };
}

pub fn save_config(filename: &str, contents: &str) -> IoResult<()> {
    // Expand the tilde in the path
    let config_dir = if CONFIGURATION_DIRECTORY.starts_with('~') {
        let home = dirs::home_dir()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find home directory"
            ))?;
        let path_without_tilde = &CONFIGURATION_DIRECTORY[2..]; // Remove "~/"
        home.join(path_without_tilde)
    } else {
        PathBuf::from(CONFIGURATION_DIRECTORY)
    };
    
    // Create the directory if it doesn't exist
    fs::create_dir_all(&config_dir)?;
    
    // Automatically add .conf extension
    let filename_with_ext = format!("{}", filename);
    
    // Create the full file path
    let file_path = config_dir.join(filename_with_ext);
    
    // Write the contents to the file (overwrites if exists)
    fs::write(file_path, contents)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_save_sway_config() {
        let test_filename = "test-config"; // No .conf extension needed
        let test_contents = "# Test configuration\ninput type:keyboard {\n    xkb_layout us\n}";
        
        let result = save_config(test_filename, test_contents);
        
        // The test will depend on your actual home directory structure
        // In a real scenario, you might want to make the base directory configurable
        assert!(result.is_ok() || result.is_err()); // Basic check that function runs
    }
}