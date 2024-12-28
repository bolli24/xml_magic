# xml_magic

A resonably fast command-line XML formatter with proper indentation and comment preservation.

## Usage
```bash
xml_magic path/to/file.xml                    # Format with tabs (default)
xml_magic --stdout path/to/file.xml           # Output to stdout
xml_magic --indent 2space path/to/file.xml    # Use 2 spaces
xml_magic --indent 4space path/to/file.xml    # Use 4 spaces