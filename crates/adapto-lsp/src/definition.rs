use tower_lsp::lsp_types::*;

use crate::document::DocumentState;

pub fn goto_definition(doc: &DocumentState, position: Position, uri: &Url) -> Option<Location> {
    let line_idx = position.line as usize;
    let col = position.character as usize;
    let lines: Vec<&str> = doc.text.lines().collect();
    let current_line = lines.get(line_idx)?;

    let word = extract_word(current_line, col)?;

    let script = doc.ast.as_ref()?.script.as_ref()?;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with(&format!("state {}", word))
            || trimmed.starts_with(&format!("prop {}", word))
            || trimmed.starts_with(&format!("memo {}", word))
        {
            return Some(Location {
                uri: uri.clone(),
                range: Range {
                    start: Position {
                        line: i as u32,
                        character: 0,
                    },
                    end: Position {
                        line: i as u32,
                        character: line.len() as u32,
                    },
                },
            });
        }

        for a in &script.actions {
            if a.name == word && trimmed.contains(&format!("fn {}", word)) {
                return Some(Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position {
                            line: i as u32,
                            character: 0,
                        },
                        end: Position {
                            line: i as u32,
                            character: line.len() as u32,
                        },
                    },
                });
            }
        }

        for l in &script.loaders {
            if l.name == word && trimmed.contains(&format!("fn {}", word)) {
                return Some(Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position {
                            line: i as u32,
                            character: 0,
                        },
                        end: Position {
                            line: i as u32,
                            character: line.len() as u32,
                        },
                    },
                });
            }
        }

        for sf in &script.server_fns {
            if sf.name == word && trimmed.contains(&format!("fn {}", word)) {
                return Some(Location {
                    uri: uri.clone(),
                    range: Range {
                        start: Position {
                            line: i as u32,
                            character: 0,
                        },
                        end: Position {
                            line: i as u32,
                            character: line.len() as u32,
                        },
                    },
                });
            }
        }
    }

    None
}

fn extract_word(line: &str, col: usize) -> Option<String> {
    let bytes = line.as_bytes();
    if col >= bytes.len() {
        return None;
    }

    let mut start = col;
    while start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
        start -= 1;
    }

    let mut end = col;
    while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_') {
        end += 1;
    }

    if start == end {
        return None;
    }

    Some(line[start..end].to_string())
}
