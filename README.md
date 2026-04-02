# GitHub Repo Control

GitHub Repo Control is a terminal app for reviewing your GitHub repositories and making selected public repositories private.

## What It Does

- Prompts for your GitHub username and token inside the TUI.
- Validates credentials before opening the dashboard.
- Lists only public repositories.
- Lets you select repositories and make them private in batches.

## Requirements

- Rust 1.85+ or a recent stable toolchain.
- A GitHub personal access token with permission to read and update repositories.
- Network access to GitHub at runtime.

## Download And Run Locally

### Option 1: Build From Source

```bash
git clone https://github.com/<your-username>/github-repo-control.git
cd github-repo-control
cargo run
```

### Option 2: Install Locally With Cargo

If you want the tool available as a local binary:

```bash
cargo install --path .
github-repo-control
```

## Offline Use

This app runs locally after it is installed, but it still needs internet access when it talks to GitHub.

If you want a version users can download and run without cloning the repository:

1. Build a release binary.
2. Share the binary with the user.
3. Have the user run it on their machine.

Example:

```bash
cargo build --release
```

The binary will be created at `target/release/github-repo-control`.

## How To Use

1. Start the app.
2. Enter your GitHub username.
3. Enter your GitHub token.
4. Press `Enter` to validate and load repositories.
5. Use `Up` and `Down` to move.
6. Press `Space` to select a repository.
7. Press `a` to select all.
8. Press `u` to clear selection.
9. Press `p` to make selected repositories private.
10. Press `q` to quit.

## Shipping A Release

The simplest shipping path is:

1. Build a release binary with `cargo build --release`.
2. Upload the binary as a GitHub release asset.
3. Tell users to download the asset and run it locally.

If you want a more polished release process later, add a GitHub Actions workflow to build and attach release artifacts automatically.

## Notes

- Only public repositories are shown in the dashboard.
- The app checks that the username and token match before entering the dashboard.
- If the username or token is wrong, the app stays on the login screen.
