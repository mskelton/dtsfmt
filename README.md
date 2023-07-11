# dtsfmt

Auto formatter for device tree files.

## Installation

You can install dtsfmt by running the install script which will download
the [latest release](https://github.com/mskelton/flashlight/releases/latest).

```bash
curl -LSfs https://mskelton.dev/flashlight/install | sh
```

Or you can build from source.

```bash
git clone git@github.com:mskelton/flashlight.git
cd flashlight
cargo install --path .
```

## Usage

### Find all imports

To find all imports for a given import source (e.g., `react`), run flashlight
with just the `--source` command.

```bash
flashlight --source react
```

### Find imported symbols by name

Finding all imports of a given source is useful, but more useful is to search
for specific symbols.

```bash
flashlight --source react --name useState
```

## Change working directory

By default, flashlight uses the current working directory to search. You can
change the working directory using the `--cwd` argument.

```bash
flashlight --source react --cwd ./packages/a
```

### Format

You can customize the output format based on your use case. The supported
formats are:

- `default` - The default console format
- `json` - Formats the output as JSON
- `quickfix` - Formats the output as a Vim quickfix list (alias `vi`)

```bash
flashlight --source react --format json
```
