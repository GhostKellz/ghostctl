//! TUI Helper Module
//!
//! Provides a consistent interface for interactive prompts with support for:
//! - Headless mode (non-interactive)
//! - Auto-yes mode
//! - Dry-run mode
//! - Consistent theming

use crate::utils::{is_dry_run, is_headless, is_plain_mode};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Password, Select};
use std::fmt::Display;

/// Icon constants for plain mode
pub mod icons {
    use super::is_plain_mode;

    pub fn success() -> &'static str {
        if is_plain_mode() {
            "[OK]"
        } else {
            "✅"
        }
    }
    pub fn error() -> &'static str {
        if is_plain_mode() {
            "[ERROR]"
        } else {
            "❌"
        }
    }
    pub fn warn() -> &'static str {
        if is_plain_mode() {
            "[WARN]"
        } else {
            "⚠️"
        }
    }
    pub fn info() -> &'static str {
        if is_plain_mode() {
            "[INFO]"
        } else {
            "ℹ️"
        }
    }
    pub fn question() -> &'static str {
        if is_plain_mode() {
            "[?]"
        } else {
            "❓"
        }
    }
    pub fn key() -> &'static str {
        if is_plain_mode() {
            "[KEY]"
        } else {
            "🔑"
        }
    }
    pub fn select() -> &'static str {
        if is_plain_mode() {
            "[SELECT]"
        } else {
            "📋"
        }
    }
    pub fn input() -> &'static str {
        if is_plain_mode() {
            "[INPUT]"
        } else {
            "📝"
        }
    }
    pub fn back() -> &'static str {
        if is_plain_mode() {
            "[BACK]"
        } else {
            "⬅️"
        }
    }
    pub fn robot() -> &'static str {
        if is_plain_mode() {
            "[AUTO]"
        } else {
            "🤖"
        }
    }
    pub fn search() -> &'static str {
        if is_plain_mode() {
            "[SEARCH]"
        } else {
            "🔍"
        }
    }
    pub fn update() -> &'static str {
        if is_plain_mode() {
            "[UPDATE]"
        } else {
            "🔄"
        }
    }
    pub fn status() -> &'static str {
        if is_plain_mode() {
            "[STATUS]"
        } else {
            "📊"
        }
    }
    pub fn docker() -> &'static str {
        if is_plain_mode() {
            "[DOCKER]"
        } else {
            "🐳"
        }
    }
    pub fn tools() -> &'static str {
        if is_plain_mode() {
            "[TOOLS]"
        } else {
            "🛠️"
        }
    }
    pub fn config() -> &'static str {
        if is_plain_mode() {
            "[CONFIG]"
        } else {
            "⚙️"
        }
    }
    pub fn network() -> &'static str {
        if is_plain_mode() {
            "[NET]"
        } else {
            "🌐"
        }
    }
    pub fn send() -> &'static str {
        if is_plain_mode() {
            "[SEND]"
        } else {
            "📤"
        }
    }
    pub fn receive() -> &'static str {
        if is_plain_mode() {
            "[RECV]"
        } else {
            "📥"
        }
    }
    pub fn chat() -> &'static str {
        if is_plain_mode() {
            "[CHAT]"
        } else {
            "💬"
        }
    }
    pub fn run() -> &'static str {
        if is_plain_mode() {
            "[RUN]"
        } else {
            "🏃"
        }
    }
    pub fn lock() -> &'static str {
        if is_plain_mode() {
            "[LOCK]"
        } else {
            "🔐"
        }
    }
    pub fn ghost() -> &'static str {
        if is_plain_mode() {
            "[GHOST]"
        } else {
            "👻"
        }
    }
    pub fn home() -> &'static str {
        if is_plain_mode() {
            "[HOME]"
        } else {
            "🏠"
        }
    }
    pub fn build() -> &'static str {
        if is_plain_mode() {
            "[BUILD]"
        } else {
            "🏗️"
        }
    }
    pub fn folder() -> &'static str {
        if is_plain_mode() {
            "[DIR]"
        } else {
            "📁"
        }
    }
    pub fn rocket() -> &'static str {
        if is_plain_mode() {
            "[LAUNCH]"
        } else {
            "🚀"
        }
    }
    pub fn lightbulb() -> &'static str {
        if is_plain_mode() {
            "[TIP]"
        } else {
            "💡"
        }
    }
    pub fn disk() -> &'static str {
        if is_plain_mode() {
            "[DISK]"
        } else {
            "💾"
        }
    }
}

