import * as vscode from "vscode";

/**
 * Control Flow Pair Highlighting
 *
 * When the cursor is on a control flow opener ({#if, {#each, {#match, {#can}),
 * its matching closer ({/if}, {/each}, etc.) is subtly highlighted, and vice versa.
 * Also highlights intermediate branches ({:else}, {:when}).
 *
 * This creates visual depth — the user always knows the extent of a control
 * flow region without manually scanning. Like matching braces, but for
 * Adapto's template control flow.
 */

const OPENERS = /\{#(if|each|match|can)\b/g;
const CLOSERS = /\{\/(if|each|match|can)\}/g;
const BRANCHES = /\{:(else(?:\s+if)?|when)\b/g;

interface ControlFlowBlock {
  type: string;
  openRange: vscode.Range;
  closeRange: vscode.Range;
  branchRanges: vscode.Range[];
}

const pairDecoration = vscode.window.createTextEditorDecorationType({
  backgroundColor: "rgba(198, 120, 221, 0.08)",
  borderColor: "rgba(198, 120, 221, 0.25)",
  borderWidth: "1px",
  borderStyle: "solid",
  borderRadius: "2px",
});

const activePairDecoration = vscode.window.createTextEditorDecorationType({
  backgroundColor: "rgba(198, 120, 221, 0.15)",
  borderColor: "rgba(198, 120, 221, 0.45)",
  borderWidth: "1px",
  borderStyle: "solid",
  borderRadius: "2px",
  fontWeight: "bold",
});

function findBlocks(text: string): ControlFlowBlock[] {
  const lines = text.split("\n");
  const blocks: ControlFlowBlock[] = [];
  const stack: {
    type: string;
    line: number;
    col: number;
    endCol: number;
    branches: vscode.Range[];
  }[] = [];

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    // Find openers
    let match: RegExpExecArray | null;
    const openerRegex = /\{#(if|each|match|can)\b[^}]*\}/g;
    while ((match = openerRegex.exec(line)) !== null) {
      stack.push({
        type: match[1],
        line: i,
        col: match.index,
        endCol: match.index + match[0].length,
        branches: [],
      });
    }

    // Find branches (associate with top of stack)
    const branchRegex = /\{:(else(?:\s+if\b[^}]*)?|when\b[^}]*)\}/g;
    while ((match = branchRegex.exec(line)) !== null) {
      if (stack.length > 0) {
        const top = stack[stack.length - 1];
        top.branches.push(
          new vscode.Range(i, match.index, i, match.index + match[0].length),
        );
      }
    }

    // Find closers
    const closerRegex = /\{\/(if|each|match|can)\}/g;
    while ((match = closerRegex.exec(line)) !== null) {
      const closeType = match[1];
      // Find matching opener
      for (let s = stack.length - 1; s >= 0; s--) {
        if (stack[s].type === closeType) {
          const opener = stack.splice(s, 1)[0];
          blocks.push({
            type: closeType,
            openRange: new vscode.Range(
              opener.line,
              opener.col,
              opener.line,
              opener.endCol,
            ),
            closeRange: new vscode.Range(
              i,
              match.index,
              i,
              match.index + match[0].length,
            ),
            branchRanges: opener.branches,
          });
          break;
        }
      }
    }
  }

  return blocks;
}

function updateDecorations(editor: vscode.TextEditor) {
  if (editor.document.languageId !== "adapto") {
    editor.setDecorations(pairDecoration, []);
    editor.setDecorations(activePairDecoration, []);
    return;
  }

  const text = editor.document.getText();
  const blocks = findBlocks(text);
  const cursorPos = editor.selection.active;

  const activeRanges: vscode.Range[] = [];
  const passiveRanges: vscode.Range[] = [];

  for (const block of blocks) {
    const allRanges = [
      block.openRange,
      block.closeRange,
      ...block.branchRanges,
    ];
    const cursorInBlock = allRanges.some((r) => r.contains(cursorPos));

    if (cursorInBlock) {
      activeRanges.push(...allRanges);
    }
  }

  editor.setDecorations(activePairDecoration, activeRanges);
}

export function registerControlFlowDecorations(
  context: vscode.ExtensionContext,
) {
  // Debounce updates
  let timeout: NodeJS.Timeout | undefined;

  function triggerUpdate(editor: vscode.TextEditor) {
    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(() => updateDecorations(editor), 50);
  }

  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      if (editor) triggerUpdate(editor);
    }),
  );

  context.subscriptions.push(
    vscode.window.onDidChangeTextEditorSelection((e) => {
      triggerUpdate(e.textEditor);
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

  // Initial
  if (vscode.window.activeTextEditor) {
    triggerUpdate(vscode.window.activeTextEditor);
  }
}
