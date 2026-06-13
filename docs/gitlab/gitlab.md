# GitLab

`ghostctl gitlab` is a lightweight client for the GitLab REST API (`/api/v4`).
It targets self-hosted instances (e.g. `https://git.cktechx.com`) as well as
gitlab.com, and covers the day-to-day checks that pair with the rest of
ghostctl's dev/CI tooling: connectivity and auth (`status`), CI file validation
(`ci-lint`), pipelines and jobs (`pipelines`, `pipeline`, `trace`), merge
requests (`mrs`), runners (`runners`), and project discovery (`projects`).

The read-only checks are always safe. Three write actions — `run`, `retry`, and
`cancel` — trigger or change pipelines; they honor the global `--dry-run` (print
intent, send nothing) and `--yes` flags, and otherwise prompt before acting.

The access token is never logged.

## Quick Commands

```bash
# Read-only
ghostctl gitlab status                  # Verify connectivity and authentication
ghostctl gitlab ci-lint                 # Validate .gitlab-ci.yml via the CI Lint API
ghostctl gitlab ci-lint path/to/ci.yml  # Validate a specific CI file
ghostctl gitlab pipelines               # List recent pipelines for the configured project
ghostctl gitlab pipeline 12345          # Show one pipeline and its jobs, grouped by stage
ghostctl gitlab trace 67890             # Print a job's log (debug a failed CI job)
ghostctl gitlab mrs                     # List open merge requests (alias: mr)
ghostctl gitlab runners                 # List CI runners available to the project
ghostctl gitlab projects                # List projects you are a member of

# Write (honors --dry-run / --yes)
ghostctl gitlab run                     # Trigger a pipeline on the default branch
ghostctl gitlab run feature/x           # Trigger a pipeline on a specific ref
ghostctl gitlab retry 12345             # Retry a pipeline
ghostctl gitlab cancel 12345            # Cancel a pipeline
```

## Configuration

Settings live under `[gitlab]` in `~/.config/ghostctl/config.toml`:

```toml
[gitlab]
url = "https://git.cktechx.com"   # Instance base URL (default: https://gitlab.com)
project = "group/subgroup/repo"   # Numeric id or path; required for most subcommands
timeout_secs = 15
# token = "glpat-..."             # Optional; env vars take precedence
```

### Token Resolution

The token is resolved in order, and the first non-empty value wins:

1. `GITLAB_TOKEN`
2. `GHOSTCTL_GITLAB_TOKEN`
3. `[gitlab].token` in the config file

Keeping the token in the environment means it never has to be written to disk.
The read-only checks work with a `read_api`-scoped token; the write actions
(`run`/`retry`/`cancel`) require the full `api` scope. `ghostctl gitlab status`
reports a clear error on a 401, and the write actions surface a 403 when the
token lacks permission.

## Subcommands

### `status`

Queries `/version` and `/user` to confirm the instance is reachable and the
token authenticates. Prints the instance URL, GitLab version, the authenticated
user, and the configured default project (if any).

### `ci-lint [file]`

Validates a CI file (default: `.gitlab-ci.yml`) via the CI Lint API. When a
`project` is configured, the project-scoped endpoint is used so `include:` and
project variables resolve; otherwise the global syntax-only endpoint is used.
Warnings and errors are printed; the command exits non-zero when the file is
invalid.

### `pipelines`

Lists the 20 most recent pipelines for the configured `project` (numeric id or
`group/repo` path — paths are percent-encoded automatically), showing id,
status, ref, and web URL. Requires `[gitlab].project` to be set.

### `pipeline <id>`

Shows a single pipeline's summary (id, ref, status, web URL) followed by its
jobs in a `STAGE / JOB / STATUS / JOB ID` table. Use the printed job id with
`trace` to read a job's log. Requires `[gitlab].project`.

### `trace <job>`

Prints the raw log for a job id (from `pipeline <id>`). Handy for inspecting a
failed CI job without opening the web UI. Requires `[gitlab].project`.

### `runners`

Lists CI runners in an `ID / STATUS / ONLINE / DESCRIPTION` table. When
`[gitlab].project` is set the project-scoped runners are shown; otherwise the
runners the token can see across the instance are listed.

### `mrs` (alias `mr`)

Lists open merge requests for the configured `project`, most-recently-updated
first: `!iid [draft] title` with the author and `source → target` branches.
Requires `[gitlab].project`.

### `projects`

Lists the projects you are a member of (`ID / VISIBILITY / PATH`), ordered by
last activity. Useful for finding the value to put in `[gitlab].project`. Does
not require a project to be configured.

### `run [ref]`

Triggers a new pipeline. With no argument it resolves the project's default
branch first; pass a branch or tag to target a specific ref. Write action:
under `--dry-run` it prints what it would do and sends nothing; otherwise it
prompts for confirmation (auto-accepted with `--yes` or in headless mode).
Requires `[gitlab].project` and a token with the `api` scope.

### `retry <id>` / `cancel <id>`

Retries or cancels a pipeline by id. Both are write actions with the same
`--dry-run`/`--yes` semantics as `run`, and require `[gitlab].project` and an
`api`-scoped token.

## See Also

- [CI/CD Workflow Audit](../security/ci-workflow-audit.md) — offline `.gitlab-ci.yml` deprecation checks
