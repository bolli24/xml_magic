use clap::Parser;
use std::fs;
use xml::{reader::ParserConfig, writer::EmitterConfig};

#[derive(Parser)]
#[command(name = "xml_magic")]
#[command(about = "A tool for XML processing")]
struct Cli {
    /// Path to the XML file
    #[arg(required = true)]
    path: std::path::PathBuf,
}

fn format_xml(src: &[u8]) -> Result<String, xml::reader::Error> {
    let mut dest = Vec::new();
    let reader = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(false)
        .create_reader(src);
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .indent_string("\t")
        .normalize_empty_elements(false)
        .autopad_comments(false)
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

    // Read the file contents
    let content = match fs::read(&cli.path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {}: {}", cli.path.display(), err);
            std::process::exit(1);
        }
    };

    // Format the XML
    match format_xml(&content) {
        Ok(formatted) => {
            // Write the formatted XML back to the file
            if let Err(err) = fs::write(&cli.path, formatted) {
                eprintln!("Error writing to file {}: {}", cli.path.display(), err);
                std::process::exit(1);
            }
            println!("Successfully formatted XML file: {}", cli.path.display());
        }
        Err(err) => {
            eprintln!("Error formatting XML: {}", err);
            std::process::exit(1);
        }
    }
}
