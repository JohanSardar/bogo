# Files

Lets your agent read one folder, and makes it ask your phone before it
writes, edits or moves a file, or makes a folder.

```sh
bogo upstream add files
```

**Server:** [@modelcontextprotocol/server-filesystem](https://github.com/modelcontextprotocol/servers/tree/main/src/filesystem)
2.3.0 (MIT), installed with npm, so you need Node.js. Nothing to sign in to.

The recipe asks which folder the agent may reach. It refuses `/` and your
whole home folder, and the server can reach nothing outside the folder you
give.

## What runs and what waits

| Runs without asking | Waits for your phone |
|---|---|
| `read_text_file`, `read_media_file`, `read_multiple_files`, `read_file` | `write_file` |
| `list_directory`, `list_directory_with_sizes`, `directory_tree` | `edit_file` |
| `search_files`, `get_file_info`, `list_allowed_directories` | `create_directory` |
| | `move_file` |

The server also offers `append_file` and `copy_file`, which the recipe asks
you about; it suggests approve for both. In BOGO the tools are named
`files__TOOL`. On the Review card, a write or edit shows the file's path as
its title, and a move shows where the file goes.

## What the command prints

```
Files in one folder, through @modelcontextprotocol/server-filesystem 2.3.0 (MIT)
Which folder may the agent reach? [] /Users/you/agent-files
Installing @modelcontextprotocol/server-filesystem@2.3.0 into /Users/you/.local/share/bogo/upstreams/files…
Starting it once to list its tools…
files__append_file: it publishes no hint that it only reads. allow, approve or deny? [approve]
files__copy_file: it publishes no hint that it only reads. allow, approve or deny? [approve]
Added upstream files.
  allow    files__read_text_file
  …
  approve  files__write_file
  approve  files__edit_file
  approve  files__create_directory
  approve  files__move_file
Nothing to sign in to. The agent can read the folder you gave; writing, editing and
moving files wait for your phone.
```

## Check that it works

1. `bogo doctor` shows `ok    upstream files starts`.
2. Ask your agent to list the folder. It runs.
3. Ask it to write a file there. Your phone shows the path on the Review card,
   and the file doesn't exist until you approve.

## What has been checked

The recipe has been run as above. The same server, behind BOGO with the same
policy, is what the [build-from-source demo](../build-from-source.md) runs: a
read ran, a write waited, was approved with the software test approver, and
then ran.
