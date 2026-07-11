# Doctor

`ghostctl unifi doctor` surfaces the L2 and adoption problems the UniFi UI can't
show you — the ones that bite a **Fortinet + UniFi** shop: STP-blocked ports,
stuck adoption, uplink flaps, and firmware skew across a mixed estate.

```bash
ghostctl unifi doctor
```

It reads the device list and, when available, the controller's per-device port
stats, then prints findings grouped by severity plus FortiGate-side hints.

## What it checks

| Area | Finding |
|------|---------|
| **Adoption** | Devices stuck pending / adopting / disconnected, heartbeat missed, or adoption failed — mapped from the controller's device state. |
| **STP** | Ports in a blocking/discarding state while the link is up (a classic loop-guard or root-bridge misconfig between FortiSwitch and UniFi). |
| **Uplinks** | Down uplink ports and links with high error counters. |
| **Firmware** | Devices whose version differs from the estate majority (migration/upgrade hint). |

Severities: **Crit** (blocking traffic or failed adoption), **Warn** (degraded —
down uplink, error counters, version skew), **Info** (context).

## Data sources and resilience

Doctor prefers the controller's private per-device stats endpoint for rich port
and STP state. That endpoint is **version-fragile** across UOS Server upgrades,
so if it isn't available the command **degrades gracefully** to the documented
integration device list and still reports adoption and firmware findings. A UOS
upgrade can't take the whole command down.

## FortiGate hints

Because the UniFi controller can't see the FortiGate/FortiSwitch side, doctor
prints a checklist of the usual cross-vendor culprits:

- **STP edge / BPDU guard** on AP/switch access ports (or lack of it) causing
  ports to sit in listening/blocking.
- **RSTP vs MSTP mismatch** or a FortiSwitch winning root bridge unexpectedly.
- **Option 43 / DNS discovery** not reaching the device VLAN (adoption never
  starts) — see [adoption.md](adoption.md).
- **Storm-control / loop-guard** dropping UniFi discovery/inform traffic.
- **Native VLAN** mismatch on the UniFi uplink trunk.

These are printed as guidance to check on the Fortinet side; doctor does not log
into the FortiGate.

## Typical flow

```bash
ghostctl unifi devices --pending    # See what hasn't adopted
ghostctl unifi doctor               # Find out why (STP, inform reachability, firmware)
```
