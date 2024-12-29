# xml_magic

A reasonably fast command-line XML formatter with proper indentation and comment preservation.

## Installation
### Using cargo-binstall (prebuilt binaries)
```bash
cargo binstall xml_magic
```

### From crates.io
```bash
cargo install xml_magic
```

### From source
```bash
# Clone the repository
git clone https://github.com/bolli24/xml_magic
cd xml_magic

# Build and install
cargo install --path .
```

## Features

- Fast XML formatting with customizable indentation
- Preserves comments and structure
- Multiple output options (in-place, stdout, or new file)
- Support for tabs or space-based indentation

## Usage

```bash
# Format in-place using tabs (default)
xml_magic path/to/file.xml

# Output to a different file
xml_magic path/to/file.xml --output path/to/output.xml

# Output to stdout instead of modifying files
xml_magic --stdout path/to/file.xml

# Customize indentation style
xml_magic --indent tab path/to/file.xml     # Use tabs (default)
xml_magic --indent two path/to/file.xml     # Use 2 spaces
xml_magic --indent four path/to/file.xml    # Use 4 spaces
```