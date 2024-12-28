use clap::Parser;
use std::fs;
use xml::{reader::ParserConfig, writer::EmitterConfig};

#[derive(Parser)]
#[command(name = "xml_magic")]
#[command(about = "A reasonably fast XML formatter")]
struct Cli {
    /// Path to the XML file
    #[arg(required = true)]
    path: std::path::PathBuf,

    /// Output to stdout instead of modifying the file
    #[arg(long, default_value_t = false)]
    stdout: bool,

    /// Indentation style: 'tab', '2space', or '4space'
    #[arg(long, default_value = "tab")]
    indent: String,
}

fn format_xml(src: &[u8], indent: &str) -> Result<String, xml::reader::Error> {
    let mut dest = Vec::new();
    let reader = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(false)
        .create_reader(src);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .autopad_comments(false)
        .indent_string(match indent {
            "2space" => "  ",
            "4space" => "    ",
            _ => "\t",
        })
        .create_writer(&mut dest);
    for event in reader {
        if let Some(event) = event?.as_writer_event() {
            writer.write(event).unwrap();
        }
    }
    Ok(String::from_utf8(dest).unwrap())
}

fn main() {
    let cli = Cli::parse();

    if !["tab", "2space", "4space"].contains(&cli.indent.as_str()) {
        eprintln!("Invalid indent option. Use 'tab', '2space', or '4space'");
        std::process::exit(1);
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
                println!("{}", formatted);
            } else {
                if let Err(err) = fs::write(&cli.path, formatted) {
                    eprintln!("Error writing to file {}: {}", cli.path.display(), err);
                    std::process::exit(1);
                }
                eprintln!("Successfully formatted XML file: {}", cli.path.display());
            }
        }
        Err(err) => {
            eprintln!("Error formatting XML: {}", err);
            std::process::exit(1);
        }
    }
}
