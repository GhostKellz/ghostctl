# CrowdSec Whitelist

When the UniFi frontend is internet-exposed, CrowdSec protects it — but you must
never ban legit device check-ins or your own admin access. `ghostctl crowdsec
unifi-exempt generate` emits a CrowdSec whitelist parser tailored from your
`[unifi]` config so that traffic is always exempt.

```bash
ghostctl crowdsec unifi-exempt generate          # Print the whitelist YAML to stdout
ghostctl crowdsec unifi-exempt generate --apply  # Write it to the parser enrich stage
ghostctl crowdsec unifi-exempt generate --apply /custom/path.yaml
```

## What it whitelists

- Every entry in `[unifi].exempt_cidrs` — split into `cidr:` (e.g. Tailscale
  CGNAT `100.64.0.0/10`) and `ip:` lists. Add your mgmt / device VLANs here.
- The controller host and inform host, when they are literal IPs (hostnames
  can't go in a CrowdSec IP whitelist — resolve them first).

The output is a `s02-enrich` stage parser named `ghostctl/unifi-whitelist`:

```yaml
name: ghostctl/unifi-whitelist
description: "Whitelist UniFi mgmt/inform + Tailscale so legit traffic is never banned"
whitelist:
  reason: "UniFi controller management, inform sources, and Tailscale"
  ip:
    - "69.169.98.98"
  cidr:
    - "100.64.0.0/10"
```

## Apply and reload

`--apply` with no value writes to
`/etc/crowdsec/parsers/s02-enrich/unifi-whitelist.yaml` (often needs `sudo`). It
respects the global `--dry-run` (print what would be written, touch nothing).
After writing, reload CrowdSec:

```bash
sudo systemctl reload crowdsec
```

## Where this runs

The whitelist belongs on the host running CrowdSec — which must sit at the
**reverse proxy in front of the exposed frontend** for the bouncer to actually
enforce. A whitelist on a box that never sees the frontend traffic does nothing.

## fail2ban

Use CrowdSec **or** fail2ban against a given log source, not both — they fight
over the same bans. CrowdSec's crowd-sourced blocklists and reverse-proxy
bouncer supersede fail2ban for an internet-exposed UniFi frontend. See
[../security/crowdsec.md](../security/crowdsec.md) for the broader CrowdSec feed
and metrics commands.

## Reference

A tailored reference copy of the generated whitelist lives at
`public-misc/unifi/crowdsec/unifi-whitelist.yaml`.
