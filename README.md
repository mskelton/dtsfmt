# dtsfmt

Auto formatter for device tree files.

## Installation

You can install dtsfmt by running the install script which will download
the [latest release](https://github.com/mskelton/dtsfmt/releases/latest).

```bash
curl -LSfs https://go.mskelton.dev/dtsfmt/install | sh
```

Or you can build from source.

```bash
git clone --recurse-submodules https://github.com/mskelton/dtsfmt.git
cd dtsfmt
cargo install --path .
```

## Usage

To run dtsfmt, simply provide a file/directory path to the `dtsfmt` command.

```bash
dtsfmt .
```

## Config

The following confirmation options are available for dtsfmt. Confirmation should
be added to a `.dtsfmtrc.toml` file at the root of your project.

```toml
layout = "kinesis:adv360"
```

## Ignoring code

You can add a `.dtsfmtignore` file at the root of your project to exclude files
and paths from formatting. This file follows the same rules as `.gitignore`.

## Flags

### `--check`

When you want to check if your files are formatted, you can run dtsfmt with
the `--check` flag (or `-c`). This will output a human-friendly message and a
list of unformatted files, if any.

```bash
dtsfmt --check .
```

### `--emit`

You can change the way dtsfmt emits the changes with the `--emit` flag.

```bash
dtsfmt --emit=stdout
```