/// Check if auto-yes mode is enabled
pub fn is_auto_yes() -> bool {
    std::env::var("GHOSTCTL_YES").is_ok()
}

/// Standard theme for all prompts
pub fn theme() -> ColorfulTheme {
    ColorfulTheme::default()
}

// ============================================================================
// Select Menu
// ============================================================================

/// Display a select menu and return the chosen index
/// In headless mode, returns the default value
pub fn select<T: Display>(prompt: &str, items: &[T], default: usize) -> Option<usize> {
    if is_headless() {
        println!("{} {} [auto-select: {}]", icons::select(), prompt, default);
        return Some(default);
    }

    Select::with_theme(&theme())
        .with_prompt(prompt)
        .items(items)
        .default(default)
        .interact_opt()
        .unwrap_or(None)
}

/// Display a select menu and return the chosen index (required)
/// In headless mode, returns the default value
pub fn select_required<T: Display>(prompt: &str, items: &[T], default: usize) -> usize {
    if is_headless() {
        println!("{} {} [auto-select: {}]", icons::select(), prompt, default);
        return default;
    }

    Select::with_theme(&theme())
        .with_prompt(prompt)
        .items(items)
        .default(default)
        .interact()
        .unwrap_or(default)
}

/// Display a select menu with a "Back" option
/// Returns None if "Back" is selected or on cancel
pub fn select_with_back<T: Display>(prompt: &str, items: &[T], default: usize) -> Option<usize> {
    if is_headless() {
        println!("{} {} [auto-select: {}]", icons::select(), prompt, default);
        return Some(default);
    }

    let mut display_items: Vec<String> = items.iter().map(|i| i.to_string()).collect();
    let back_label = if is_plain_mode() {
        "[BACK] Back"
    } else {
        "⬅️  Back"
    };
    display_items.push(back_label.to_string());

    let choice = Select::with_theme(&theme())
        .with_prompt(prompt)
        .items(&display_items)
        .default(default)
        .interact()
        .unwrap_or(display_items.len() - 1);

    if choice == display_items.len() - 1 {
        None
    } else {
        Some(choice)
    }
}

// ============================================================================
// Multi-Select Menu
// ============================================================================

/// Display a multi-select menu and return the chosen indices
/// In headless mode, returns empty or all based on auto_select_all
pub fn multi_select<T: Display>(
    prompt: &str,
    items: &[T],
    defaults: Option<&[bool]>,
) -> Vec<usize> {
    if is_headless() {
        println!("{} {} [skipped in headless mode]", icons::select(), prompt);
        return Vec::new();
    }

    let theme = theme();
    let mut builder = MultiSelect::with_theme(&theme).with_prompt(prompt);

    for (i, item) in items.iter().enumerate() {
        let default = defaults
            .map(|d| d.get(i).copied().unwrap_or(false))
            .unwrap_or(false);
        builder = builder.item_checked(item, default);
    }

    builder.interact().unwrap_or_default()
}

// ============================================================================
// Confirmation
// ============================================================================

/// Ask for confirmation
/// In headless mode or auto-yes mode, returns the default
pub fn confirm(prompt: &str, default: bool) -> bool {
    if is_headless() || is_auto_yes() {
        println!(
            "{} {} [auto-answer: {}]",
            icons::question(),
            prompt,
            if default { "yes" } else { "no" }
        );
        return default;
    }

    if is_dry_run() {
        println!("[DRY RUN] Would ask: {} (default: {})", prompt, default);
        return default;
    }

    Confirm::with_theme(&theme())
        .with_prompt(prompt)
        .default(default)
        .interact()
        .unwrap_or(default)
}

/// Ask for confirmation with a warning (defaults to false)
pub fn confirm_dangerous(prompt: &str) -> bool {
    if is_headless() {
        println!(
            "{} {} [dangerous action skipped in headless mode]",
            icons::warn(),
            prompt
        );
        return false;
    }

    if is_auto_yes() {
        println!(
            "{} {} [auto-yes for dangerous action]",
            icons::warn(),
            prompt
        );
        return true;
    }

    let warn_prefix = if is_plain_mode() { "[!]" } else { "⚠️ " };
    Confirm::with_theme(&theme())
        .with_prompt(format!("{} {}", warn_prefix, prompt))
        .default(false)
        .interact()
        .unwrap_or(false)
}

