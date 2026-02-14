/// Platform-specific module declarations
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

use std::fmt;

/// Platform trait that all platform implementations must implement
/// This provides a common interface for platform-specific functionality
pub trait Platform: fmt::Debug {
    /// Returns the human-readable name of the platform
    fn name(&self) -> &str;

    /// Initialize platform-specific features
    /// Called during application startup
    fn init(&self) -> Result<(), String>;

    /// Get platform-specific configuration or settings
    /// Returns a description of the platform's capabilities
    fn get_info(&self) -> String {
        format!("Platform: {}", self.name())
    }
}

/// Get the current platform implementation
/// This function uses conditional compilation to return the appropriate platform
pub fn get_current_platform() -> Box<dyn Platform> {
    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsPlatform::new())
    }

    #[cfg(target_os = "macos")]
    {
        Box::new(macos::MacOSPlatform::new())
    }

    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxPlatform::new())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        compile_error!("Unsupported platform. Only Windows, macOS, and Linux are supported.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = get_current_platform();
        assert!(!platform.name().is_empty());
    }

    #[test]
    fn test_platform_init() {
        let platform = get_current_platform();
        assert!(platform.init().is_ok());
    }
}
