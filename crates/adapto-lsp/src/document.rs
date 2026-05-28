use adapto_compiler::compiler::{CompileOutput, Compiler};
use adapto_compiler::error::CompileError;
use adapto_parser::{parse, AdaptoFile, ParseError};
use dashmap::DashMap;
use tower_lsp::lsp_types::Url;

#[derive(Debug)]
pub struct DocumentState {
    pub text: String,
    pub version: i32,
    pub ast: Option<AdaptoFile>,
    pub compiled: Option<CompileOutput>,
    pub parse_errors: Vec<ParseError>,
    pub compile_errors: Vec<CompileError>,
}

impl DocumentState {
    pub fn new(text: String, version: i32) -> Self {
        let mut state = Self {
            text,
            version,
            ast: None,
            compiled: None,
            parse_errors: Vec::new(),
            compile_errors: Vec::new(),
        };
        state.reparse();
        state
    }

    pub fn update(&mut self, text: String, version: i32) {
        self.text = text;
        self.version = version;
        self.reparse();
    }

    fn reparse(&mut self) {
        self.parse_errors.clear();
        self.compile_errors.clear();

        match parse(&self.text) {
            Ok(ast) => {
                let mut compiler = Compiler::new();
                match compiler.compile_file(&ast, "<editor>") {
                    Ok(output) => {
                        self.compiled = Some(output);
                    }
                    Err(e) => {
                        self.compile_errors.push(e);
                        self.compiled = None;
                    }
                }
                self.ast = Some(ast);
            }
            Err(e) => {
                self.parse_errors.push(e);
                self.ast = None;
                self.compiled = None;
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct DocumentStore {
    documents: DashMap<Url, DocumentState>,
}

impl DocumentStore {
    pub fn open(&self, uri: Url, text: String, version: i32) {
        self.documents.insert(uri, DocumentState::new(text, version));
    }

    pub fn change(&self, uri: &Url, text: String, version: i32) {
        if let Some(mut doc) = self.documents.get_mut(uri) {
            doc.update(text, version);
        }
    }

    pub fn close(&self, uri: &Url) {
        self.documents.remove(uri);
    }

    pub fn get(&self, uri: &Url) -> Option<dashmap::mapref::one::Ref<'_, Url, DocumentState>> {
        self.documents.get(uri)
    }
}