// ============================================================================
// Text Input
// ============================================================================

/// Get text input from user
/// In headless mode, returns the default value
pub fn input(prompt: &str, default: Option<&str>) -> Option<String> {
    if is_headless() {
        let value = default.unwrap_or("");
        println!("{} {} [auto-input: {}]", icons::input(), prompt, value);
        return Some(value.to_string());
    }

    let theme = theme();
    let mut builder = Input::<String>::with_theme(&theme).with_prompt(prompt);

    if let Some(def) = default {
        builder = builder.default(def.to_string());
    }

    builder.interact_text().ok()
}

/// Get required text input from user
/// In headless mode, returns the default value or errors
pub fn input_required(prompt: &str, default: &str) -> String {
    if is_headless() {
        println!("{} {} [auto-input: {}]", icons::input(), prompt, default);
        return default.to_string();
    }

    Input::<String>::with_theme(&theme())
        .with_prompt(prompt)
        .default(default.to_string())
        .interact_text()
        .unwrap_or_else(|_| default.to_string())
}

/// Get password input (hidden)
/// In headless mode, checks for environment variable or returns error
pub fn password(prompt: &str, env_var: Option<&str>) -> Option<String> {
    // In headless mode, try to get from environment
    if is_headless() {
        if let Some(var) = env_var
            && let Ok(pass) = std::env::var(var) {
                println!("{} {} [from environment: {}]", icons::key(), prompt, var);
                return Some(pass);
            }
        println!(
            "{} {} [skipped in headless mode - set {} env var]",
            icons::key(),
            prompt,
            env_var.unwrap_or("GHOSTCTL_PASSWORD")
        );
        return None;
    }

    Password::with_theme(&theme())
        .with_prompt(prompt)
        .interact()
        .ok()
}

// ============================================================================
// Menu Loop Helper
// ============================================================================

/// Run a menu loop until "Back" is selected
/// Returns when user selects the last item (assumed to be "Back")
pub fn menu_loop<F>(title: &str, items: &[&str], handler: F)
where
    F: Fn(usize),
{
    if is_headless() {
        println!(
            "{} {} [menu skipped in headless mode]",
            icons::select(),
            title
        );
        return;
    }

    loop {
        let choice = Select::with_theme(&theme())
            .with_prompt(title)
            .items(items)
            .default(0)
            .interact()
            .unwrap_or(items.len() - 1);

        if choice == items.len() - 1 {
            break;
        }

        handler(choice);
    }
}

// ============================================================================
// Progress/Status Display
// ============================================================================

/// Print a status message with custom icon
pub fn status(icon: &str, message: &str) {
    println!("{} {}", icon, message);
}

/// Print a success message
pub fn success(message: &str) {
    println!("{} {}", icons::success(), message);
}

/// Print an error message
pub fn error(message: &str) {
    eprintln!("{} {}", icons::error(), message);
}

/// Print a warning message
pub fn warn(message: &str) {
    println!("{} {}", icons::warn(), message);
}

/// Print an info message
pub fn info(message: &str) {
    println!("{} {}", icons::info(), message);
}

/// Print a section header
pub fn header(title: &str) {
    println!("\n{}", title);
    let separator = if is_plain_mode() { "=" } else { "═" };
    println!("{}", separator.repeat(title.chars().count().max(40)));
}

/// Print a sub-header
pub fn subheader(title: &str) {
    println!("\n{}", title);
    let separator = if is_plain_mode() { "-" } else { "─" };
    println!("{}", separator.repeat(title.chars().count().max(30)));
}

// ============================================================================
// Dry-Run Helpers
// ============================================================================

/// Wrap an action with dry-run check
/// Returns true if action should proceed, false if in dry-run mode
pub fn should_execute(action_description: &str) -> bool {
    if is_dry_run() {
        println!("[DRY RUN] Would: {}", action_description);
        return false;
    }
    true
}

/// Print what would be done in dry-run mode, or execute the action
pub fn execute_or_dry_run<F>(action_description: &str, action: F)
where
    F: FnOnce(),
{
    if is_dry_run() {
        println!("[DRY RUN] Would: {}", action_description);
    } else {
        action();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_auto_yes_default() {
        unsafe { std::env::remove_var("GHOSTCTL_YES") };
        assert!(!is_auto_yes());
    }
}
