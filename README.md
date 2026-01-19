# Donetick TUI

A terminal user interface (TUI) application for managing tasks with your Donetick server. Built with Rust and Ratatui.

## Features

- View all tasks from your Donetick server
- Add new tasks with name and due date
- Complete tasks directly from the terminal
- Keyboard-driven navigation (vim-style supported)
- Real-time updates from the server
- Clean, intuitive interface

## Prerequisites

- Rust 1.70 or later
- A running Donetick server instance
- Access token from your Donetick server

## Installation

1. Clone or download this repository
2. Navigate to the project directory:
   ```bash
   cd donetick-tui
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

4. The binary will be available at `target/release/donetick-tui`

## Configuration

1. Create a `.env` file in the project root (or set environment variables):
   ```bash
   cp .env.example .env
   ```

2. Edit the `.env` file with your Donetick server details:
   ```
   DONETICK_URL=https://your-donetick-instance.com
   DONETICK_TOKEN=your-access-token-here
   ```

### Getting Your Access Token

1. Log in to your Donetick web interface
2. Navigate to Settings → Advanced Settings → Access Token
3. Generate a new access token
4. Copy the token to your `.env` file

## Usage

Run the application:
```bash
cargo run
```

Or if you've built the release version:
```bash
./target/release/donetick-tui
```

### Keyboard Shortcuts

#### Task List View
- `↑` or `k` - Move selection up
- `↓` or `j` - Move selection down
- `Enter` - Complete selected task
- `a` - Add new task
- `r` - Refresh task list
- `q` - Quit application

#### Add Task Form
- `Tab` - Move to next field
- `Shift+Tab` - Move to previous field
- `Enter` - Submit form
- `Esc` - Cancel and return to task list
- `Backspace` - Delete character
- Type normally to enter text

#### Error Dialog
- Any key - Dismiss error and return

## Development

### Project Structure

```
donetick-tui/
├── src/
│   ├── main.rs          # Entry point and event loop
│   ├── app.rs           # Application state management
│   ├── ui.rs            # UI rendering logic
│   ├── event.rs         # Event handling
│   ├── config.rs        # Configuration management
│   └── api/
│       ├── mod.rs       # API module exports
│       ├── client.rs    # HTTP client
│       ├── types.rs     # Data types
│       └── error.rs     # Error types
├── Cargo.toml
├── .env.example
└── README.md
```

### Running in Development Mode

```bash
cargo run
```

### Building for Release

```bash
cargo build --release
```

## Troubleshooting

### "DONETICK_URL environment variable not set"
Make sure you have created a `.env` file with your configuration, or set the environment variables directly.

### "Network error: connection refused"
- Check that your Donetick server is running
- Verify the URL in your `.env` file is correct
- Ensure you're using `https://` for production servers or `http://` for local development

### "Server error (401): Unauthorized"
Your access token may be invalid or expired. Generate a new token from your Donetick web interface.

### Terminal appears broken after crash
If the application crashes and your terminal is in a bad state, run:
```bash
reset
```

## License

This project is provided as-is for use with Donetick servers.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## Resources

- [Donetick Documentation](https://docs.donetick.com/)
- [Donetick GitHub Repository](https://github.com/donetick/donetick)
- [Ratatui Documentation](https://ratatui.rs/)
