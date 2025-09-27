# Rust Web App

A simple web application built with `tiny_http` that displays a welcome message and Ferris the crab. This is a test web app for upcoming project `isable-cli`

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

## Getting Started

1. Clone the repository
2. Navigate to the project directory
3. Run the application:

```bash
# Build the project
cargo build

# Run the application
cargo run
```

The application will be available at `http://localhost:5000` by default

## Configuration

The application uses a `.env` file for configuration. Create a `.env` file in the project root with:

```
PORT=5000
```

Available environment variables:
- `PORT` - The port number the server will listen on (default: 5000)

## Project Structure

- `src/main.rs` - Main application code and HTML/CSS
- `Cargo.toml` - Project dependencies and metadata
- `.env` - Environment variables (create this file)

## Dependencies

- `tiny_http` - Minimal HTTP server implementation
- `dotenv` - Environment variable management

## Development

To restart the server after making changes, press `Ctrl+C` to stop the current instance and run `cargo run` again.

## License

This project is open source and available under the [MIT License](LICENSE).
- env_logger - Logging
- log - Logging facade

## License

This project is open source and available under the MIT License.
