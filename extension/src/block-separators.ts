import * as vscode from "vscode";

/**
 * Block Separator Lines
 *
 * Draws a subtle horizontal rule between block boundaries.
 * This creates visual "rooms" — each block is a distinct zone.
 * The separator is a thin line just above each block opening tag,
 * except the first block in the file.
 *
 * Apple HIG: "Distinct visual layers convey hierarchy."
 */

const separatorDecoration = vscode.window.createTextEditorDecorationType({
  isWholeLine: true,
  borderWidth: "1px 0 0 0",
  borderStyle: "solid",
  borderColor: "rgba(128, 128, 128, 0.15)",
  overviewRulerColor: "rgba(128, 128, 128, 0.2)",
  overviewRulerLane: vscode.OverviewRulerLane.Right,
});

function findBlockStarts(document: vscode.TextDocument): vscode.Range[] {
  const blockPattern = /^<(route|script|template|style|resource|layout)[\s>]/;
  const ranges: vscode.Range[] = [];
  let isFirst = true;

  for (let i = 0; i < document.lineCount; i++) {
    const line = document.lineAt(i);
    if (blockPattern.test(line.text.trim())) {
      if (isFirst) {
        isFirst = false;
        continue; // Skip separator above first block
      }
      ranges.push(new vscode.Range(i, 0, i, 0));
    }
  }

  return ranges;
}

function updateSeparators(editor: vscode.TextEditor) {
  if (editor.document.languageId !== "adapto") {
    editor.setDecorations(separatorDecoration, []);
    return;
  }

  const ranges = findBlockStarts(editor.document);
  editor.setDecorations(separatorDecoration, ranges);
}

export function registerBlockSeparators(context: vscode.ExtensionContext) {
  let timeout: NodeJS.Timeout | undefined;

  function triggerUpdate(editor: vscode.TextEditor) {
    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(() => updateSeparators(editor), 100);
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
