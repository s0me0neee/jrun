use owo_colors::OwoColorize;

/// Colorizes javac compiler error/warning output.
pub fn colorize_compile_error(output: &str) -> String {
    output
        .lines()
        .map(colorize_javac_line)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Colorizes JVM stderr output (uncaught exceptions, stack traces).
pub fn colorize_runtime_stderr(output: &str) -> String {
    output
        .lines()
        .map(colorize_stacktrace_line)
        .collect::<Vec<_>>()
        .join("\n")
}

// ── Javac output ──────────────────────────────────────────────────────────────

fn colorize_javac_line(line: &str) -> String {
    let trimmed = line.trim_start();

    // Location pointer ("    ^")
    if trimmed.starts_with('^') {
        return line.yellow().to_string();
    }

    // Detail lines printed under diagnostics
    if trimmed.starts_with("symbol:") || trimmed.starts_with("location:") {
        return line.dimmed().to_string();
    }

    // Diagnostic lines: "File.java:N: error/warning/note: message"
    if let Some(colored) = try_colorize_diagnostic(line) {
        return colored;
    }

    // Summary: "1 error", "2 errors"
    if looks_like_error_summary(trimmed) {
        return line.red().bold().to_string();
    }
    // Summary: "1 warning", "2 warnings"
    if looks_like_warning_summary(trimmed) {
        return line.yellow().bold().to_string();
    }

    line.to_string()
}

fn try_colorize_diagnostic(line: &str) -> Option<String> {
    // javac format: "<file>:<line>: <severity>: <message>"
    if let Some(idx) = line.find(": error:") {
        let (loc, rest) = line.split_at(idx);
        let msg = &rest[": error:".len()..];
        return Some(format!("{}{}{}", loc.yellow(), ": error:".red().bold(), msg.red()));
    }
    if let Some(idx) = line.find(": warning:") {
        let (loc, rest) = line.split_at(idx);
        let msg = &rest[": warning:".len()..];
        return Some(format!(
            "{}{}{}",
            loc.yellow(),
            ": warning:".yellow().bold(),
            msg.yellow()
        ));
    }
    if let Some(idx) = line.find(": note:") {
        let (loc, rest) = line.split_at(idx);
        let msg = &rest[": note:".len()..];
        return Some(format!("{}{}{}", loc.cyan(), ": note:".cyan().bold(), msg.cyan()));
    }
    None
}

fn looks_like_error_summary(trimmed: &str) -> bool {
    (trimmed.ends_with(" error") || trimmed.ends_with(" errors"))
        && trimmed.split_whitespace().next().map_or(false, |w| w.parse::<u32>().is_ok())
}

fn looks_like_warning_summary(trimmed: &str) -> bool {
    (trimmed.ends_with(" warning") || trimmed.ends_with(" warnings"))
        && trimmed.split_whitespace().next().map_or(false, |w| w.parse::<u32>().is_ok())
}

// ── JVM stack traces ──────────────────────────────────────────────────────────

fn colorize_stacktrace_line(line: &str) -> String {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];

    // Exception header: "Exception in thread ...", "Caused by:", or bare exception class
    if trimmed.starts_with("Exception in thread")
        || trimmed.starts_with("Caused by:")
        || (trimmed.contains("Exception:") || trimmed.contains("Error:"))
            && !trimmed.starts_with("at ")
    {
        // Bold the exception type, keep the message dimmer
        if let Some(colon) = trimmed.find(": ") {
            let (exc_type, msg) = trimmed.split_at(colon);
            return format!("{}{}{}", indent, exc_type.red().bold(), msg.red());
        }
        return line.red().bold().to_string();
    }

    // Stack frame: "    at pkg.Class.method(File.java:42)"
    if let Some(frame) = trimmed.strip_prefix("at ") {
        return colorize_stack_frame(indent, frame);
    }

    // "... N more"
    if trimmed.starts_with("...") && trimmed.ends_with("more") {
        return line.dimmed().to_string();
    }

    line.to_string()
}

fn colorize_stack_frame(indent: &str, frame: &str) -> String {
    // Split "pkg.Class.method(File.java:42)" at the last '('
    if let Some(paren) = frame.rfind('(') {
        let class_method = &frame[..paren];
        let file_loc = &frame[paren..]; // includes '('
        format!(
            "{}{}{}{}",
            indent.dimmed(),
            "at ".dimmed(),
            class_method.cyan(),
            file_loc.yellow(),
        )
    } else {
        format!("{}{}{}",indent.dimmed(), "at ".dimmed(), frame.cyan())
    }
}
