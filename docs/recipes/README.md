# Recipes

A recipe adds a well-known MCP server behind BOGO in one command. It installs
the server at a pinned version, asks for its credentials, lists its tools and
writes a policy that lets reading run and makes sending, writing, paying and
deleting wait for your phone.

```sh
bogo upstream add gmail
```

| Recipe | Server | Runs without asking | Waits for your phone |
|---|---|---|---|
| [`gmail`](gmail.md) | [workspace-mcp](https://github.com/taylorwilsdon/google_workspace_mcp) 2.3.0 | searching and reading mail, listing labels and filters | sending, drafting, attachments, changing labels or filters |
| [`github`](github.md) | [github-mcp-server](https://github.com/github/github-mcp-server) 2.0.3 | tools GitHub marks read-only, once you confirm each | writing files, pull requests, issues, comments, merges |
| [`slack`](slack.md) | [slack-mcp-server](https://github.com/korotovsky/slack-mcp-server) 1.3.0 | what you confirm | posting messages |
| [`stripe`](stripe.md) | Stripe's hosted server, test-mode keys only | what you confirm | payment links, refunds, invoices and other writes |
| [`files`](files.md) | [server-filesystem](https://github.com/modelcontextprotocol/servers/tree/main/src/filesystem) 2.3.0 | reading and listing one folder | writing, editing, moving, making folders |

`bogo upstream recipes` lists them.

## How every recipe works

1. It asks for everything first: credentials with echo off, and any other
   value it needs. Nothing is installed if you stop here.
2. It installs the server outside BOGO's home, in
   `~/.local/share/bogo/upstreams/NAME`: npm packages with `npm`, Python
   packages in their own virtual environment made by
   [uv](https://docs.astral.sh/uv/), and release archives checked against
   their published SHA-256.
3. It starts the server once to list its tools. The recipe's own lists come
   first. For any other tool the server offers, it suggests a policy from the
   server's annotations and you choose allow, approve or deny.
4. It adds an `[upstreams.NAME]` section and the tool policies to `bogo.toml`,
   and the credentials to `secrets.toml`, where `bogo.toml` refers to them by
   name. If BOGO is running in the background, it restarts BOGO to load them.
5. It prints the policy and the sign-in steps.

Protected tools need the agent's justification, which the Review card shows
as "Reason", marked "Not verified by BOGO". `--yes` takes every suggestion,
but never allows a tool the recipe doesn't allow itself.

To check a recipe: `bogo doctor` shows `ok    upstream NAME starts`, and
`bogo tools` lists its tools as `NAME__TOOL` with their policies. Then ask your
agent to do one thing that's allowed and one that's protected: the first runs,
and the second waits on your phone.

Change a policy later in `bogo.toml` ([configuration](../configuration.md)).
To remove a recipe, delete its sections from `bogo.toml` and its credentials
from `secrets.toml`, and delete its folder in `~/.local/share/bogo/upstreams`.
