# GhostCTL Architecture

GhostCTL is a single Rust CLI that routes operational workflows across Linux
system administration domains. The command surface is broad, but the runtime
shape is intentionally simple: parse command intent, collect local context,
call the smallest necessary local tool or API, and write evidence to XDG state
paths when a workflow needs support or audit artifacts.

## System Map

```mermaid
flowchart TD
    operator["Operator"] --> cli["ghostctl CLI"]
    operator --> menu["interactive menus"]
    operator --> docs["docs/reference/COMMANDS.md"]

    cli --> router["clap command router"]
    menu --> router

    router --> system["System modules\nArch, Btrfs, NVIDIA, VFIO, UEFI"]
    router --> infra["Infrastructure modules\nDocker, Proxmox, GitLab, OpenShell"]
    router --> security["Security modules\nSSH, GPG, signing, audit, CrowdSec"]
    router --> observability["Observability modules\nmonitor, OBS, support"]
    router --> dev["Developer modules\nRust/Zig/Go/Python/JS tools"]

    system --> local_tools["local commands\npacman, btrfs, nvidia-smi, efibootmgr"]
    infra --> services["service APIs and CLIs\nDocker, PVE, GitLab, openshell"]
    security --> trust["keys, credentials,\nlockfiles, workflow files"]
    observability --> state["XDG state\nlogs, support bundles, metadata"]
    dev --> toolchains["toolchains and package managers"]
```

## Command Execution Model

```mermaid
flowchart TD
    input["CLI args or menu selection"] --> parse["parse with clap"]
    parse --> mode["resolve global mode\n--dry-run, --yes, --plain, headless"]
    mode --> command["dispatch command module"]
    command --> preflight["validate inputs and required tools"]
    preflight --> decision{"mutating action?"}

    decision -->|no| collect["collect local facts"]
    decision -->|yes| guard["confirm or honor --yes\nsupport --dry-run where available"]
    guard --> execute["run command or API call"]
    collect --> render["render table/text/json output"]
    execute --> render
    render --> log["record activity when supported"]
    log --> exit["exit status reflects command outcome"]
```

## Safety Boundaries

```mermaid
flowchart LR
    readonly["Read-only diagnostics"] --> safe["default command path"]
    dryrun["--dry-run"] --> safe
    prompts["interactive confirmation"] --> mutations["mutating operations"]
    yes["--yes"] --> mutations

    mutations --> local["local package/service/file changes"]
    mutations --> remote["remote API actions\nGitLab/PVE/etc."]
    safe --> evidence["support bundle, audit report,\nstatus output"]

    evidence --> review["operator reviews before sharing"]
```

## Documentation Map

```mermaid
flowchart TD
    start["Start here"] --> index["docs/README.md"]
    index --> install["deployment/INSTALL.md"]
    index --> commands["reference/COMMANDS.md"]
    index --> support["support/README.md"]

    index --> system["System management"]
    system --> arch["arch/README.md"]
    system --> btrfs["btrfs/README.md"]
    system --> nvidia["nvidia/README.md"]
    system --> virt["virtualization/README.md"]

    index --> infra["Infrastructure"]
    infra --> docker["docker/README.md"]
    infra --> proxmox["proxmox/README.md"]
    infra --> gitlab["gitlab/gitlab.md"]
    infra --> openshell["openshell/openshell.md"]

    index --> security["Security and audit"]
    security --> security_index["security/README.md"]
    security --> deps["security/dependency-audit.md"]
    security --> ci["security/ci-workflow-audit.md"]
    security --> package["security/package-audit.md"]
    security --> signing["signing/README.md"]

    index --> ops["Observability and desktop"]
    ops --> monitor["monitor/monitoring.md"]
    ops --> obs["obs/wayland-screencapture.md"]
```

## Release Gate Shape

```mermaid
flowchart TD
    worktree["reviewed worktree"] --> fmt["cargo fmt --check"]
    fmt --> check["cargo check"]
    check --> clippy["cargo clippy --release -- -D warnings"]
    clippy --> tests["cargo test"]
    tests --> build["cargo build --release"]
    build --> audit["cargo audit"]
    audit --> docs["docs and changelog review"]
    docs --> packages["package metadata\nArch, Debian, Fedora"]
    packages --> tag{"tag ready?"}
    tag -->|yes| release["vX.Y.Z release"]
    tag -->|no| fix["fix blocker and rerun gates"]
    fix --> fmt
```

## Design Notes

| Area | Policy |
|------|--------|
| Output | Human-readable by default; JSON where automation needs it |
| State | XDG config/data/state paths, with support artifacts under state |
| External tools | Checked before use where practical; failures should be reported, not panicked |
| Mutations | Prefer confirmation, `--dry-run`, and explicit user intent |
| Security | Favor pinned workflow actions, lockfile-based audits, and local parsing before remote calls |
