
# Preg - Pattern Recognition Command-line Utility

[![GitHub license](https://img.shields.io/github/license/sonirico/preg)](https://github.com/sonirico/preg/blob/main/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/sonirico/preg)](https://github.com/sonirico/preg/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/sonirico/preg)](https://github.com/sonirico/preg/issues)

Preg is a command-line utility that allows you to perform pattern recognition in files or standard input. It's designed to help you find and highlight specific patterns within text data.

## Features

- Search for patterns within files or standard input.
- Highlight matched patterns for easy recognition.
- Case-sensitive and case-insensitive search options.
- Specify multiple patterns to search for simultaneously.

## Installation

Preg is built using Rust and the Tokio asynchronous runtime. To install and use it, you need to have Rust and Cargo installed on your system. If you don't have them, you can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

1. Clone the repository:

   ```sh
   git clone https://github.com/sonirico/preg.git
   ```

2. Navigate to the `preg` directory:

   ```sh
   cd preg
   ```

3. Build the utility:

   ```sh
   cargo build --release
   ```

4. Run preg with the desired search pattern:

   ```sh
   ./target/release/preg -m "pattern" -f [file]
   ```

  ```sh
  cat file | ./target/release/preg -m "pattern"
   ```

## Usage

```
Usage: preg [OPTIONS] PATTERN [FILE]
  Preg - Pattern Recognition Command-line Utility

Positional arguments:
  PATTERN       The pattern to search for
  FILE          The input file (optional, read from stdin if not provided)

Options:
  -i, --ignore-case    Perform case-insensitive search
  -h, --help           Print this help message
  -v, --version        Print version information
```

## Examples

Search for a pattern within a file:

```sh
./target/release/preg -m "example" -f input.txt
```

Search for a case-insensitive pattern within standard input:

```sh
echo "Example input" | ./target/release/preg -m "example"
```

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, feel free to [open an issue](https://github.com/sonirico/preg/issues) or submit a pull request.

## Acknowledgements

Preg was inspired by the `grep` command-line utility.

---

**Disclaimer:** This utility is provided as-is, and the author takes no responsibility for its use or misuse. Use it responsibly and respect copyright and privacy laws.

For questions, feedback, or support, please [open an issue](https://github.com/sonirico/preg/issues).
```

You can copy and paste this content into your GitHub repository's README.md file.