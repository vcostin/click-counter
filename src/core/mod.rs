/// Core business logic module
/// This module contains all platform-agnostic functionality
/// that is shared across all platforms.

use serde::{Deserialize, Serialize};

/// Application state that is shared across all platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// The current click count
    pub count: u32,
    /// Whether the counter is active
    pub is_active: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    /// Create a new AppState with default values
    pub fn new() -> Self {
        Self {
            count: 0,
            is_active: true,
        }
    }

    /// Increment the counter
    pub fn increment(&mut self) {
        self.count = self.count.saturating_add(1);
    }

    /// Decrement the counter
    pub fn decrement(&mut self) {
        self.count = self.count.saturating_sub(1);
    }

    /// Reset the counter to zero
    pub fn reset(&mut self) {
        self.count = 0;
    }

    /// Get the current count
    pub fn get_count(&self) -> u32 {
        self.count
    }

    /// Set the counter to a specific value
    pub fn set_count(&mut self, value: u32) {
        self.count = value;
    }

    /// Toggle the active state
    pub fn toggle_active(&mut self) {
        self.is_active = !self.is_active;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new();
        assert_eq!(state.count, 0);
        assert!(state.is_active);
    }

    #[test]
    fn test_increment() {
        let mut state = AppState::new();
        state.increment();
        assert_eq!(state.count, 1);
    }

    #[test]
    fn test_decrement() {
        let mut state = AppState::new();
        state.set_count(5);
        state.decrement();
        assert_eq!(state.count, 4);
    }

    #[test]
    fn test_reset() {
        let mut state = AppState::new();
        state.set_count(10);
        state.reset();
        assert_eq!(state.count, 0);
    }

    #[test]
    fn test_saturating_add() {
        let mut state = AppState::new();
        state.set_count(u32::MAX);
        state.increment();
        assert_eq!(state.count, u32::MAX);
    }

    #[test]
    fn test_saturating_sub() {
        let mut state = AppState::new();
        state.decrement();
        assert_eq!(state.count, 0);
    }
}
