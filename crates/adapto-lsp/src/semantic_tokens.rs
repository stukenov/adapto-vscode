use tower_lsp::lsp_types::*;

use crate::document::DocumentState;

pub const TOKEN_TYPES: &[SemanticTokenType] = &[
    SemanticTokenType::VARIABLE,  // 0 - state
    SemanticTokenType::PROPERTY,  // 1 - prop
    SemanticTokenType::FUNCTION,  // 2 - action
    SemanticTokenType::KEYWORD,   // 3 - control flow
    SemanticTokenType::STRING,    // 4 - string
    SemanticTokenType::DECORATOR, // 5 - decorator
    SemanticTokenType::TYPE,      // 6 - type name
    SemanticTokenType::NAMESPACE, // 7 - block name
];

pub const TOKEN_MODIFIERS: &[SemanticTokenModifier] = &[
    SemanticTokenModifier::DECLARATION,
    SemanticTokenModifier::READONLY,
    SemanticTokenModifier::ASYNC,
];

pub fn semantic_tokens(doc: &DocumentState) -> Vec<SemanticToken> {
    let mut tokens = Vec::new();
    let mut prev_line = 0u32;
    let mut prev_start = 0u32;

    if doc.ast.is_none() {
        return tokens;
    }

    let lines: Vec<&str> = doc.text.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let i = i as u32;
        let trimmed = line.trim();

        if trimmed.starts_with("state ") {
            if let Some(name_end) = trimmed[6..].find(':') {
                let offset = line.len() - line.trim_start().len();
                let name_start = (offset + 6) as u32;
                push_token(
                    &mut tokens,
                    &mut prev_line,
                    &mut prev_start,
                    i,
                    name_start,
                    name_end as u32,
                    0,
                    0b001,
                );
            }
        }

        if trimmed.starts_with("prop ") {
            if let Some(name_end) = trimmed[5..].find(':') {
                let offset = line.len() - line.trim_start().len();
                let name_start = (offset + 5) as u32;
                push_token(
                    &mut tokens,
                    &mut prev_line,
                    &mut prev_start,
                    i,
                    name_start,
                    name_end as u32,
                    1,
                    0b010,
                );
            }
        }

        if trimmed.starts_with("memo ") {
            if let Some(name_end) = trimmed[5..].find(':') {
                let offset = line.len() - line.trim_start().len();
                let name_start = (offset + 5) as u32;
                push_token(
                    &mut tokens,
                    &mut prev_line,
                    &mut prev_start,
                    i,
                    name_start,
                    name_end as u32,
                    0,
                    0b010,
                );
            }
        }

        if trimmed.contains("action ") && trimmed.contains("fn ") {
            if let Some(fn_pos) = trimmed.find("fn ") {
                let after_fn = &trimmed[fn_pos + 3..];
                if let Some(paren) = after_fn.find('(') {
                    let offset = line.len() - line.trim_start().len();
                    let abs_start = (offset + fn_pos + 3) as u32;
                    push_token(
                        &mut tokens,
                        &mut prev_line,
                        &mut prev_start,
                        i,
                        abs_start,
                        paren as u32,
                        2,
                        0,
                    );
                }
            }
        }

        if trimmed.starts_with("#[permission") || trimmed.starts_with("#[audit") {
            let start = (line.len() - line.trim_start().len()) as u32;
            push_token(
                &mut tokens,
                &mut prev_line,
                &mut prev_start,
                i,
                start,
                trimmed.len() as u32,
                5,
                0,
            );
        }
    }

    tokens
}

fn push_token(
    tokens: &mut Vec<SemanticToken>,
    prev_line: &mut u32,
    prev_start: &mut u32,
    line: u32,
    start: u32,
    length: u32,
    token_type: u32,
    token_modifiers: u32,
) {
    let delta_line = line - *prev_line;
    let delta_start = if delta_line == 0 {
        start - *prev_start
    } else {
        start
    };

    tokens.push(SemanticToken {
        delta_line,
        delta_start,
        length,
        token_type,
        token_modifiers_bitset: token_modifiers,
    });

    *prev_line = line;
    *prev_start = start;
}
