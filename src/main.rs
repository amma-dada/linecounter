mod counter;
use counter::{count_file_lines, scan_directory_tree};
use std::env;
use std::path::Path;
use std::process;

fn print_file_stats(path_str: &str, editor_mode: bool, strict_empty: bool) {
    let p = Path::new(path_str);
    if !p.exists() {
        eprintln!("Error: Target '{}' not found.", path_str);
        process::exit(1);
    }

    match count_file_lines(p, None, editor_mode, !strict_empty, true) {
        Ok(res) => {
            println!("{}", "=".repeat(60));
            println!(" 📄 File: {}", path_str);
            println!("{}", "=".repeat(60));
            println!(" TOTAL LINES (All):   {:>10}  (Empty lines included)", res.total_lines);
            println!(" Empty Lines:         {:>10}  (Pure: {}, Whitespace-only: {})", res.empty_lines, res.pure_empty_lines, res.whitespace_only_lines);
            println!(" Content Lines:       {:>10}", res.content_lines);
            println!(" File Size:           {:>10} ({} bytes)", res.file_size_formatted, res.file_size_bytes);
            println!(" Total Words:         {:>10}", res.word_count);
            println!(" Total Characters:    {:>10}", res.char_count);
            println!(" Detected Line End:   {}", res.detected_newline);
            println!("{}", "=".repeat(60));
        }
        Err(e) => {
            eprintln!("Error counting file: {}", e);
            process::exit(1);
        }
    }
}

fn print_dir_stats(path_str: &str, recursive: bool, editor_mode: bool, strict_empty: bool) {
    let p = Path::new(path_str);
    if !p.exists() {
        eprintln!("Error: Target directory '{}' not found.", path_str);
        process::exit(1);
    }

    println!("{}", "=".repeat(65));
    println!(" 📁 Scanning Directory: {} (Recursive: {})", path_str, recursive);
    println!("{}", "=".repeat(65));
    println!(" {:<35} {:>8} {:>8} {:>8} {:>8}", "File / Relative Path", "Total", "Empty", "Content", "Size");
    println!("{}", "-".repeat(65));

    match scan_directory_tree(p, recursive, editor_mode, !strict_empty) {
        Ok(summary) => {
            for f in &summary.files {
                let short_name = if f.rel_path.len() <= 35 {
                    f.rel_path.clone()
                } else {
                    format!("...{}", &f.rel_path[f.rel_path.len() - 32..])
                };
                println!(" {:<35} {:>8} {:>8} {:>8} {:>8}", short_name, f.total_lines, f.empty_lines, f.content_lines, f.file_size_formatted);
            }
            println!("{}", "=".repeat(65));
            println!(
                " Scanned {} files | Total Lines: {} | Empty: {} | Content: {}",
                summary.total_files, summary.total_lines, summary.total_empty_lines, summary.total_content_lines
            );
            println!("{}", "=".repeat(65));
        }
        Err(e) => {
            eprintln!("Error scanning directory: {}", e);
            process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("LineCounter 1.0 (Rust SIMD Engine)");
        println!("Usage: {} <file-or-dir> [--no-recursive] [--editor-mode] [--strict-empty]", args[0]);
        println!("Examples:");
        println!("  {} test_sample.txt", args[0]);
        println!("  {} .", args[0]);
        return;
    }

    let target = &args[1];
    let recursive = !args.contains(&"--no-recursive".to_string());
    let editor_mode = args.contains(&"--editor-mode".to_string());
    let strict_empty = args.contains(&"--strict-empty".to_string());

    let p = Path::new(target);
    if p.is_dir() {
        print_dir_stats(target, recursive, editor_mode, strict_empty);
    } else {
        print_file_stats(target, editor_mode, strict_empty);
    }
}
