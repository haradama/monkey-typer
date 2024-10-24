# Monkey Typer

Monkey Typer is a command-line tool designed to assist with live coding style performances. It allows you to record your keystrokes during a coding session and play them back later, simulating real-time typing. This can be useful for presentations, tutorials, or performances where you want to showcase code being written live without the risk of typos or mistakes.

## Features

- **Record Keystrokes**: Capture all your keystrokes during a coding session.
- **Playback Sessions**: Replay the recorded keystrokes to simulate live typing.
- **List Sessions**: View all saved recording sessions.
- **Cross-Platform**: Works on Windows, macOS, and Linux.

## Installation

To build Monkey Typer from source, you'll need to have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Clone the repository

```bash
git clone https://github.com/yourusername/monkey-typer.git
cd monkey-typer
```

### Build the project

```bash
cargo build --release
```

### (Optional) Install the binary

```bash
cargo install --path .
```

This will install `monkey-typer` into your Cargo binaries directory (usually `~/.cargo/bin`).

## Usage

Monkey Typer provides three main commands:

- `record`: Start a new recording session.
- `play`: Play back a recorded session.
- `list`: List all saved sessions.

### Recording a Session

To start recording a session:

```bash
monkey-typer record <session_name>
```

- Replace `<session_name>` with a name for your session.
- Press `Ctrl+C` to stop recording.

**Example:**

```bash
monkey-typer record my_live_coding_session
```

### Playing Back a Session

To play back a recorded session:

```bash
monkey-typer play <session_name>
```

- Replace `<session_name>` with the name of the session you want to play.

**Example:**

```bash
monkey-typer play my_live_coding_session
```

### Listing Saved Sessions

To list all saved recording sessions:

```bash
monkey-typer list
```

## Examples

### Record a Session

```bash
monkey-typer record hello_world
```

Start typing your code. When finished, press `Ctrl+C` to stop recording.

### Play Back a Session

```bash
monkey-typer play hello_world
```

The keystrokes from your recording will be replayed in the terminal.

### List Sessions

```bash
$ monkey-typer list

Saved sessions:
- hello_world
- my_live_coding_session
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
