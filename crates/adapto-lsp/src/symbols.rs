use tower_lsp::lsp_types::*;

use crate::document::DocumentState;

#[allow(deprecated)]
pub fn document_symbols(doc: &DocumentState) -> Vec<DocumentSymbol> {
    let mut symbols = Vec::new();
    let ast = match &doc.ast {
        Some(a) => a,
        None => return symbols,
    };

    if let Some(ref route) = ast.route {
        let mut children = Vec::new();
        if let Some(ref path) = route.path {
            children.push(make_property("path", path, SymbolKind::STRING));
        }
        if let Some(ref method) = route.method {
            children.push(make_property("method", method, SymbolKind::STRING));
        }
        symbols.push(DocumentSymbol {
            name: "route".to_string(),
            detail: route.path.clone(),
            kind: SymbolKind::MODULE,
            tags: None,
            deprecated: None,
            range: Range::default(),
            selection_range: Range::default(),
            children: Some(children),
        });
    }

    if let Some(ref script) = ast.script {
        let mut children = Vec::new();

        for s in &script.states {
            children.push(make_symbol(
                &s.name,
                Some(s.ty.clone()),
                SymbolKind::VARIABLE,
            ));
        }
        for p in &script.props {
            children.push(make_symbol(
                &p.name,
                Some(p.ty.clone()),
                SymbolKind::PROPERTY,
            ));
        }
        for m in &script.memos {
            children.push(make_symbol(
                &m.name,
                Some(format!("{} (memo)", m.ty)),
                SymbolKind::VARIABLE,
            ));
        }
        for l in &script.loaders {
            children.push(make_symbol(
                &l.name,
                Some("loader".to_string()),
                SymbolKind::FUNCTION,
            ));
        }
        for a in &script.actions {
            children.push(make_symbol(
                &a.name,
                Some("action".to_string()),
                SymbolKind::METHOD,
            ));
        }
        for sf in &script.server_fns {
            children.push(make_symbol(
                &sf.name,
                Some("server fn".to_string()),
                SymbolKind::FUNCTION,
            ));
        }
        for f in &script.forms {
            children.push(make_symbol(
                &f.name,
                Some("form".to_string()),
                SymbolKind::STRUCT,
            ));
        }

        symbols.push(DocumentSymbol {
            name: "script".to_string(),
            detail: None,
            kind: SymbolKind::NAMESPACE,
            tags: None,
            deprecated: None,
            range: Range::default(),
            selection_range: Range::default(),
            children: Some(children),
        });
    }

    if ast.template.is_some() {
        symbols.push(make_symbol("template", None, SymbolKind::NAMESPACE));
    }

    if let Some(ref style) = ast.style {
        let scope = if style.scoped { "scoped" } else { "global" };
        symbols.push(make_symbol(
            "style",
            Some(scope.to_string()),
            SymbolKind::NAMESPACE,
        ));
    }

    if let Some(ref resource) = ast.resource {
        let mut children = Vec::new();
        for f in &resource.fields {
            children.push(make_symbol(&f.name, Some(f.ty.clone()), SymbolKind::FIELD));
        }
        symbols.push(DocumentSymbol {
            name: format!("resource ({})", resource.table),
            detail: Some(resource.name.clone()),
            kind: SymbolKind::CLASS,
            tags: None,
            deprecated: None,
            range: Range::default(),
            selection_range: Range::default(),
            children: Some(children),
        });
    }

    symbols
}

#[allow(deprecated)]
fn make_symbol(name: &str, detail: Option<String>, kind: SymbolKind) -> DocumentSymbol {
    DocumentSymbol {
        name: name.to_string(),
        detail,
        kind,
        tags: None,
        deprecated: None,
        range: Range::default(),
        selection_range: Range::default(),
        children: None,
    }
}

#[allow(deprecated)]
fn make_property(name: &str, value: &str, kind: SymbolKind) -> DocumentSymbol {
    DocumentSymbol {
        name: name.to_string(),
        detail: Some(value.to_string()),
        kind,
        tags: None,
        deprecated: None,
        range: Range::default(),
        selection_range: Range::default(),
        children: None,
    }
}
