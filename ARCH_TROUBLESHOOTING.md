# Arch Linux Troubleshooting with ghostctl

## Quick Commands

### Auto-Fix Everything (Recommended)
```bash
ghostctl arch bouncer
```
This will:
- 🔍 Automatically detect issues (network, mirrors, locks, keyring, etc.)
- 🔧 Fix them in the optimal order
- ✅ Report what was fixed

### Manual Fix by Category
```bash
ghostctl arch bouncer pacman    # Fix pacman database issues
ghostctl arch bouncer keyring   # Fix GPG keyring issues
ghostctl arch bouncer mirrors   # Fix mirror connectivity
ghostctl arch bouncer all       # Fix everything manually
```

### System Fix with Diagnostics
```bash
ghostctl arch fix
```
This runs comprehensive diagnostics and fixes issues before upgrading.

## What Gets Auto-Detected

The new diagnostics system automatically detects:

1. **getcwd errors** - Working directory issues
   - Auto-fixes by changing to $HOME or /tmp

2. **Pacman locks** - `/var/lib/pacman/db.lck`
   - Auto-removes stale locks

3. **Network issues** - Connection failures
   - Suggests network restart

4. **Mirror problems** - Unreachable/slow mirrors
   - Updates mirrorlist with fast, working mirrors

5. **Keyring issues** - Corrupted GPG keys
   - Reinitializes and repopulates keyring

6. **Database issues** - Out of sync packages
   - Syncs package database

7. **Orphaned packages** - Unused dependencies
   - Removes them safely

## Example Scenarios

### Scenario 1: Network Unreachable After Update
```
Error: Network is unreachable
Warning: failed to rate http(s) download
```

**Solution:**
```bash
ghostctl arch bouncer
```
Output:
```
🏀 Auto-Bouncer: Detecting and fixing issues...
🔍 Running system diagnostics...
  📡 Network issue: Cannot reach archlinux.org
  🌐 Mirror issues detected
⚠️  Issues detected:
  • Network connectivity issues
  • Mirror configuration problems

🔧 Executing 2 fix action(s)...
[1/2] Fix network connectivity
[2/2] Update mirror list
  ✅ Mirrors updated
```

### Scenario 2: getcwd Error After Directory Deleted
```
shell-init: error retrieving current directory: getcwd: cannot access parent directories
```

**Solution:**
```bash
ghostctl arch bouncer
```
The bouncer will detect and fix the working directory issue automatically.

### Scenario 3: Pacman Lock After Crash
```
error: failed to init transaction (unable to lock database)
```

**Solution:**
```bash
ghostctl arch bouncer
```
Auto-detects and removes the lock, then syncs the database.

### Scenario 4: Permission Errors with du/System Commands
The diagnostics system now:
- Runs as non-root where possible
- Only uses sudo when necessary
- Handles permission errors gracefully

## Advanced Usage

### Get Diagnostics Without Fixing
```bash
# The fix command shows diagnostics first
ghostctl arch fix
# Then decide whether to proceed
```

### Clean Specific Issues
```bash
ghostctl arch clean orphans     # Remove orphaned packages
ghostctl arch clean mirrors     # Clean and optimize mirrors
ghostctl arch clean pkgfix      # Clean build environment
ghostctl arch clean gpg         # Fix GPG keys
ghostctl arch clean locks       # Clear pacman locks
ghostctl arch clean all         # All of the above
```

### Other Useful Commands
```bash
ghostctl arch mirrors           # Optimize mirror list only
ghostctl arch orphans           # Remove orphans only
ghostctl arch optimize          # Performance optimizations
ghostctl arch health            # System health check
```

## How It Works

1. **Diagnostics Phase**: Scans for common issues
   - Network connectivity
   - Mirror availability
   - Pacman locks
   - Keyring integrity
   - Database sync status
   - Orphaned packages
   - Working directory

2. **Fix Sequence**: Prioritizes fixes in optimal order
   - Critical issues first (locks, directories)
   - Then infrastructure (network, mirrors)
   - Then package system (keyring, database)
   - Finally cleanup (orphans)

3. **Execution**: Each fix runs independently
   - Success/failure reported per action
   - Continues even if one fails
   - Final system sync at the end

## Benefits Over Manual Troubleshooting

- ✅ No need to diagnose issues yourself
- ✅ Fixes applied in optimal order
- ✅ Safe - doesn't break working systems
- ✅ Fast - one command does it all
- ✅ Informative - shows what it's doing
- ✅ Resilient - continues even if one fix fails

## When to Use

Use `ghostctl arch bouncer` when you encounter:
- Package update failures
- Network unreachable errors
- Keyring signature errors
- Database lock errors
- Mirror timeout issues
- getcwd/permission errors
- "Something went wrong" with pacman

It's safe to run anytime - if no issues are detected, it just syncs the database.
