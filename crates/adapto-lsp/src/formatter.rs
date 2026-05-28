use tower_lsp::lsp_types::*;

pub fn format_document(text: &str) -> Vec<TextEdit> {
    let mut formatted = String::new();
    let mut indent = 0usize;
    let mut in_style = false;
    let mut in_script = false;

    for line in text.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            formatted.push('\n');
            continue;
        }

        if trimmed.starts_with("</style>") {
            in_style = false;
        }
        if trimmed.starts_with("</script>") {
            in_script = false;
        }

        if in_style || in_script {
            formatted.push_str(line);
            formatted.push('\n');
            continue;
        }

        if trimmed.starts_with("</") || trimmed.starts_with("{/") {
            indent = indent.saturating_sub(1);
        }

        if trimmed == "{:else}"
            || trimmed.starts_with("{:else ")
            || trimmed.starts_with("{:when ")
        {
            let temp_indent = indent.saturating_sub(1);
            formatted.push_str(&"  ".repeat(temp_indent));
            formatted.push_str(trimmed);
            formatted.push('\n');
        } else {
            formatted.push_str(&"  ".repeat(indent));
            formatted.push_str(trimmed);
            formatted.push('\n');
        }

        if (trimmed.starts_with('<')
            && !trimmed.starts_with("</")
            && !trimmed.ends_with("/>")
            && !trimmed.ends_with("-->"))
            || trimmed.starts_with("{#")
        {
            indent += 1;
        }

        if trimmed.starts_with("<style") {
            in_style = true;
        }
        if trimmed.starts_with("<script") {
            in_script = true;
        }
    }

    let line_count = text.lines().count() as u32;
    let last_line_len = text.lines().last().map(|l| l.len()).unwrap_or(0) as u32;

    vec![TextEdit {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: line_count,
                character: last_line_len,
            },
        },
        new_text: formatted,
    }]
}
