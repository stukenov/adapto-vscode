import * as vscode from "vscode";

/**
 * Block Indicators — Status bar items showing which blocks are present
 * in the active .adapto file. Each block type gets a subtle indicator
 * that lights up when present, dims when absent.
 *
 * Visual language: uses codicon icons, left-aligned, grouped together.
 * The order matches the canonical .adapto file order:
 * route > script > template > style > resource > layout
 */

interface BlockDef {
  tag: string;
  icon: string;
  label: string;
  tooltip: string;
}

const BLOCKS: BlockDef[] = [
  {
    tag: "route",
    icon: "$(globe)",
    label: "R",
    tooltip: "Route block",
  },
  {
    tag: "script",
    icon: "$(code)",
    label: "S",
    tooltip: "Script block",
  },
  {
    tag: "template",
    icon: "$(window)",
    label: "T",
    tooltip: "Template block",
  },
  {
    tag: "style",
    icon: "$(paintcan)",
    label: "St",
    tooltip: "Style block",
  },
  {
    tag: "resource",
    icon: "$(database)",
    label: "Rs",
    tooltip: "Resource block",
  },
  {
    tag: "layout",
    icon: "$(layout)",
    label: "L",
    tooltip: "Layout block",
  },
];

let statusItem: vscode.StatusBarItem | undefined;

function detectBlocks(text: string): Set<string> {
  const present = new Set<string>();
  for (const block of BLOCKS) {
    const regex = new RegExp(`<${block.tag}[\\s>]`);
    if (regex.test(text)) {
      present.add(block.tag);
    }
  }
  return present;
}

function updateIndicator(editor: vscode.TextEditor | undefined) {
  if (!statusItem) return;

  if (!editor || editor.document.languageId !== "adapto") {
    statusItem.hide();
    return;
  }

  const text = editor.document.getText();
  const present = detectBlocks(text);

  const parts: string[] = [];
  for (const block of BLOCKS) {
    if (present.has(block.tag)) {
      parts.push(`${block.icon}`);
    }
  }

  if (parts.length === 0) {
    statusItem.hide();
    return;
  }

  statusItem.text = parts.join(" ");

  const tooltipLines = BLOCKS.map((b) => {
    const check = present.has(b.tag) ? "$(check)" : "$(circle-slash)";
    return `${check} ${b.tooltip}`;
  });
  statusItem.tooltip = tooltipLines.join("\n");
  statusItem.show();
}

export function registerBlockIndicators(context: vscode.ExtensionContext) {
  statusItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    200,
  );
  statusItem.name = "Adapto Blocks";
  context.subscriptions.push(statusItem);

  // Update on editor change
  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor(updateIndicator),
  );

  // Update on document change (real-time as user types)
  context.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((e) => {
      const editor = vscode.window.activeTextEditor;
      if (editor && e.document === editor.document) {
        updateIndicator(editor);
      }
    }),
  );

  // Initial update
  updateIndicator(vscode.window.activeTextEditor);
}
