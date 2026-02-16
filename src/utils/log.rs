//! Simple logging utilities for the CLI 3D object viewer.

use color_eyre::owo_colors::OwoColorize;

// ===== Definition ============================================================

/// Logs a debug message to the console with a consistent format and color.
pub fn dbg(message: &str) {
    println!("{}", format!("[DBG] {message}").bright_black());
}

/// Logs an error message to the console with a consistent format and color.
pub fn err(message: &str) {
    println!("{}", format!("[ERR] {message}").bright_red());
}
