use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinePreview(pub usize, pub String, pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub filename: String,
    pub rel_path: String,
    pub abs_path: String,
    pub path: String,
    pub total_lines: usize,
    pub empty_lines: usize,
    pub content_lines: usize,
    pub pure_empty_lines: usize,
    pub whitespace_only_lines: usize,
    pub empty_pct: f64,
    pub content_pct: f64,
    pub max_line_len: usize,
    pub max_line_num: usize,
    pub avg_line_len: f64,
    pub char_count: usize,
    pub word_count: usize,
    pub file_size_bytes: u64,
    pub file_size_formatted: String,
    pub ends_with_newline: bool,
    pub detected_newline: String,
    pub is_binary: bool,
    pub preview_lines: Vec<LinePreview>,
    pub preview_truncated: bool,
    pub total_preview_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectorySummary {
    pub target_directory: String,
    pub total_files: usize,
    pub total_lines: usize,
    pub total_empty_lines: usize,
    pub total_content_lines: usize,
    pub total_pure_empty_lines: usize,
    pub total_whitespace_only_lines: usize,
    pub overall_empty_pct: f64,
    pub overall_content_pct: f64,
    pub total_size_bytes: u64,
    pub total_size_formatted: String,
    pub total_empty: usize,
    pub total_content: usize,
    pub files: Vec<FileStats>,
}

pub fn format_file_size(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{} B", bytes)
    } else {
        format!("{:.1} {}", size, UNITS[unit_idx])
    }
}

pub fn is_binary_extension(ext: &str) -> bool {
    matches!(
        ext,
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "webp" | "svgz" | "tiff" | "psd"
            | "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" | "m4v"
            | "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a"
            | "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "tgz"
            | "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx"
            | "exe" | "dll" | "so" | "dylib" | "bin" | "iso" | "img" | "apk"
            | "woff" | "woff2" | "ttf" | "eot" | "otf"
            | "pyc" | "pyd" | "class" | "o" | "a" | "obj" | "lib"
            | "sqlite" | "db" | "wasm"
    )
}

pub fn is_binary_buffer(bytes: &[u8]) -> bool {
    let check_len = bytes.len().min(1024);
    bytes[..check_len].contains(&0)
}

