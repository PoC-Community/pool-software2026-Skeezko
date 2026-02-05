use clap::Parser;
use std::fs;
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// String pattern to search for within the file
    pattern: String,

    /// Path to the file to be examined
    file: String,

    /// Ignore case distinctions in patterns and data
    #[arg(short = 'i', long = "case-insensitive")]
    case_insensitive: bool,

    /// Print line number with output lines
    #[arg(short = 'n', long = "line-numbers")]
    line_numbers: bool,
}


fn highlight_pattern(pattern: &String, flag_i: bool, line: &str) -> String {

    let color = pattern.red().bold().to_string();

    if flag_i {
        let lower_line = line.to_lowercase();
        let lower_pattern = pattern.to_lowercase();
        let mut res = String::new();
        let mut end = 0;

        for (start, _) in lower_line.match_indices(&lower_pattern) {
            let full = start + pattern.len();
            res.push_str(&line[end..start]);
            let occurence = line[start..full].red().bold().to_string();
            res.push_str(&occurence);
            end = full
        }
        res.push_str(&line[end..]);
        res
    } else {
        line.replace(&*pattern, &color)
    }
}


fn search_in_file(args: Args){

    let content = fs::read_to_string(args.file).expect("Failed to read file");

    for (num, line) in content.lines().enumerate() {
        if args.case_insensitive {
            if line.to_lowercase().contains(&args.pattern.to_lowercase()) {
                if args.line_numbers {
                    println!("{}: {}", num + 1, highlight_pattern(&args.pattern, args.case_insensitive, line))
                } else {
                    println!("{}", highlight_pattern(&args.pattern, args.case_insensitive, line));
                }
            }
        } else {
            if line.contains(&args.pattern) {
                if args.line_numbers {
                    println!("{}: {}", num + 1, highlight_pattern(&args.pattern, args.case_insensitive, line));
                } else {
                    println!("{}", highlight_pattern(&args.pattern, args.case_insensitive, line))
                }
            }
        }
    }
}


fn main() {
    let _args = Args::parse();
    search_in_file(_args);
}