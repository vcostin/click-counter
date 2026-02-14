// Module declarations
pub mod app;
pub mod core;
pub mod platform;

use app::App;

/// Run the Tauri application with cross-platform support
pub fn run() {
    // Create and initialize the application
    let app = App::new();

    // Initialize platform-specific features
    if let Err(e) = app.initialize() {
        eprintln!("Failed to initialize platform: {}", e);
        std::process::exit(1);
    }

    // Run the Tauri application
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
