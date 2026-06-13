# OBS, Wayland Screencapture & NVENC

Wayland has no legacy X11 screen grabbing, so screen capture goes through
`xdg-desktop-portal` + PipeWire. GhostCTL detects your session and compositor,
installs the right portal backend, sets up the OBS virtual camera via
`v4l2loopback`, and verifies NVIDIA hardware encoding (NVENC). It assists with
setup, not just diagnosis.

## Quick Commands

```bash
ghostctl obs doctor              # Full report: session, portal, PipeWire, vcam, NVENC
ghostctl obs portal check        # Inspect xdg-desktop-portal + PipeWire status
ghostctl obs portal setup        # Install + enable the correct portal backend
ghostctl obs vcam status         # Show v4l2loopback / virtual-camera state
ghostctl obs vcam enable         # Load v4l2loopback with OBS-friendly options
ghostctl obs vcam enable --persist   # Also load the virtual camera on every boot
ghostctl obs vcam disable        # Unload the virtual camera module
ghostctl obs nvenc check         # Verify NVIDIA NVENC availability
ghostctl obs screencast test     # Confirm the Wayland ScreenCast portal works
```

## Features

- Session (Wayland/X11) and compositor detection
- Automatic xdg-desktop-portal backend selection per compositor:
  GNOME, KDE, Hyprland, Sway/wlroots, with a GTK fallback
- Portal + PipeWire/WirePlumber install and enable
- OBS virtual camera setup via `v4l2loopback` (optionally persistent)
- NVIDIA NVENC capability checks (driver, `nvidia-smi`, ffmpeg encoders)
- Wayland ScreenCast portal verification

## Configuration

Settings live under `[obs]` in `config.toml` (`~/.config/ghostctl/config.toml`):
the virtual-camera label, its `/dev/videoN` number, and an optional explicit
portal backend (otherwise auto-detected). Run `ghostctl config show` to see
resolved values.

## Notes

- The virtual camera requires the `v4l2loopback` kernel module (DKMS).
- `--persist` writes a modules-load configuration so the camera survives reboots.
- NVENC requires the proprietary NVIDIA driver and an ffmpeg build with NVENC
  encoders enabled.
