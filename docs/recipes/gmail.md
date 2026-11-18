# Gmail

Lets your agent search and read your mail freely, and makes it ask your phone
before it sends, drafts, opens an attachment, or changes labels or filters.

```sh
bogo upstream add gmail
```

**Server:** [workspace-mcp](https://github.com/taylorwilsdon/google_workspace_mcp)
2.3.0 (MIT), with only its Gmail tools, for one user. It's installed in its own
Python virtual environment, so you need [uv](https://docs.astral.sh/uv/).

## What runs and what waits

| Runs without asking | Waits for your phone |
|---|---|
| `search_gmail_messages` | `send_gmail_message` |
| `get_gmail_message_content` | `draft_gmail_message` |
| `get_gmail_messages_content_batch` | `get_gmail_attachment_content` |
| `get_gmail_thread_content` | `modify_gmail_message_labels` |
| `get_gmail_threads_content_batch` | `batch_modify_gmail_message_labels` |
| `list_gmail_labels` | `manage_gmail_label` |
| `list_gmail_filters` | `manage_gmail_filter` |
| | `start_google_auth` |

In BOGO the tools are named `gmail__TOOL`, for example
`gmail__send_gmail_message`. On the Review card, a mail being sent shows its
subject as the title, the recipient under it, and the body as text; a draft
shows its subject and recipient. **All details** still shows every argument.

Any tool a later version of the server adds needs approval until you choose
otherwise.

## Sign in to Google

You sign in to Google yourself, in your browser. BOGO never sees your Google
password.

1. In [Google Cloud Console](https://console.cloud.google.com), make a project
   and enable the Gmail API for it.
2. Under APIs & Services → OAuth consent screen, add yourself as a test user.
3. Under Credentials, create an OAuth client ID of type **Desktop app**.
4. Run `bogo upstream add gmail` and give it that client's ID and secret. The
   ID is kept in `bogo.toml`; the secret goes in `secrets.toml`.
5. With BOGO running, ask your agent to read your mail. The first call answers
   with a Google sign-in link. Open it, sign in and allow Gmail.

The server keeps its Google token in a private folder BOGO makes for it,
`~/.bogo/upstreams/gmail` (mode 0700). Its own `HOME` points there too, so it
writes nothing to your home folder.

## What the command prints

```
Gmail through workspace-mcp 2.3.0 (taylorwilsdon/google_workspace_mcp, MIT)
Google OAuth client ID (ends in .apps.googleusercontent.com) [] 1234-abc.apps.googleusercontent.com
Google OAuth client secret (hidden; leave empty to skip)
Installing workspace-mcp==2.3.0 into /Users/you/.local/share/bogo/upstreams/gmail…
Starting it once to list its tools…
Added upstream gmail.
  allow    gmail__search_gmail_messages
  …
  approve  gmail__send_gmail_message
  …
```

followed by the sign-in steps above.

## Check that it works

1. `bogo doctor` shows `ok    upstream gmail starts`.
2. `bogo tools` lists the `gmail__` tools with the policies above.
3. Ask your agent to find your latest mail. It runs without asking (the first
   time, it answers with the sign-in link).
4. Ask it to draft a reply to yourself. Your phone shows the draft's subject
   and recipient on the Review card. Deny it, and the agent is told it was
   denied; nothing is drafted.

## What has been checked

The recipe has been run: it installed workspace-mcp 2.3.0, listed its tools
and wrote the policy above. Through BOGO, a search ran without approval and a
send waited for approval and was denied. A real Google sign-in and a real sent
mail have not been recorded. See [what is verified](../status.md).
