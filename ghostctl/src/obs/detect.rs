//! Desktop-environment detection used to assist OBS screencapture setup.
//!
//! On Wayland, screen capture is not native to OBS; it relies on the
//! `xdg-desktop-portal` ScreenCast interface backed by PipeWire, and the
//! correct portal *backend* depends on the running compositor. These helpers
//! figure out the session type and compositor so the assist flows can install
//! and enable the right pieces.

/// Display-server session type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SessionType {
    Wayland,
    X11,
    Unknown,
}

impl SessionType {
    pub fn label(self) -> &'static str {
        match self {
            SessionType::Wayland => "Wayland",
            SessionType::X11 => "X11",
            SessionType::Unknown => "unknown",
        }
    }
}

/// Detect the session type from the environment.
pub fn session_type() -> SessionType {
    match std::env::var("XDG_SESSION_TYPE")
        .unwrap_or_default()
        .to_lowercase()
        .as_str()
    {
        "wayland" => SessionType::Wayland,
        "x11" => SessionType::X11,
        _ => {
            if std::env::var("WAYLAND_DISPLAY").is_ok() {
                SessionType::Wayland
            } else if std::env::var("DISPLAY").is_ok() {
                SessionType::X11
            } else {
                SessionType::Unknown
            }
        }
    }
}

/// Detected compositor / desktop family.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Compositor {
    Gnome,
    Kde,
    Hyprland,
    Sway,
    Wlroots,
    Other,
}

impl Compositor {
    pub fn label(self) -> &'static str {
        match self {
            Compositor::Gnome => "GNOME",
            Compositor::Kde => "KDE Plasma",
            Compositor::Hyprland => "Hyprland",
            Compositor::Sway => "Sway",
            Compositor::Wlroots => "wlroots-based",
            Compositor::Other => "other/unknown",
        }
    }
}

/// Detect the compositor from the environment.
pub fn detect_compositor() -> Compositor {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let session = std::env::var("XDG_SESSION_DESKTOP").unwrap_or_default();
    let hyprland = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok();
    let sway = std::env::var("SWAYSOCK").is_ok();
    compositor_from_env(&desktop, &session, hyprland, sway)
}

/// Pure mapping from environment hints to a compositor (unit-testable).
pub fn compositor_from_env(desktop: &str, session: &str, hyprland: bool, sway: bool) -> Compositor {
    let hay = format!("{desktop}:{session}").to_lowercase();
    if hyprland || hay.contains("hyprland") {
        return Compositor::Hyprland;
    }
    if sway || hay.contains("sway") {
        return Compositor::Sway;
    }
    if hay.contains("gnome") {
        return Compositor::Gnome;
    }
    if hay.contains("kde") || hay.contains("plasma") {
        return Compositor::Kde;
    }
    if hay.contains("wlroots")
        || hay.contains("river")
        || hay.contains("wayfire")
        || hay.contains("hikari")
        || hay.contains("labwc")
        || hay.contains("niri")
    {
        return Compositor::Wlroots;
    }
    Compositor::Other
}

/// The xdg-desktop-portal backend package that matches a compositor.
pub fn portal_backend_package(c: Compositor) -> &'static str {
    match c {
        Compositor::Gnome => "xdg-desktop-portal-gnome",
        Compositor::Kde => "xdg-desktop-portal-kde",
        Compositor::Hyprland => "xdg-desktop-portal-hyprland",
        Compositor::Sway | Compositor::Wlroots => "xdg-desktop-portal-wlr",
        Compositor::Other => "xdg-desktop-portal-gtk",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compositor_from_env() {
        assert_eq!(
            compositor_from_env("Hyprland", "", false, false),
            Compositor::Hyprland
        );
        assert_eq!(
            compositor_from_env("", "", true, false),
            Compositor::Hyprland
        );
        assert_eq!(
            compositor_from_env("sway", "", false, false),
            Compositor::Sway
        );
        assert_eq!(compositor_from_env("", "", false, true), Compositor::Sway);
        assert_eq!(
            compositor_from_env("GNOME", "gnome", false, false),
            Compositor::Gnome
        );
        assert_eq!(
            compositor_from_env("KDE", "plasma", false, false),
            Compositor::Kde
        );
        assert_eq!(
            compositor_from_env("river", "", false, false),
            Compositor::Wlroots
        );
        assert_eq!(
            compositor_from_env("XFCE", "xfce", false, false),
            Compositor::Other
        );
    }

    #[test]
    fn test_portal_backend_package() {
        assert_eq!(
            portal_backend_package(Compositor::Hyprland),
            "xdg-desktop-portal-hyprland"
        );
        assert_eq!(
            portal_backend_package(Compositor::Sway),
            "xdg-desktop-portal-wlr"
        );
        assert_eq!(
            portal_backend_package(Compositor::Gnome),
            "xdg-desktop-portal-gnome"
        );
        assert_eq!(
            portal_backend_package(Compositor::Kde),
            "xdg-desktop-portal-kde"
        );
        assert_eq!(
            portal_backend_package(Compositor::Other),
            "xdg-desktop-portal-gtk"
        );
    }
}
