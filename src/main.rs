use clap::{Parser, ValueEnum};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};
use xml::{reader::ParserConfig, writer::EmitterConfig};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Path to the input XML file
    #[arg(required = true)]
    path: PathBuf,

    /// Path to output file (optional)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output to stdout instead of modifying the file
    #[arg(long, default_value_t = false)]
    stdout: bool,

    /// Indentation style
    #[arg(value_enum, short, long, default_value = "tab")]
    indent: Indent,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Indent {
    Tab,
    Two,
    Four,
}

impl Indent {
    fn get_str(&self) -> &'static str {
        match self {
            Self::Tab => "\t",
            Self::Two => "  ",
            Self::Four => "    ",
        }
    }
}

fn format_xml(src: &[u8], indent: &Indent) -> Result<Vec<u8>, xml::reader::Error> {
    let mut dest = Vec::new();
    let reader = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(false)
        .create_reader(src);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .autopad_comments(false)
        .indent_string(indent.get_str())
        .create_writer(&mut dest);

    for event in reader {
        if let Some(event) = event?.as_writer_event() {
            writer.write(event).unwrap();
        }
    }
    Ok(dest)
}

fn main() {
    let cli = Cli::parse();

    // Validate that stdout and output path aren't used together
    if cli.stdout && cli.output.is_some() {
        eprintln!("Error: Cannot use both --stdout and --output options");
        std::process::exit(1);
    }

    // Check if input and output paths are the same
    if let Some(output_path) = &cli.output {
        if let Ok(input_canonical) = fs::canonicalize(&cli.path) {
            if let Ok(output_canonical) = fs::canonicalize(output_path) {
                if input_canonical == output_canonical {
                    eprintln!("Error: Input and output paths are the same file. Use no output path to modify the file in place, or specify a different output path.");
                    std::process::exit(1);
                }
            }
        }
    }

    let content = match fs::read(&cli.path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {}: {}", cli.path.display(), err);
            std::process::exit(1);
        }
    };

    match format_xml(&content, &cli.indent) {
        Ok(formatted) => {
            if cli.stdout {
                if let Err(err) = io::stdout().write_all(&formatted) {
                    eprintln!("Error writing to stdout: {err}");
                    std::process::exit(1);
                }
            } else if let Some(output_path) = cli.output {
                if let Err(err) = fs::write(&output_path, formatted) {
                    eprintln!("Error writing to file {}: {}", output_path.display(), err);
                    std::process::exit(1);
                }
                println!(
                    "Successfully formatted XML file to: {}",
                    output_path.display()
                );
            } else {
                if let Err(err) = fs::write(&cli.path, formatted) {
                    eprintln!("Error writing to file {}: {}", cli.path.display(), err);
                    std::process::exit(1);
                }
                println!("Successfully formatted XML file: {}", cli.path.display());
            }
        }
        Err(err) => {
            eprintln!("Error formatting XML: {err}");
            std::process::exit(1);
        }
    }
}
