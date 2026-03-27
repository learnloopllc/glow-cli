# Glow CLI

CLI for the Glow platform. Talks to both the central LearnLoop API and individual Glow instances.

## Install

### Homebrew (macOS / Linux)

```bash
brew install learnloopllc/glow-cli/glw
```

### Shell script

```bash
curl -fsSL https://raw.githubusercontent.com/learnloopllc/glow-cli/main/install.sh | sh
```

### From source

```bash
cargo install --git https://github.com/learnloopllc/glow-cli.git
```

Installs two binaries: `glow` (full name) and `glw` (short alias).

## Shell completions

```bash
# Bash
echo 'source <(glow completions bash)' >> ~/.bashrc

# Zsh
echo 'source <(glow completions zsh)' >> ~/.zshrc

# Fish
glow completions fish > ~/.config/fish/completions/glow.fish
```

## Global flags

| Flag | Env | Description |
|------|-----|-------------|
| `--api-url` | `GLOW_API_URL` | LearnLoop API URL (default: `https://api.learn-loop.org`) |
| `--instance-url` | `GLOW_INSTANCE_URL` | Glow instance URL |
| `--client-id` | `GLOW_CLIENT_ID` | OAuth client ID |
| `--json` | | Output as JSON |
| `--yes` / `-y` | | Skip confirmation prompts |
| `--version` | | Print version |

## Glow instance commands

```bash
glow login                                  # OAuth login to Glow instance
glow logout                                 # Remove stored token
glow health                                 # Check instance health
glow context                                # Show current user identity
glow emulate <profile_id> [--ttl 120]       # Emulate another user
glow unemulate                              # Stop emulating
glow generate <group_id> [--body JSON]      # Generate content for a group
glow stream --artifact X --operation Y      # Stream SSE events
```

## Instance management

```bash
glow instances list                         # List configured instances
glow instances add <name> --url <url>       # Add an instance
glow instances remove <name>                # Remove an instance
glow use <name>                             # Switch active instance
```

## Admin commands (LearnLoop management plane)

### Auth & status

```bash
glow admin login                            # OAuth login to LearnLoop API
glow admin logout                           # Remove stored token
glow admin sessions                         # List all active sessions
glow admin whoami                           # Show authenticated user
glow admin network                          # Check API connectivity
glow admin status                           # Config + connection status
```

### Organizations

```bash
glow admin orgs list                        # List your organizations
glow admin orgs create --name X             # Create organization
glow admin orgs get <id>                    # Get organization details
glow admin orgs update <id> [--name X]      # Update organization
glow admin orgs delete <id>                 # Delete organization
glow admin orgs members <id> list           # List members
glow admin orgs members <id> add --email X  # Add member
glow admin orgs members <id> remove <uid>   # Remove member
glow admin orgs deployments <id>            # List org deployments
```

### Deployments

```bash
glow admin deploy create --org-id X --name Y --subdomain Z --version V
glow admin deploy stop <id>                 # Stop deployment
glow admin deploy destroy <id>              # Destroy deployment
glow admin deploy list [--all]              # List deployments
```

### Usage & billing

```bash
glow admin usage <org_id>                   # Usage summary
glow admin billing plans                    # List available plans
glow admin billing status <org_id>          # Subscription status
glow admin billing checkout <org_id>        # Start checkout
glow admin billing portal <org_id>          # Open billing portal
```

## Dynamic resources (Glow instance)

Parameters can be passed directly as flags or as raw JSON:

```bash
glow personas search --query math --limit 10
glow personas search --body '{"query": "math", "limit": 10}'
glow <resource> <action> --help             # Show available parameters
```

### Media operations

```bash
glow <resource> <media> <action> [flags]    # e.g. glow documents file upload --file X
```

Actions: `upload`, `download`, `create`, `chunk`, `status`, `finalize`, `discover`, `preview`
