/// Application module
/// This module contains the main application logic that uses both
/// the core business logic and platform-specific functionality.

use crate::core::AppState;
use crate::platform::{get_current_platform, Platform};

/// Main application structure that coordinates between
/// platform-specific code and core business logic
pub struct App {
    state: AppState,
    platform: Box<dyn Platform>,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
            platform: get_current_platform(),
        }
    }

    /// Initialize the application
    /// This sets up platform-specific features and prepares the app for use
    pub fn initialize(&self) -> Result<(), String> {
        println!("Initializing Click Counter application...");
        println!("{}", self.platform.get_info());

        // Initialize platform-specific features
        self.platform.init()?;

        println!("Application initialized successfully");
        Ok(())
    }

    /// Get the current platform name
    pub fn get_platform_name(&self) -> &str {
        self.platform.name()
    }

    /// Get a reference to the app state
    pub fn get_state(&self) -> &AppState {
        &self.state
    }

    /// Get a mutable reference to the app state
    pub fn get_state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    /// Example method showing how to use platform-specific features
    /// with core business logic
    pub fn on_click(&mut self) {
        // Use core business logic
        self.state.increment();

        // Platform-specific behavior could be added here
        // For example, different notification styles per platform
        println!(
            "[{}] Click registered! Count: {}",
            self.platform.name(),
            self.state.get_count()
        );
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert_eq!(app.get_state().get_count(), 0);
    }

    #[test]
    fn test_app_initialization() {
        let app = App::new();
        assert!(app.initialize().is_ok());
    }

    #[test]
    fn test_platform_detection() {
        let app = App::new();
        let platform_name = app.get_platform_name();

        // Should be one of the supported platforms
        assert!(
            platform_name == "Windows"
            || platform_name == "macOS"
            || platform_name == "Linux"
        );
    }

    #[test]
    fn test_on_click() {
        let mut app = App::new();
        app.on_click();
        assert_eq!(app.get_state().get_count(), 1);

        app.on_click();
        assert_eq!(app.get_state().get_count(), 2);
    }
}
