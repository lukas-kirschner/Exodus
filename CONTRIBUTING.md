# Contributing to the project

## Bug Reports

If you find a bug in the game, feel free to open a bug report here in the issue tracker.
Please check if the bug has already been reported before opening a new issue.
I would appreciate if you added additional information (e.g. Commit hash, system information, etc.) to the bug report so
I can reproduce the bug.

## Feature Requests

If you have a great idea for a new feature that can be added to the game, or if you notice any room for improvement,
please feel free to open a feature request here in the issue tracker (using the "Enhancement" tag).

## Pull Requests

To contribute code to the project, please fork the repository, implement your own (stable) improvements, additions or
bug fixes on your fork and open a pull request for your fork here.

## Developer Information

To speed up compile times, run your debug builds with

```bash
cargo run --features bevy/dynamic_linking
```

and test builds with

```bash
cargo test --features bevy/dynamic_linking --workspace -- --include-ignored
```

## Code Conventions

To keep a well-formatted repository, we reject all pull requests that do not correspond to our formatting and code conventions automatically.

#### Formatting rules

The formatting rules are defined in [rustfmt's config file](rustfmt.toml). To automatically reformat the code, run

```bash
cargo fmt --all
```

#### Code Conventions

The code conventions are defined in the [Clippy Config File](clippy.toml). To check if there are any problems, run

```bash
cargo clippy -- -D warnings
```