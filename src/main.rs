use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::process::Command;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use std::str::FromStr;

#[derive(Debug)]
enum ExportType {
    Script,
    Image,
    Shape,
    MorphShape,
    Movie,
    Font,
    Font4,
    Frame,
    Sprite,
    Button,
    Sound,
    BinaryData,
    SymbolClass,
    Text,
    All,
    Fla,
    Xfl,
    Other(String),
}

impl FromStr for ExportType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "script" => Ok(ExportType::Script),
            "image" => Ok(ExportType::Image),
            "shape" => Ok(ExportType::Shape),
            "morphshape" => Ok(ExportType::MorphShape),
            "movie" => Ok(ExportType::Movie),
            "font" => Ok(ExportType::Font),
            "font4" => Ok(ExportType::Font4),
            "frame" => Ok(ExportType::Frame),
            "sprite" => Ok(ExportType::Sprite),
            "button" => Ok(ExportType::Button),
            "sound" => Ok(ExportType::Sound),
            "binarydata" => Ok(ExportType::BinaryData),
            "symbolclass" => Ok(ExportType::SymbolClass),
            "text" => Ok(ExportType::Text),
            "all" => Ok(ExportType::All),
            "fla" => Ok(ExportType::Fla),
            "xfl" => Ok(ExportType::Xfl),
            other => Ok(ExportType::Other(other.to_string())),
        }
    }
}

impl ExportType {
    fn as_str(&self) -> &str {
        match self {
            ExportType::Script => "script",
            ExportType::Image => "image",
            ExportType::Shape => "shape",
            ExportType::MorphShape => "morphshape",
            ExportType::Movie => "movie",
            ExportType::Font => "font",
            ExportType::Font4 => "font4",
            ExportType::Frame => "frame",
            ExportType::Sprite => "sprite",
            ExportType::Button => "button",
            ExportType::Sound => "sound",
            ExportType::BinaryData => "binaryData",
            ExportType::SymbolClass => "symbolClass",
            ExportType::Text => "text",
            ExportType::All => "all",
            ExportType::Fla => "fla",
            ExportType::Xfl => "xfl",
            ExportType::Other(s) => s.as_str(),
        }
    }
}

fn main() -> io::Result<()> {
    let matches = ClapCommand::new("swf-packer")
        .version("1.0")
        .author("bluisblu")
        .about("CLI for bulk-exporting SWF content using ffdec-cli recursively, maintaining the directory structure")
        .subcommand(ClapCommand::new("export")
            .about("Export content from SWF files")
            .arg(Arg::new("type")
                .help("Type of export (e.g., script, sound, image, etc.)")
                .required(true)
                .index(1)))
        .subcommand(ClapCommand::new("list-types")
            .about("List all available export types"))
        .get_matches();

    match matches.subcommand() {
        Some(("export", export_matches)) => {
            if let Some(export_type) = export_matches.get_one::<String>("type") {
                match ExportType::from_str(export_type) {
                    Ok(export_type) => {
                        let input_dir = std::env::current_dir()?.join("original");
                        let output_dir = std::env::current_dir()?.join("assets");

                        println!("Input directory: {:?}", input_dir);
                        println!("Output directory: {:?}", output_dir);

                        if !output_dir.exists() {
                            println!("Creating output directory: {:?}", output_dir);
                            fs::create_dir_all(&output_dir)?;
                        }

                        process_swf_files(&input_dir, &output_dir, &export_type)?;
                    }
                    Err(_) => {
                        eprintln!("Error: Unknown export type '{}'. Use 'swf-packer list-types' to see available types.", export_type);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Error: The 'export' subcommand requires a <type> argument.");
                std::process::exit(1);
            }
        }
        Some(("list-types", _)) => {
            println!("Available export types:");
            for export_type in &[
                "script", "image", "shape", "morphshape", "movie", "font", "font4",
                "frame", "sprite", "button", "sound", "binarydata", "symbolclass",
                "text", "all", "fla", "xfl"
            ] {
                println!("- {}", export_type);
            }
        }
        _ => {
            eprintln!("Error: No valid subcommand provided. Use 'swf-packer export <type>' or 'swf-packer list-types'.");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn process_swf_files(input_path: &Path, output_path: &Path, export_type: &ExportType) -> io::Result<()> {
    for entry in fs::read_dir(input_path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry.file_name();

        if entry_path.is_dir() {
            let current_output_path = output_path.join(file_name);
            if !current_output_path.exists() {
                fs::create_dir_all(&current_output_path)?;
            }
            process_swf_files(&entry_path, &current_output_path, export_type)?;
        } else if entry_path.extension() == Some(std::ffi::OsStr::new("swf")) {
            let mut output_file = output_path.join(file_name);
            output_file.set_extension(""); // Remove the .swf extension
            output_file.set_file_name(format!(
                "{}_swf",
                output_file.file_stem().unwrap().to_string_lossy()
            ));

            execute_ffdec_cli(&entry_path, &output_file, export_type)?;
        }
    }
    Ok(())
}

fn execute_ffdec_cli(input_file: &Path, output_file: &Path, export_type: &ExportType) -> io::Result<()> {
    println!("Executing: ffdec-cli -export {} {:?} {:?}", export_type.as_str(), output_file, input_file);

    let status = Command::new("ffdec-cli")
        .arg("-export")
        .arg(export_type.as_str())
        .arg(output_file)
        .arg(input_file)
        .status()?;

    if status.success() {
        println!("Successfully exported: {:?}", output_file);
    } else {
        eprintln!("Failed to export: {:?}", input_file);
    }

    Ok(())
}