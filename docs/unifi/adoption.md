# Adoption

A factory-default UniFi device has to learn the controller URL before it can be
adopted. There are three ways to tell it, in order of preference:

1. **DHCP Option 43** — best on a managed LAN.
2. **DNS `unifi` record** — clean when you control the resolver.
3. **`set-inform`** — direct, for remote/L3 adoption where discovery can't reach.

`ghostctl unifi adopt` automates option 3 across a subnet. Options 1 and 2 are
environment glue documented in `public-misc/unifi/dhcp-option-43/`.

## `ghostctl unifi adopt`

```bash
ghostctl unifi adopt --subnet 192.168.1.0/24
ghostctl unifi adopt --subnet 192.168.1.0/24 --dry-run     # Print planned set-inform, run nothing
ghostctl unifi adopt --subnet 10.20.30.0/24 --user ubnt --inform-port 8080
```

| Flag | Purpose |
|------|---------|
| `--subnet <cidr>` | **Required.** CIDR to scan for factory devices. |
| `--controller <host>` | Override the inform host (default: configured controller). |
| `--inform-port <port>` | Override the inform port (default: `8080`). |
| `--user <u>` | Override the SSH user (default: tries the configured `adopt_ssh_users`). |

### What it does

1. **Preflight** — checks `nmap` and `ssh` are present (and `sshpass` only if a
   password is configured). Missing tools give a clear error.
2. **Discover** — `nmap -n -sU -p10001,22 <subnet>` and parses hosts with
   `10001/open` (the UniFi discovery port).
3. **Confirm** — lists what it found and prompts, unless `--yes`/headless.
4. **Adopt** — for each device, over SSH runs:
   `mca-cli-op set-inform http://<controller>:<port>/inform`
   trying each configured user with `StrictHostKeyChecking=no` and a short
   connect timeout (factory devices have throwaway host keys).
5. **Summary** — a per-device pass/fail report.

### Requirements

- Run from a host with **L2/L3 reachability to the device subnet on tcp/22**, and
  the devices must be able to reach the **controller on the inform port**.
- Key auth is preferred (`adopt_ssh_key`). For password auth, set
  `UNIFI_ADOPT_PASSWORD` in the environment — it is passed to `sshpass` via the
  `SSHPASS` env var, never on the command line (so it can't leak in the process
  list).

## Controller discovery (DHCP / DNS)

For managed LANs, prefer pushing the controller address via DHCP or DNS so
devices self-discover without SSH. See the tailored env docs:

- `public-misc/unifi/dhcp-option-43/windows-ad-dhcp.md` — Windows Server / AD.
- `public-misc/unifi/dhcp-option-43/fortigate-dhcp.md` — FortiGate DHCP server.
- `public-misc/unifi/dhcp-option-43/dns-discovery.md` — the `unifi` A record.

After any discovery change, confirm the device landed:

```bash
ghostctl unifi devices --pending
```
