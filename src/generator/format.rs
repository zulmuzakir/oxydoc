use typst::diag::SourceDiagnostic;

pub fn typst_string(value: &str) -> String {
    let mut rendered = String::with_capacity(value.len() + 2);
    rendered.push('"');
    
    for ch in value.chars() {
        match ch {
            '\\' => rendered.push_str("\\\\"),
            '"' => rendered.push_str("\\\""),
            '\n' => rendered.push_str("\\n"),
            '\r' => rendered.push_str("\\r"),
            '\t' => rendered.push_str("\\t"),
            _ => rendered.push(ch),
        }
    }

    rendered.push('"');
    rendered
}

pub fn diagnostic_summary(diagnostic: &SourceDiagnostic) -> String {
    if diagnostic.hints.is_empty() {
        diagnostic.message.to_string()
    } else {
        format!("{} ({})", diagnostic.message, diagnostic.hints.join("; "))
    }
}

pub fn join_diagnostics(diagnostics: &[SourceDiagnostic]) -> String {
    if diagnostics.is_empty() {
        return "unknown Typst diagnostic".to_owned();
    }

    diagnostics
        .iter()
        .map(diagnostic_summary)
        .collect::<Vec<_>>()
        .join(" | ")
}

pub fn truncate(text: &str, max_len: usize) -> String {
    let mut truncated = String::new();
    let mut chars = text.chars();

    for _ in 0..max_len {
        let Some(ch) = chars.next() else {
            return text.to_owned();
        };

        truncated.push(ch);
    }

    if chars.next().is_some() {
        truncated.push_str("...")
    }

    truncated
}