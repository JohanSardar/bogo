# Connect an agent

An agent reaches BOGO as one MCP server. Give each agent its own name and
token, so BOGO knows which agent asked for each call:

```sh
bogo agent add 'Trip planner'
```

This prints the agent's token once. BOGO keeps only its hash. Put the token
where only that agent's MCP client can read it, and never in a project the
agent can edit.

**The quickest way:** `bogo setup agent` (part of [`bogo setup`](setup.md))
finds Claude Code, Claude Desktop and Cursor configs, moves their MCP servers
behind BOGO, registers one agent per client and writes its `bogo` entry, with
the token in a private file named by `BOGO_AGENT_TOKEN_FILE` instead of in the
client's config. `bogo setup --undo` restores the files.

There are two ways in. Both give the agent the same tools.

- **The shim (preferred).** The client starts `bogo-shim` as a stdio MCP
  server. The shim forwards each message to BOGO unchanged and ties the
  agent's session to the client process: if the client dies, BOGO ends the
  session at once and its pending requests with it.
- **HTTP.** The client connects to BOGO's MCP URL with
  `Authorization: Bearer <agent token>`. Use it only for clients that keep
  their HTTP session open reliably.

The shim needs two environment variables: `BOGO_URL`, BOGO's MCP URL
(`http://127.0.0.1:8790/mcp` with the default port), and `BOGO_AGENT_TOKEN`.
Use the absolute path to `bogo-shim`.

## Claude Code

Tested: the [build-from-source demo](build-from-source.md) runs this way.

Write `mcp.json` with only the command, and give Claude the two variables
through its environment, so the token stays out of the file:

```json
{"mcpServers": {"bogo": {"command": "/absolute/path/to/bogo-shim"}}}
```

```sh
export BOGO_URL=http://127.0.0.1:8790/mcp
export BOGO_AGENT_TOKEN='bogo_agt_...'
claude --mcp-config /path/to/mcp.json --strict-mcp-config
```

`--strict-mcp-config` makes BOGO the only MCP server for that run. Claude's
built-in tools, such as Bash, can still act without BOGO; turn them off with
`--tools ''`, or run Claude in the [reference sandbox](../sandbox/README.md).

## Other MCP clients

Not yet tested with BOGO. These follow each client's documented config format
and BOGO's [documented interface](daemon.md); please
[report](https://github.com/JohanSardar/bogo/issues/new/choose) what works.

**Any stdio client** (Codex, Cursor, VS Code, Zed and others): run
`bogo-shim` as the server command, with `BOGO_URL` and `BOGO_AGENT_TOKEN` in
its environment. For example, in Codex's `config.toml`:

```toml
[mcp_servers.bogo]
command = "/absolute/path/to/bogo-shim"
env = { BOGO_URL = "http://127.0.0.1:8790/mcp", BOGO_AGENT_TOKEN = "bogo_agt_..." }
```

A token written into a config file is readable by anything that can read that
file. Keep the file private to your account, outside any project the agent
works in.

**Any Streamable HTTP client:** use BOGO's URL with a bearer header:

```json
{"url": "http://127.0.0.1:8790/mcp", "headers": {"Authorization": "Bearer bogo_agt_..."}}
```

## After connecting

- `bogo tools` lists every tool BOGO exposes and its policy.
- Every tool BOGO doesn't list by name needs approval. Open or protect tools in
  [`bogo.toml`](configuration.md), then restart BOGO.
- `bogo agent pause 'Trip planner'` rejects all of that agent's calls until you
  unpause it.
