# Slack

Lets your agent read the channels you choose to allow, and makes it ask your
phone before it posts.

```sh
bogo upstream add slack
```

**Server:** [slack-mcp-server](https://github.com/korotovsky/slack-mcp-server)
1.3.0 (MIT), installed with npm, so you need Node.js.

## What runs and what waits

| Waits for your phone |
|---|
| `conversations_add_message` (posting) |

The server's other tools read channels, threads, users and search. For each
one, the recipe asks you, suggesting from the server's annotations; with
`--yes`, they all stay at approve. In BOGO the tools are named `slack__TOOL`.
On the Review card, a post shows its channel and its text.

The server leaves posting off unless you turn it on. To let the agent post at
all, add `SLACK_MCP_ADD_MESSAGE_TOOL` to the `env` of `[upstreams.slack]` in
`bogo.toml`, with a value the
[server's README](https://github.com/korotovsky/slack-mcp-server#readme)
describes. Each post still waits for your phone.

## Sign in to Slack

1. Make a Slack app at [api.slack.com/apps](https://api.slack.com/apps).
2. Add the user token scopes it needs: `channels:history`, `channels:read` and
   `users:read`, plus `chat:write` to post.
3. Install it to your workspace and copy its **user OAuth token**, which
   starts with `xoxp-`.
4. Give that token to the recipe. It goes in `secrets.toml`, and BOGO passes
   it to the server as `SLACK_MCP_XOXP_TOKEN`.

## Check that it works

1. `bogo doctor` shows `ok    upstream slack starts`.
2. Ask your agent to summarise a channel's latest messages.
3. With posting turned on, ask it to post "hello" to a test channel. Your
   phone shows the channel and the text. Deny it, and nothing is posted.

## What has been checked

The recipe's pinned version and policy are in the code. A live run against
Slack has not been recorded. See [what is verified](../status.md).
