use ariadne::{Color, Label, Report, ReportKind, Source};

enum Severity {
    Error,
    Warning,
    Note,
}

struct Diagnostic {
    file: String,
    line: usize,     // 1-indexed
    col: usize,      // byte offset within the line (from ^ pointer)
    span_len: usize, // number of consecutive ^ chars
    severity: Severity,
    message: String,
    details: Vec<String>,
}

/// Parses javac output and renders each diagnostic via ariadne to stderr.
pub fn render_javac_errors(output: &str) {
    for diag in parse(output) {
        print_diagnostic(&diag);
    }
}

// ── Parser ────────────────────────────────────────────────────────────────────

fn parse(output: &str) -> Vec<Diagnostic> {
    let lines: Vec<&str> = output.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        match parse_header(lines[i]) {
            Some(mut diag) => {
                i += 1;
                // Source code snippet (not a header, not the ^ pointer line)
                if i < lines.len()
                    && !is_header(lines[i])
                    && !lines[i].trim_start().starts_with('^')
                {
                    i += 1;
                }
                // Pointer line: "        ^^^^^^^^^"
                if i < lines.len() && lines[i].trim_start().starts_with('^') {
                    let ptr = lines[i];
                    diag.col = ptr.find('^').unwrap_or(0);
                    diag.span_len =
                        ptr[diag.col..].chars().take_while(|&c| c == '^').count().max(1);
                    i += 1;
                }
                // Detail lines: "symbol: ...", "location: ..."
                while i < lines.len() {
                    let t = lines[i].trim();
                    if t.starts_with("symbol:") || t.starts_with("location:") {
                        diag.details.push(t.to_string());
                        i += 1;
                    } else {
                        break;
                    }
                }
                result.push(diag);
            }
            None => {
                i += 1;
            }
        }
    }
    result
}

/// Parses a javac diagnostic header: `<file>:<line>: <severity>: <message>`
fn parse_header(line: &str) -> Option<Diagnostic> {
    let (severity, tag) = if line.contains(": error:") {
        (Severity::Error, ": error:")
    } else if line.contains(": warning:") {
        (Severity::Warning, ": warning:")
    } else if line.contains(": note:") {
        (Severity::Note, ": note:")
    } else {
        return None;
    };

    let tag_idx = line.find(tag)?;
    let prefix = &line[..tag_idx]; // "path/to/File.java:10"
    let message = line[tag_idx + tag.len()..].trim().to_string();

    // Split at the last ':' to separate the file path from the line number
    let colon = prefix.rfind(':')?;
    let file = prefix[..colon].to_string();
    let line_num: usize = prefix[colon + 1..].parse().ok()?;

    // Sanity check: javac errors reference .java files at valid line numbers
    if !file.ends_with(".java") || line_num == 0 {
        return None;
    }

    Some(Diagnostic {
        file,
        line: line_num,
        col: 0,
        span_len: 1,
        severity,
        message,
        details: Vec::new(),
    })
}

fn is_header(line: &str) -> bool {
    parse_header(line).is_some()
}

// ── Renderer ──────────────────────────────────────────────────────────────────

fn print_diagnostic(diag: &Diagnostic) {
    let source = match std::fs::read_to_string(&diag.file) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("{}:{}: {}", diag.file, diag.line, diag.message);
            return;
        }
    };

    let start = byte_offset(&source, diag.line, diag.col);
    let end = (start + diag.span_len).min(source.len());

    // Show path relative to cwd so diagnostics read as "Foo.java:4" not "/abs/path/Foo.java:4"
    let rel = std::env::current_dir()
        .ok()
        .and_then(|cwd| std::path::Path::new(&diag.file).strip_prefix(&cwd).ok()
            .map(|p| p.to_string_lossy().into_owned()))
        .unwrap_or_else(|| diag.file.clone());
    let file_id = rel.as_str();

    let (kind, color) = match diag.severity {
        Severity::Error => (ReportKind::Error, Color::Red),
        Severity::Warning => (ReportKind::Warning, Color::Yellow),
        Severity::Note => (ReportKind::Advice, Color::Cyan),
    };

    let mut builder = Report::build(kind, (file_id, start..end))
        .with_message(&diag.message)
        .with_label(
            Label::new((file_id, start..end))
                .with_message(&diag.message)
                .with_color(color),
        );

    for detail in &diag.details {
        builder = builder.with_note(detail);
    }

    builder
        .finish()
        .eprint((file_id, Source::from(source.as_str())))
        .ok();
}

/// Converts a 1-indexed line number and 0-indexed byte column into a byte
/// offset from the start of the source string.
fn byte_offset(source: &str, line: usize, col: usize) -> usize {
    source.lines().take(line - 1).map(|l| l.len() + 1).sum::<usize>() + col
}
