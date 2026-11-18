# GitHub

Lets your agent read repositories, issues and pull requests, and makes it ask
your phone before it writes a file, opens or merges a pull request, opens an
issue or comments.

```sh
bogo upstream add github
```

**Server:** [github-mcp-server](https://github.com/github/github-mcp-server)
2.0.3 (MIT), GitHub's own server. The recipe downloads its release archive for
your Mac and checks it against the pinned SHA-256 before unpacking it.

## What runs and what waits

| Waits for your phone |
|---|
| `create_or_update_file`, `push_files`, `delete_file` |
| `create_pull_request`, `merge_pull_request` |
| `create_issue`, `add_issue_comment` |

The server offers many more tools. For each one, the recipe asks you: it
suggests allow for tools GitHub marks read-only and approve for the rest.
With `--yes`, every one of them stays at approve. In BOGO the tools are named
`github__TOOL`.

## Sign in to GitHub

Make a [fine-grained personal access token](https://github.com/settings/personal-access-tokens),
limited to the repositories and permissions the agent needs, and give it to
the recipe. It goes in `secrets.toml`, and BOGO passes it to the server as
`GITHUB_PERSONAL_ACCESS_TOKEN`. A token that can only read can't be used to
write, whatever you approve.

## Check that it works

1. `bogo doctor` shows `ok    upstream github starts`.
2. Ask your agent to list the open issues in one of your repositories. It
   runs, if you allowed that tool.
3. Ask it to comment on one. Your phone shows `github__add_issue_comment` on
   the Review card. Deny it, and no comment is posted.

## What has been checked

The recipe's pinned archive and checksum are in the code, and its policy is
written as above. A live run against GitHub has not been recorded. See
[what is verified](../status.md).
