use tower_lsp::lsp_types::*;

use crate::document::DocumentState;

pub fn completions(doc: &DocumentState, position: Position) -> Vec<CompletionItem> {
    let line_idx = position.line as usize;
    let lines: Vec<&str> = doc.text.lines().collect();
    let current_line = lines.get(line_idx).copied().unwrap_or("");

    let mut items = Vec::new();
    let context = detect_context(current_line, &lines, line_idx);

    match context {
        Context::TopLevel => items.extend(block_completions()),
        Context::Route => items.extend(route_attribute_completions()),
        Context::Template => {
            items.extend(template_completions());
            if let Some(ref ast) = doc.ast {
                items.extend(state_field_completions(ast));
            }
        }
        Context::Resource => items.extend(resource_completions()),
        Context::Script => items.extend(script_keyword_completions()),
        Context::EventModifier => items.extend(event_modifier_completions()),
        Context::Unknown => {}
    }

    items
}

#[derive(Debug)]
enum Context {
    TopLevel,
    Route,
    Template,
    Resource,
    Script,
    EventModifier,
    Unknown,
}

fn detect_context(current_line: &str, lines: &[&str], line_idx: usize) -> Context {
    let trimmed = current_line.trim();
    if trimmed.starts_with('<') && !trimmed.contains('>') {
        return Context::TopLevel;
    }

    if trimmed.contains("on:") {
        return Context::EventModifier;
    }

    for i in (0..=line_idx).rev() {
        let l = lines.get(i).copied().unwrap_or("").trim().to_string();
        if l.starts_with("</") {
            continue;
        }
        if l.starts_with("<route") {
            return Context::Route;
        }
        if l.starts_with("<template") {
            return Context::Template;
        }
        if l.starts_with("<resource") {
            return Context::Resource;
        }
        if l.starts_with("<script") {
            return Context::Script;
        }
    }

    Context::Unknown
}

fn block_completions() -> Vec<CompletionItem> {
    vec![
        make_snippet("route", "<route>\n\t$0\n</route>", "Route block"),
        make_snippet(
            "script",
            "<script lang=\"rust\">\n\t$0\n</script>",
            "Script block",
        ),
        make_snippet("template", "<template>\n\t$0\n</template>", "Template block"),
        make_snippet("style", "<style scoped>\n\t$0\n</style>", "Style block"),
        make_snippet(
            "resource",
            "<resource table=\"$1\">\n\t$0\n</resource>",
            "Resource block",
        ),
        make_snippet(
            "layout",
            "<layout name=\"$1\">\n\t$0\n</layout>",
            "Layout block",
        ),
    ]
}

fn route_attribute_completions() -> Vec<CompletionItem> {
    vec![
        make_keyword("path", "Route path pattern"),
        make_keyword("method", "HTTP method (get, post, put, delete)"),
        make_keyword("layout", "Layout name"),
        make_keyword("auth", "Auth level (public, required, verified)"),
        make_keyword("permission", "Required permission"),
        make_keyword("tenant", "Tenant mode (none, required, optional)"),
        make_keyword("cache", "Cache policy"),
    ]
}

fn template_completions() -> Vec<CompletionItem> {
    vec![
        make_snippet("if", "{#if $1}\n\t$0\n{/if}", "Conditional block"),
        make_snippet(
            "if-else",
            "{#if $1}\n\t$2\n{:else}\n\t$0\n{/if}",
            "Conditional with else",
        ),
        make_snippet(
            "each",
            "{#each $1 as $2}\n\t$0\n{/each}",
            "Loop over collection",
        ),
        make_snippet(
            "match",
            "{#match $1}\n\t{:when $2}\n\t\t$0\n{/match}",
            "Pattern match",
        ),
        make_snippet("can", "{#can \"$1\"}\n\t$0\n{/can}", "Permission guard"),
        make_snippet("html", "{@html $0}", "Raw HTML output"),
    ]
}

fn resource_completions() -> Vec<CompletionItem> {
    vec![
        make_snippet(
            "field",
            "field $1: $2 ${3|@required,@optional,@unique,@searchable|}",
            "Resource field",
        ),
        make_keyword("permission", "Resource permission"),
        make_keyword("tenant", "Tenant scope"),
        make_keyword("primary_key", "Primary key field"),
    ]
}

fn script_keyword_completions() -> Vec<CompletionItem> {
    vec![
        make_snippet("state", "state $1: $2 = $0", "State declaration"),
        make_snippet("prop", "prop $1: $2", "Prop declaration"),
        make_snippet("memo", "memo $1: $2 = $0", "Memo (computed) declaration"),
        make_snippet(
            "load",
            "load async fn $1(ctx: Ctx) {\n\t$0\n}",
            "Load function",
        ),
        make_snippet(
            "action",
            "action async fn $1($2) {\n\t$0\n}",
            "Action handler",
        ),
        make_snippet(
            "server",
            "server async fn $1($2) {\n\t$0\n}",
            "Server function",
        ),
    ]
}

fn event_modifier_completions() -> Vec<CompletionItem> {
    vec![
        make_keyword("prevent", "Call preventDefault()"),
        make_keyword("stop", "Call stopPropagation()"),
        make_snippet("debounce", "debounce.${1:300}", "Debounce with ms delay"),
        make_snippet("throttle", "throttle.${1:300}", "Throttle with ms delay"),
    ]
}

fn state_field_completions(ast: &adapto_parser::AdaptoFile) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    if let Some(ref script) = ast.script {
        for s in &script.states {
            items.push(CompletionItem {
                label: s.name.clone(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some(format!("state: {}", s.ty)),
                ..Default::default()
            });
        }
        for p in &script.props {
            items.push(CompletionItem {
                label: p.name.clone(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some(format!("prop: {}", p.ty)),
                ..Default::default()
            });
        }
        for m in &script.memos {
            items.push(CompletionItem {
                label: m.name.clone(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some(format!("memo: {}", m.ty)),
                ..Default::default()
            });
        }
        for a in &script.actions {
            items.push(CompletionItem {
                label: a.name.clone(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("action".to_string()),
                ..Default::default()
            });
        }
    }
    items
}

fn make_snippet(label: &str, snippet: &str, detail: &str) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some(detail.to_string()),
        insert_text: Some(snippet.to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    }
}

fn make_keyword(label: &str, detail: &str) -> CompletionItem {
    CompletionItem {
        label: label.to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        detail: Some(detail.to_string()),
        ..Default::default()
    }
}