pub fn count_file_lines(
    path: &Path,
    rel_path_override: Option<&str>,
    include_trailing_empty: bool,
    treat_whitespace_as_empty: bool,
    with_preview: bool,
) -> Result<FileStats, String> {
    let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
    let file_size_bytes = metadata.len();
    let abs_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let filename = path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let rel_path = rel_path_override
        .map(|s| s.to_string())
        .unwrap_or_else(|| filename.clone());

    let abs_path_str = abs_path.to_string_lossy().to_string();

    if file_size_bytes == 0 {
        return Ok(FileStats {
            filename,
            rel_path,
            abs_path: abs_path_str.clone(),
            path: abs_path_str,
            total_lines: 0,
            empty_lines: 0,
            content_lines: 0,
            pure_empty_lines: 0,
            whitespace_only_lines: 0,
            empty_pct: 0.0,
            content_pct: 0.0,
            max_line_len: 0,
            max_line_num: 0,
            avg_line_len: 0.0,
            char_count: 0,
            word_count: 0,
            file_size_bytes: 0,
            file_size_formatted: "0 B".to_string(),
            ends_with_newline: false,
            detected_newline: "None".to_string(),
            is_binary: false,
            preview_lines: Vec::new(),
            preview_truncated: false,
            total_preview_count: 0,
            error: None,
        });
    }

    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        if is_binary_extension(&ext.to_lowercase()) {
            return Ok(FileStats {
                filename,
                rel_path,
                abs_path: abs_path_str.clone(),
                path: abs_path_str,
                total_lines: 0,
                empty_lines: 0,
                content_lines: 0,
                pure_empty_lines: 0,
                whitespace_only_lines: 0,
                empty_pct: 0.0,
                content_pct: 0.0,
                max_line_len: 0,
                max_line_num: 0,
                avg_line_len: 0.0,
                char_count: 0,
                word_count: 0,
                file_size_bytes,
                file_size_formatted: format_file_size(file_size_bytes),
                ends_with_newline: false,
                detected_newline: "None (Binary)".to_string(),
                is_binary: true,
                preview_lines: Vec::new(),
                preview_truncated: false,
                total_preview_count: 0,
                error: None,
            });
        }
    }

    let buffer = fs::read(path).map_err(|e| e.to_string())?;
    if is_binary_buffer(&buffer) {
        return Ok(FileStats {
            filename,
            rel_path,
            abs_path: abs_path_str.clone(),
            path: abs_path_str,
            total_lines: 0,
            empty_lines: 0,
            content_lines: 0,
            pure_empty_lines: 0,
            whitespace_only_lines: 0,
            empty_pct: 0.0,
            content_pct: 0.0,
            max_line_len: 0,
            max_line_num: 0,
            avg_line_len: 0.0,
            char_count: 0,
            word_count: 0,
            file_size_bytes,
            file_size_formatted: format_file_size(file_size_bytes),
            ends_with_newline: false,
            detected_newline: "None (Binary)".to_string(),
            is_binary: true,
            preview_lines: Vec::new(),
            preview_truncated: false,
            total_preview_count: 0,
            error: None,
        });
    }

    let ends_with_newline = buffer.ends_with(b"\n") || buffer.ends_with(b"\r");
    let text = String::from_utf8_lossy(&buffer);

    // Detect newline format
    let crlf_count = text.matches("\r\n").count();
    let lf_count = text.matches('\n').count().saturating_sub(crlf_count);
    let cr_count = text.matches('\r').count().saturating_sub(crlf_count);
    let detected_newline = if crlf_count > lf_count && crlf_count > cr_count {
        "CRLF (Windows)"
    } else if lf_count >= crlf_count && lf_count >= cr_count && lf_count > 0 {
        "LF (Unix/Mac)"
    } else if cr_count > 0 {
        "CR (Legacy Mac)"
    } else {
        "None"
    }
    .to_string();

    let mut raw_lines: Vec<&str> = text
        .split("\r\n")
        .flat_map(|s| s.split('\n'))
        .flat_map(|s| s.split('\r'))
        .collect();

    if ends_with_newline && !include_trailing_empty && !raw_lines.is_empty() && raw_lines.last() == Some(&"") {
        raw_lines.pop();
    }

    let total_lines = raw_lines.len();
    let mut pure_empty_lines = 0;
    let mut whitespace_only_lines = 0;
    let mut max_line_len = 0;
    let mut max_line_num = 0;
    let mut word_count = 0;
    let mut char_count = 0;
    let mut preview_lines = if with_preview {
        Vec::with_capacity(total_lines)
    } else {
        Vec::new()
    };

    for (i, line) in raw_lines.iter().enumerate() {
        let line_num = i + 1;
        let line_len = line.chars().count();
        char_count += line_len;

        if line_len > max_line_len {
            max_line_len = line_len;
            max_line_num = line_num;
        }

        if !line.is_empty() {
            word_count += line.split_whitespace().count();
        }

        let is_empty = line.is_empty();
        let is_ws = !is_empty && line.chars().all(|c| c.is_whitespace());

        if is_empty {
            pure_empty_lines += 1;
        } else if is_ws && treat_whitespace_as_empty {
            whitespace_only_lines += 1;
        }

        if with_preview {
            let line_type = if is_empty {
                "empty"
            } else if is_ws {
                if treat_whitespace_as_empty {
                    "whitespace"
                } else {
                    "content"
                }
            } else {
                "content"
            };
            preview_lines.push(LinePreview(line_num, line.to_string(), line_type.to_string()));
        }
    }

    let empty_lines = pure_empty_lines + whitespace_only_lines;
    let content_lines = total_lines.saturating_sub(empty_lines);
    let empty_pct = if total_lines > 0 {
        ((empty_lines as f64 / total_lines as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    };
    let content_pct = if total_lines > 0 {
        ((content_lines as f64 / total_lines as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    };
    let avg_line_len = if total_lines > 0 {
        ((char_count as f64 / total_lines as f64) * 10.0).round() / 10.0
    } else {
        0.0
    };

    let total_preview_count = if with_preview { preview_lines.len() } else { 0 };

    Ok(FileStats {
        filename,
        rel_path,
        abs_path: abs_path_str.clone(),
        path: abs_path_str,
        total_lines,
        empty_lines,
        content_lines,
        pure_empty_lines,
        whitespace_only_lines,
        empty_pct,
        content_pct,
        max_line_len,
        max_line_num,
        avg_line_len,
        char_count,
        word_count,
        file_size_bytes,
        file_size_formatted: format_file_size(file_size_bytes),
        ends_with_newline,
        detected_newline,
        is_binary: false,
        total_preview_count,
        preview_lines,
        preview_truncated: false,
        error: None,
    })
}

pub fn count_text_lines(
    text: &str,
    filename: &str,
    include_trailing_empty: bool,
    treat_whitespace_as_empty: bool,
    with_preview: bool,
) -> FileStats {
    let file_size_bytes = text.len() as u64;
    let ends_with_newline = text.ends_with('\n') || text.ends_with('\r');

    let crlf_count = text.matches("\r\n").count();
    let lf_count = text.matches('\n').count().saturating_sub(crlf_count);
    let cr_count = text.matches('\r').count().saturating_sub(crlf_count);
    let detected_newline = if crlf_count > lf_count && crlf_count > cr_count {
        "CRLF (Windows)"
    } else if lf_count >= crlf_count && lf_count >= cr_count && lf_count > 0 {
        "LF (Unix/Mac)"
    } else if cr_count > 0 {
        "CR (Legacy Mac)"
    } else {
        "None"
    }
    .to_string();

    let mut raw_lines: Vec<&str> = text
        .split("\r\n")
        .flat_map(|s| s.split('\n'))
        .flat_map(|s| s.split('\r'))
        .collect();

    if ends_with_newline && !include_trailing_empty && !raw_lines.is_empty() && raw_lines.last() == Some(&"") {
        raw_lines.pop();
    }

    let total_lines = raw_lines.len();
    let mut pure_empty_lines = 0;
    let mut whitespace_only_lines = 0;
    let mut max_line_len = 0;
    let mut max_line_num = 0;
    let mut word_count = 0;
    let mut char_count = 0;
    let mut preview_lines = if with_preview {
        Vec::with_capacity(total_lines)
    } else {
        Vec::new()
    };

    for (i, line) in raw_lines.iter().enumerate() {
        let line_num = i + 1;
        let line_len = line.chars().count();
        char_count += line_len;

        if line_len > max_line_len {
            max_line_len = line_len;
            max_line_num = line_num;
        }

        if !line.is_empty() {
            word_count += line.split_whitespace().count();
        }

        let is_empty = line.is_empty();
        let is_ws = !is_empty && line.chars().all(|c| c.is_whitespace());

        if is_empty {
            pure_empty_lines += 1;
        } else if is_ws && treat_whitespace_as_empty {
            whitespace_only_lines += 1;
        }

        if with_preview {
            let line_type = if is_empty {
                "empty"
            } else if is_ws {
                if treat_whitespace_as_empty {
                    "whitespace"
                } else {
                    "content"
                }
            } else {
                "content"
            };
            preview_lines.push(LinePreview(line_num, line.to_string(), line_type.to_string()));
        }
    }

    let empty_lines = pure_empty_lines + whitespace_only_lines;
    let content_lines = total_lines.saturating_sub(empty_lines);
    let empty_pct = if total_lines > 0 {
        ((empty_lines as f64 / total_lines as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    };
    let content_pct = if total_lines > 0 {
        ((content_lines as f64 / total_lines as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    };
    let avg_line_len = if total_lines > 0 {
        ((char_count as f64 / total_lines as f64) * 10.0).round() / 10.0
    } else {
        0.0
    };

    let total_preview_count = if with_preview { preview_lines.len() } else { 0 };

    FileStats {
        filename: filename.to_string(),
        rel_path: filename.to_string(),
        abs_path: String::new(),
        path: String::new(),
        total_lines,
        empty_lines,
        content_lines,
        pure_empty_lines,
        whitespace_only_lines,
        empty_pct,
        content_pct,
        max_line_len,
        max_line_num,
        avg_line_len,
        char_count,
        word_count,
        file_size_bytes,
        file_size_formatted: format_file_size(file_size_bytes),
        ends_with_newline,
        detected_newline,
        is_binary: false,
        total_preview_count,
        preview_lines,
        preview_truncated: false,
        error: None,
    }
}

pub fn scan_directory_tree(
    dir_path: &Path,
    recursive: bool,
    include_trailing_empty: bool,
    treat_whitespace_as_empty: bool,
) -> Result<DirectorySummary, String> {
    if !dir_path.exists() {
        return Err(format!("Directory does not exist: {}", dir_path.display()));
    }

    let skip_dirs: HashSet<&'static str> = [
        ".git", ".svn", ".hg", "node_modules", "target", "build", "dist",
        ".venv", "venv", "__pycache__", ".idea", ".vscode", ".tox", ".cache",
    ]
    .iter()
    .cloned()
    .collect();

    let mut walker = WalkDir::new(dir_path);
    if !recursive {
        walker = walker.max_depth(1);
    }

    let entries: Vec<PathBuf> = walker
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy();
                !skip_dirs.contains(name.as_ref())
            } else {
                true
            }
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect();

    let base_canonical = dir_path.canonicalize().unwrap_or_else(|_| dir_path.to_path_buf());

    // Process files concurrently with Rayon (no preview allocation for bulk speed)
    let files: Vec<FileStats> = entries
        .par_iter()
        .filter_map(|path| {
            let rel = path
                .strip_prefix(&base_canonical)
                .ok()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| {
                    path.file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_default()
                });

            count_file_lines(
                path,
                Some(&rel),
                include_trailing_empty,
                treat_whitespace_as_empty,
                false, // with_preview = false for lightning directory scans
            )
            .ok()
        })
        .collect();

    let total_files = files.len();
    let mut total_lines = 0;
    let mut total_empty_lines = 0;
    let mut total_pure_empty_lines = 0;
    let mut total_whitespace_only_lines = 0;
    let mut total_size_bytes = 0;

    for f in &files {
        total_lines += f.total_lines;
        total_empty_lines += f.empty_lines;
        total_pure_empty_lines += f.pure_empty_lines;
        total_whitespace_only_lines += f.whitespace_only_lines;
        total_size_bytes += f.file_size_bytes;
    }

    let total_content_lines = total_lines.saturating_sub(total_empty_lines);
    let overall_empty_pct = if total_lines > 0 {
        ((total_empty_lines as f64 / total_lines as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    };
    let overall_content_pct = if total_lines > 0 {
        ((total_content_lines as f64 / total_lines as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    };

    Ok(DirectorySummary {
        target_directory: dir_path.to_string_lossy().to_string(),
        total_files,
        total_lines,
        total_empty_lines,
        total_content_lines,
        total_pure_empty_lines,
        total_whitespace_only_lines,
        overall_empty_pct,
        overall_content_pct,
        total_size_bytes,
        total_size_formatted: format_file_size(total_size_bytes),
        total_empty: total_empty_lines,
        total_content: total_content_lines,
        files,
    })
}
