use tower_lsp::lsp_types::*;

use crate::document::DocumentState;

pub fn hover_info(doc: &DocumentState, position: Position) -> Option<Hover> {
    let line_idx = position.line as usize;
    let col = position.character as usize;
    let lines: Vec<&str> = doc.text.lines().collect();
    let current_line = lines.get(line_idx)?;

    let word = extract_word(current_line, col)?;

    if let Some(info) = keyword_hover(&word) {
        return Some(info);
    }

    if let Some(ref ast) = doc.ast {
        if let Some(info) = state_hover(ast, &word) {
            return Some(info);
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

fn keyword_hover(word: &str) -> Option<Hover> {
    let docs = match word {
        "route" => "**`<route>`** \u{2014} Route metadata block\n\nDefines HTTP route: path, method, auth level, layout, permissions, tenant scope, caching.",
        "script" => "**`<script lang=\"rust\">`** \u{2014} Rust logic block\n\nDeclare state, props, memos, load functions, actions, server functions, and forms.",
        "template" => "**`<template>`** \u{2014} HTML template block\n\nReactive HTML with control flow (`{#if}`, `{#each}`, `{#match}`, `{#can}`), expressions (`{expr}`), event bindings (`on:event`), and two-way bindings (`bind:field`).",
        "style" => "**`<style>`** \u{2014} CSS styles\n\nAdd `scoped` for component-local styles or `global` for app-wide.",
        "resource" => "**`<resource>`** \u{2014} Data model definition\n\nDefines fields with types and constraints, permissions, tenant scoping. Generates CRUD operations.",
        "layout" => "**`<layout>`** \u{2014} Layout wrapper\n\nNamed layout with auth and tenant requirements. Referenced by routes.",
        "state" => "**`state`** \u{2014} Reactive state field\n\n```\nstate name: Type = default\n```\n\nDeclares reactive state. Changes trigger template re-render via dynamic segments.",
        "prop" => "**`prop`** \u{2014} Component property\n\n```\nprop name: Type\n```\n\nRead-only input from parent component.",
        "memo" => "**`memo`** \u{2014} Computed value\n\n```\nmemo name: Type = expression\n```\n\nDerived from state, re-computed when dependencies change.",
        "load" => "**`load`** \u{2014} Data loader\n\n```\nload async fn name(ctx: Ctx) { ... }\n```\n\nRuns on page load. Populates initial state from DB or API.",
        "action" => "**`action`** \u{2014} Event handler\n\n```\n#[permission(\"...\")]\naction async fn name(params) { ... }\n```\n\nHandles user interactions. Can have permission and audit decorators.",
        "island" => "**`island`** \u{2014} Component isolation\n\nMarks component for independent hydration. Only this component's JS ships to client.",
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: docs.to_string(),
        }),
        range: None,
    })
}

fn state_hover(ast: &adapto_parser::AdaptoFile, word: &str) -> Option<Hover> {
    let script = ast.script.as_ref()?;

    for s in &script.states {
        if s.name == word {
            let mut info = format!("**state** `{}: {}`", s.name, s.ty);
            if let Some(ref d) = s.default {
                info.push_str(&format!(" = `{}`", d));
            }
            if s.secret {
                info.push_str("\n\nSecret \u{2014} cannot be rendered in template");
            }
            return Some(make_hover(&info));
        }
    }

    for p in &script.props {
        if p.name == word {
            let info = format!("**prop** `{}: {}`", p.name, p.ty);
            return Some(make_hover(&info));
        }
    }

    for m in &script.memos {
        if m.name == word {
            let info = format!("**memo** `{}: {}` = `{}`", m.name, m.ty, m.expr);
            return Some(make_hover(&info));
        }
    }

    for a in &script.actions {
        if a.name == word {
            let params: Vec<String> = a
                .params
                .iter()
                .map(|p| format!("{}: {}", p.name, p.ty))
                .collect();
            let mut info = format!("**action** `{}({})`", a.name, params.join(", "));
            if let Some(ref perm) = a.permission {
                info.push_str(&format!("\n\nPermission: `{}`", perm));
            }
            if a.is_async {
                info.push_str("\n\nasync");
            }
            return Some(make_hover(&info));
        }
    }

    None
}

fn make_hover(content: &str) -> Hover {
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content.to_string(),
        }),
        range: None,
    }
}
