# ticked

`ticked` opens your [ticktick.com](https://ticktick.com) inbox in your favorite
editor (vim/neovim, for example) so you able to edit them (modify title/body or
"mark as done") from it (yes, from your lovely editor).

## Demo

<img src="https://github.com/UnkwUsr/ticked/assets/49063932/10cae58e-874a-46d8-bff1-fa3728991914" width="412" height="381">

## Features

* uses web api
* have 2 modes: "[e]dit" and "[d]elete"

## Installation

With `cargo`:

```sh
cargo install ticked
```

## Preparation

You need to extract value of cookie named "t" from authorized session on
[ticktick.com](https://ticktick.com) site.

## Usage

```sh
TICKTICK_COOKIE=<your_cookie_there> ticked
```

(you may want to create wrapper script for this)

It will open your `$EDITOR` with tasks from your ticktick inbox list. On the
first line you'll see `# Mode: d`. There is two possible modes: `d` and `e`.
Just edit this line to select mode you want.

### Mode `d` (delete/done)

In `d` mode program detects only deleted tasks and then marks them as "done" on
ticktick side. Under the hood it detects if some of the original lines
disappeared. So if in this mode you'll edit some task title, it will be
considered as this task disappeared, hence should be removed. So be careful.

### Mode `e` (edit)

In `d` mode program detects changes of tasks and then update them on ticktick
side. Under the hood it compares line by line in original order and looks for
changed lines, so you shouldn't reorder lines in this mode, as it will mess
everything.

---

When all done, just save file and close editor. `ticked` will show your changes
and ask for final confirmation.

## Status / Future of the project

The code base is very dirty right now. Sorry.

I am currently in the process of migrating from ticktick to plaintext local
files, so using this tool daily as easy way to extract inbox ("delete" mode).

(2023/05/16: I still use it and it works)

No active work is planned on this. I'll be happy to apply pull requests, or you
may fork or just get inspiration (I'll be glad to add links to related
projects/forks, so feel free to create issue for that). However, there is
[TODO.md](./TODO.md) file with old plans and thoughts.

"Edit" mode is not well tested. But `ticked` asks you to confirm any changes
(in "delete" mode too), so it should be safe to use. Just **review everything
before confirm**. And even if you'll mark something as done occasionally, you
can find it in ticktick "completed" folder.
