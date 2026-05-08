use owo_colors::OwoColorize;

/// Colorizes JVM stderr output (uncaught exceptions, stack traces).
pub fn colorize_runtime_stderr(output: &str) -> String {
    output.lines().map(colorize_stacktrace_line).collect::<Vec<_>>().join("\n")
}

fn colorize_stacktrace_line(line: &str) -> String {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];

    // Exception header: "Exception in thread ...", "Caused by:", bare exception class
    if trimmed.starts_with("Exception in thread")
        || trimmed.starts_with("Caused by:")
        || ((trimmed.contains("Exception:") || trimmed.contains("Error:"))
            && !trimmed.starts_with("at "))
    {
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
    if let Some(paren) = frame.rfind('(') {
        let class_method = &frame[..paren];
        let file_loc = &frame[paren..];
        format!("{}{}{}{}",indent.dimmed(), "at ".dimmed(), class_method.cyan(), file_loc.yellow())
    } else {
        format!("{}{}{}", indent.dimmed(), "at ".dimmed(), frame.cyan())
    }
}
