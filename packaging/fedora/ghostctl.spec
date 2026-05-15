Name:           ghostctl
Version:        0.10.0
Release:        1%{?dist}
Summary:        Universal system administration toolkit

License:        MIT
URL:            https://github.com/ghostkellz/ghostctl
Source0:        https://github.com/ghostkellz/ghostctl/archive/v%{version}.tar.gz

BuildRequires:  rust >= 1.90
BuildRequires:  cargo >= 1.90
BuildRequires:  git
BuildRequires:  gcc
BuildRequires:  openssl-devel
BuildRequires:  pkg-config

Recommends:     docker
Recommends:     nginx
Recommends:     restic
Suggests:       btrfs-progs

%description
GhostCTL is a comprehensive command-line toolkit designed for Linux
system administrators, homelabbers, and power users. It provides
modular functionality for system monitoring and optimization, Docker
and container management, network configuration and diagnostics,
backup and storage management, security hardening and auditing,
and development environment setup.

The tool features an interactive menu system and supports both
automated scripting and manual operations across multiple Linux
distributions.

%prep
%autosetup -n %{name}-%{version}

%build
cd ghostctl
cargo build --release --locked

%install
# Install main binary
install -Dm755 ghostctl/target/release/ghostctl %{buildroot}%{_bindir}/ghostctl

# Install documentation
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md
install -Dm644 docs/README.md %{buildroot}%{_docdir}/%{name}/DOCS.md
install -Dm644 docs/reference/COMMANDS.md %{buildroot}%{_docdir}/%{name}/COMMANDS.md
install -Dm644 LICENSE %{buildroot}%{_docdir}/%{name}/LICENSE

# Install man page and shell completions
install -Dm644 man/ghostctl.1 %{buildroot}%{_mandir}/man1/ghostctl.1
ghostctl/target/release/ghostctl completion bash > ghostctl.bash
ghostctl/target/release/ghostctl completion zsh > _ghostctl
ghostctl/target/release/ghostctl completion fish > ghostctl.fish
install -Dm644 ghostctl.bash %{buildroot}%{_datadir}/bash-completion/completions/ghostctl
install -Dm644 _ghostctl %{buildroot}%{_datadir}/zsh/site-functions/_ghostctl
install -Dm644 ghostctl.fish %{buildroot}%{_datadir}/fish/vendor_completions.d/ghostctl.fish

# Install desktop entry
install -Dm644 packaging/ghostctl.desktop %{buildroot}%{_datadir}/applications/ghostctl.desktop

# Install icon
install -Dm644 assets/icons/png/ghostctl-icon-48.png %{buildroot}%{_datadir}/pixmaps/ghostctl.png

# Install example scripts if available
if [ -d scripts ]; then
    cp -r scripts %{buildroot}%{_docdir}/%{name}/
fi

%check
cd ghostctl
cargo test --release --locked

%files
%license LICENSE
%doc README.md
%{_bindir}/ghostctl
%{_datadir}/applications/ghostctl.desktop
%{_datadir}/pixmaps/ghostctl.png
%{_datadir}/bash-completion/completions/ghostctl
%{_datadir}/zsh/site-functions/_ghostctl
%{_datadir}/fish/vendor_completions.d/ghostctl.fish
%{_docdir}/%{name}/
%if 0%{?_mandir:1}
%{_mandir}/man1/ghostctl.1*
%endif

%changelog
* Thu May 15 2026 Christopher Kelley <ckelley@ghostctl.sh> - 0.10.0-1
- Release polish: docs generate command, domain diagnostics, Makefile
- Release archive now includes man page, completions, CHANGELOG
- Fixed FIXME sentinel in UEFI boot entry generation
- Removed dead code

* Sat Apr 25 2026 Christopher Kelley <ckelley@ghostctl.sh> - 0.9.9-1
- Align package version with Cargo metadata
- Install generated shell completions and man page

* Mon Sep 16 2025 Christopher Kelley <ckelley@ghostctl.sh> - 1.0.1-1
- New upstream release v1.0.1
- Gaming Module Completion with comprehensive optimization features
- Arch Module Major Enhancement with performance tuning system
- Native scanner implemented - replaced gscan with native Rust implementation
- Enterprise features integrated with advanced nftables, PVE security
- All modules accessible through menu system
- Code quality improvements and compilation fixes

* Sun Sep 15 2025 Christopher Kelley <ckelley@ghostctl.sh> - 1.0.0-1
- Major release v1.0.0
- PVE Storage Migration features
- MinIO/S3 Management integration
- Docker Registry Mirror support
- PVE Backup Rotation system
- PVE Template Management and Firewall Automation
- Advanced Container Cleanup tools
- Network Storage management (NFS/CIFS)

* Sat Sep 14 2025 Christopher Kelley <ckelley@ghostctl.sh> - 0.9.4-1
- Remove musl target and fix help command
- OpenSSL cross-compilation fixes for musl builds

* Fri Sep 13 2025 Christopher Kelley <ckelley@ghostctl.sh> - 0.9.3-1
- Initial RPM package release
- Cross-platform system administration toolkit
- Modular architecture with multiple subsystems
- Interactive menu-driven interface
- Support for Docker, networking, storage, and development tools
