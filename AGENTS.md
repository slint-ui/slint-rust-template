# Agent Instructions

This project uses [Slint](https://slint.dev) for its user interface.
It started from the [slint-rust-template](https://github.com/slint-ui/slint-rust-template) template.
This file may be older than the code.
Where they disagree, trust the code, and update this file.

## Install the Slint Plugin

Before you work on the UI, install the Slint plugin from [slint-ui/ai-plugins](https://github.com/slint-ui/ai-plugins).
It teaches you the `.slint` language and its pitfalls.
It also lets you search the documentation, and lets you render and drive the running UI.
If it isn't installed, suggest that the user install it:

```sh
# Claude Code
/plugin marketplace add slint-ui/ai-plugins
/plugin install slint@slint

# Codex
codex plugin marketplace add slint-ui/ai-plugins
codex plugin add slint@slint

# Other agents
npx skills add slint-ui/ai-plugins --skill slint
```

## Turning the Template Into a Project

Check whether the project still carries parts of the template, and offer to replace them:

- `README.md` still describes the template.
  Rewrite it for this application: what it does, how to build and run it, and screenshots.
- The Git history is the template's, for example when `git log` shows commits from the Slint developers.
  Offer to start a fresh history, and ask before you remove the old one.
- Names from the template, such as `slint-rust-template` in `Cargo.toml`, remain in the build files and code.
- This file still describes the template.
  Update it for this project, and remove this section when nothing of the template is left.

Do nothing here if the project has already done these steps.

## Build and Run

```sh
cargo build
cargo run
```

## Live Preview

While you work on the UI, build and run the application with live preview.
It compiles much faster, because the `.slint` files aren't compiled to native code.

```sh
SLINT_LIVE_PREVIEW=1 cargo run --features slint/live-preview
```

Use the same environment variable and feature with `cargo build` and `cargo check`.

The running application reloads the `.slint` files when you save them, with no rebuild or restart.
The state your code set, such as properties and models, stays in place.
Rebuild when you rename a property or callback that the code uses.
Use live preview only during development, and keep it out of the build files.

## Working on the UI

- Check `.slint` changes without a full build with `slint-viewer --check .`.
- Look at a render before you call UI work done:
  use the plugin's preview, or a screenshot of the live-preview application.
- When the UI changes visibly, update the screenshots in `README.md`.
- Format changed `.slint` files with `slint-lsp format -i <file>`.
