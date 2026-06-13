# OpenShell

[OpenShell](https://github.com/nvidia/openshell) runs sandboxed, policy-governed
runtimes for autonomous AI agents behind a gateway control plane. GhostCTL does
not reimplement that CLI; it provides a readiness `doctor` and thin passthroughs
to the `openshell` binary so isolated-session workflows stay in one place.

## Quick Commands

```bash
ghostctl openshell doctor              # Check prerequisites (binary, docker, gateway, registration)
ghostctl openshell status              # Show the active gateway connection (passthrough)
ghostctl openshell gateway select dev  # Manage gateways (passthrough)
ghostctl openshell sandbox create -- claude   # Start an isolated session (passthrough)
ghostctl openshell policy ...          # Manage sandbox policy (passthrough)
```

## Doctor

`ghostctl openshell doctor` verifies the prerequisites for running isolated
sessions and prints a pass/fail checklist:

- `openshell` binary on `PATH`
- `docker` binary present and the daemon reachable
- gateway reachable at the configured URL (any HTTP response counts as up)
- an active gateway registered (`~/.config/openshell/active_gateway`)
- the CLI gateway registry present (`~/.config/openshell/gateways/`)

If a check fails, the doctor prints a short hint for resolving it.

## Passthrough

`status`, `gateway`, `sandbox`, and `policy` forward their arguments straight to
the `openshell` CLI with stdio inherited, so interactive flows and exit codes
behave exactly as if you ran `openshell` directly. GhostCTL only adds the
readiness check and a single entry point.

## Configuration

Settings live under `[openshell]` in `config.toml`:

```toml
[openshell]
bin = "openshell"                     # binary name or path
gateway_url = "http://127.0.0.1:18080"
timeout_secs = 10                     # doctor's gateway-reachability timeout
```
