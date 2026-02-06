use std::fs;
use clap::Parser;
use walkdir::WalkDir;
use std::os::unix::fs::MetadataExt;
use colored::Colorize;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File name pattern to search
   name: String,

   /// Optional flag to filter by file extension
   #[arg(short = 't', long = "type")]
   extension: Option<String>,

   /// Optional flag to specify the starting directory
   #[arg(short = 'd', long = "dir", default_value = ".")]
   directory: String,

   /// Optional flag to search for files containing the pattern in their content (e.g., rs)
   #[arg(short = 'c', long = "content")]
   pattern: Option<String>
}


fn main() {
    let _args = Args::parse();
    
    for entries in WalkDir::new(_args.directory) {
        let entry = match entries {
            Ok(ok) => ok,
            Err(_) => continue
        };
        if entry.file_type().is_file() {
            let file = entry.file_name().to_string_lossy();
            if file.contains(&_args.name) {
                let found = entry.path().to_string_lossy().to_string();
                let (before, _) = found.split_once(&*file).unwrap();
                let colored_path = before.bright_white();
                let colored_file = file.yellow().bold();
                let path = "Path:";
                let metadata = fs::metadata(entry.path()).unwrap();
                let size = "Size:";
                if _args.extension.is_some() {
                    let ext = _args.extension.as_deref();
                    if entry.path().extension().and_then(|s| s.to_str()) == ext {
                        if let Some((name, extension)) = file.rsplit_once('.'){
                            let colored_file = name.yellow().bold();
                            let colored_extension = extension.red().bold();
                            println!("{} {}{}.{}  {} {}", path.green().italic().bold(), colored_path, colored_file, colored_extension, size.cyan().italic().bold(), metadata.size().to_string().bright_blue().italic());
                        }
                    }
                } else {
                    println!("{} {}{}  {} {}", path.green().italic().bold(), colored_path, colored_file, size.cyan().italic().bold(), metadata.size().to_string().bright_blue().italic());
                }
            }
        } else {
            continue;
        }
    }
}