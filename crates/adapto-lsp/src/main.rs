mod completion;
mod definition;
mod document;
mod formatter;
mod hover;
mod semantic_tokens;
mod symbols;

use document::DocumentStore;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct AdaptoLsp {
    client: Client,
    documents: DocumentStore,
}

#[tower_lsp::async_trait]
impl LanguageServer for AdaptoLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![
                        "<".to_string(),
                        "{".to_string(),
                        ":".to_string(),
                        ".".to_string(),
                        "@".to_string(),
                    ]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: semantic_tokens::TOKEN_TYPES.to_vec(),
                                token_modifiers: semantic_tokens::TOKEN_MODIFIERS.to_vec(),
                            },
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            range: None,
                            ..Default::default()
                        },
                    ),
                ),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Adapto LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let version = params.text_document.version;
        self.documents.open(uri.clone(), text, version);
        self.publish_diagnostics(&uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;
        if let Some(change) = params.content_changes.into_iter().last() {
            self.documents.change(&uri, change.text, version);
            self.publish_diagnostics(&uri).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.close(&params.text_document.uri);
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        let items = if let Some(doc) = self.documents.get(&uri) {
            completion::completions(&doc, pos)
        } else {
            Vec::new()
        };
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let result = if let Some(doc) = self.documents.get(&uri) {
            hover::hover_info(&doc, pos)
        } else {
            None
        };
        Ok(result)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let result = if let Some(doc) = self.documents.get(&uri) {
            definition::goto_definition(&doc, pos, &uri).map(GotoDefinitionResponse::Scalar)
        } else {
            None
        };
        Ok(result)
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let syms = if let Some(doc) = self.documents.get(&uri) {
            symbols::document_symbols(&doc)
        } else {
            Vec::new()
        };
        Ok(Some(DocumentSymbolResponse::Nested(syms)))
    }

    async fn formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;
        let edits = if let Some(doc) = self.documents.get(&uri) {
            formatter::format_document(&doc.text)
        } else {
            Vec::new()
        };
        Ok(Some(edits))
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        let tokens = if let Some(doc) = self.documents.get(&uri) {
            semantic_tokens::semantic_tokens(&doc)
        } else {
            Vec::new()
        };
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
        })))
    }
}

impl AdaptoLsp {
    async fn publish_diagnostics(&self, uri: &Url) {
        let diagnostics = if let Some(doc) = self.documents.get(uri) {
            let mut diags = Vec::new();
            for err in &doc.parse_errors {
                if let adapto_parser::ParseError::Syntax { line, col, message } = err {
                    diags.push(Diagnostic {
                        range: Range {
                            start: Position {
                                line: line.saturating_sub(1) as u32,
                                character: col.saturating_sub(1) as u32,
                            },
                            end: Position {
                                line: line.saturating_sub(1) as u32,
                                character: *col as u32,
                            },
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        source: Some("adapto".to_string()),
                        message: message.clone(),
                        ..Default::default()
                    });
                }
            }
            for err in &doc.compile_errors {
                diags.push(Diagnostic {
                    range: Range::default(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("adapto".to_string()),
                    message: err.to_string(),
                    ..Default::default()
                });
            }
            diags
        } else {
            Vec::new()
        };

        self.client
            .publish_diagnostics(uri.clone(), diagnostics, None)
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| AdaptoLsp {
        client,
        documents: DocumentStore::default(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
