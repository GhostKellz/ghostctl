# ghostctl

[![Tailscale](https://img.shields.io/badge/Tailscale-enabled-blue?logo=tailscale)](https://tailscale.com)
[![Headscale](https://img.shields.io/badge/Headscale-compatible-brightgreen)](https://headscale.net)
[![WireGuard](https://img.shields.io/badge/WireGuard-powered-critical?logo=wireguard)](https://www.wireguard.com/)
[![CLI Tool](https://img.shields.io/badge/CLI-Go%20+Cobra-orange?logo=go)](https://github.com/spf13/cobra)

> ⚙️ A powerful CLI tool for managing Tailscale and Headscale networks, automating ACLs, devices, routes, and more.

---

## 👻 ✨ What is ghostctl?

`ghostctl` is a modern, opinionated command-line interface written in Go (using Cobra) that simplifies management of Tailscale and Headscale environments. Whether you’re setting up subnet routes, managing user devices, or configuring ACLs, `ghostctl` wraps common workflows in clean, scriptable commands.

---

## 🔧 Features

- 🖧 **Tailscale & Headscale Integration** – seamlessly interact with both cloud and self-hosted mesh networks.
- 🛡️ **ACL Management** – automate and version your access controls with confidence.
- 🚪 **Subnet Routing** – easily register and manage exit nodes or subnet routes.
- 📦 **Modular CLI Design** – built using Go + Cobra for clean subcommands and extensibility.
- 🔐 **WireGuard Support** – built on top of secure WireGuard tunnels.

---

## 📦 Installation

> **Coming soon...** Releases with precompiled binaries.

For now, clone and build manually:

```bash
git clone https://github.com/GhostKellz/ghostctl.git
cd ghostctl
go build -o ghostctl
./ghostctl --help
