import * as vscode from "vscode";

/**
 * Security Lens — CodeLens annotations for permission/audit decorators
 *
 * Apple HIG principle: important information should be visible without
 * interaction. Security boundaries are the most important thing in
 * enterprise SaaS code. This makes them impossible to miss.
 *
 * Shows inline annotations above:
 * - #[permission("...")] — shield icon + permission name
 * - #[audit("...")] — eye icon + audit event
 * - {#can "..."} — lock icon + permission name
 * - auth: required/verified — in route block
 */

const securityDecoration = vscode.window.createTextEditorDecorationType({
  after: {
    margin: "0 0 0 1em",
    color: "rgba(224, 108, 117, 0.7)",
    fontStyle: "italic",
    fontWeight: "normal",
  },
});

const auditDecoration = vscode.window.createTextEditorDecorationType({
  after: {
    margin: "0 0 0 1em",
    color: "rgba(209, 154, 102, 0.7)",
    fontStyle: "italic",
    fontWeight: "normal",
  },
});

interface LensAnnotation {
  range: vscode.Range;
  text: string;
  type: "security" | "audit";
}

function findAnnotations(document: vscode.TextDocument): LensAnnotation[] {
  const text = document.getText();
  const annotations: LensAnnotation[] = [];

  // #[permission("...")]
  const permRegex = /#\[permission\("([^"]+)"\)\]/g;
  let match: RegExpExecArray | null;
  while ((match = permRegex.exec(text)) !== null) {
    const pos = document.positionAt(match.index);
    annotations.push({
      range: new vscode.Range(pos.line, 0, pos.line, 0),
      text: `  requires: ${match[1]}`,
      type: "security",
    });
  }

  // #[audit("...")]
  const auditRegex = /#\[audit\("([^"]+)"\)\]/g;
  while ((match = auditRegex.exec(text)) !== null) {
    const pos = document.positionAt(match.index);
    annotations.push({
      range: new vscode.Range(pos.line, 0, pos.line, 0),
      text: `  logs: ${match[1]}`,
      type: "audit",
    });
  }

  return annotations;
}

function updateAnnotations(editor: vscode.TextEditor) {
  if (editor.document.languageId !== "adapto") {
    editor.setDecorations(securityDecoration, []);
    editor.setDecorations(auditDecoration, []);
    return;
  }

  const annotations = findAnnotations(editor.document);

  const securityItems: vscode.DecorationOptions[] = annotations
    .filter((a) => a.type === "security")
    .map((a) => ({
      range: new vscode.Range(
        a.range.start.line,
        Number.MAX_SAFE_INTEGER,
        a.range.start.line,
        Number.MAX_SAFE_INTEGER,
      ),
      renderOptions: {
        after: {
          contentText: a.text,
        },
      },
    }));

  const auditItems: vscode.DecorationOptions[] = annotations
    .filter((a) => a.type === "audit")
    .map((a) => ({
      range: new vscode.Range(
        a.range.start.line,
        Number.MAX_SAFE_INTEGER,
        a.range.start.line,
        Number.MAX_SAFE_INTEGER,
      ),
      renderOptions: {
        after: {
          contentText: a.text,
        },
      },
    }));

  editor.setDecorations(securityDecoration, securityItems);
  editor.setDecorations(auditDecoration, auditItems);
}

export function registerSecurityLens(context: vscode.ExtensionContext) {
  let timeout: NodeJS.Timeout | undefined;

  function triggerUpdate(editor: vscode.TextEditor) {
    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(() => updateAnnotations(editor), 100);
  }

  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      if (editor) triggerUpdate(editor);
    }),
  );

  context.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((e) => {
      const editor = vscode.window.activeTextEditor;
      if (editor && e.document === editor.document) {
        triggerUpdate(editor);
      }
    }),
  );

  if (vscode.window.activeTextEditor) {
    triggerUpdate(vscode.window.activeTextEditor);
  }
}
