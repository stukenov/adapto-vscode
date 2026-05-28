import * as vscode from "vscode";
import * as path from "path";

export class AdaptoPreviewProvider {
  private panel: vscode.WebviewPanel | undefined;
  private context: vscode.ExtensionContext;
  private disposables: vscode.Disposable[] = [];

  constructor(context: vscode.ExtensionContext) {
    this.context = context;
  }

  show() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== "adapto") {
      vscode.window.showWarningMessage("Open an .adapto file first");
      return;
    }

    if (this.panel) {
      this.panel.reveal(vscode.ViewColumn.Beside);
      this.update(editor.document);
      return;
    }

    this.panel = vscode.window.createWebviewPanel(
      "adaptoPreview",
      "Adapto Preview",
      vscode.ViewColumn.Beside,
      {
        enableScripts: true,
        localResourceRoots: [
          vscode.Uri.file(path.join(this.context.extensionPath, "media")),
        ],
      },
    );

    const cssUri = this.panel.webview.asWebviewUri(
      vscode.Uri.file(
        path.join(this.context.extensionPath, "media", "preview.css"),
      ),
    );

    this.panel.webview.html = this.getBaseHtml(cssUri);
    this.update(editor.document);

    this.panel.onDidDispose(() => {
      this.panel = undefined;
      this.disposables.forEach((d) => d.dispose());
      this.disposables = [];
    });

    this.disposables.push(
      vscode.workspace.onDidChangeTextDocument((e) => {
        if (
          e.document.languageId === "adapto" &&
          e.document === vscode.window.activeTextEditor?.document
        ) {
          this.update(e.document);
        }
      }),
    );

    this.disposables.push(
      vscode.window.onDidChangeActiveTextEditor((editor) => {
        if (editor?.document.languageId === "adapto") {
          this.update(editor.document);
        }
      }),
    );
  }

  private update(document: vscode.TextDocument) {
    if (!this.panel) return;

    const text = document.getText();
    const fileName = path.basename(document.fileName);

    const templateMatch = text.match(/<template>([\s\S]*?)<\/template>/);
    const routeMatch = text.match(/path:\s*"([^"]+)"/);
    const stateMatches = [
      ...text.matchAll(/state\s+(\w+):\s+(\w+)\s*=\s*(.+)/g),
    ];

    const templateContent = templateMatch?.[1]?.trim() || "";
    const routePath = routeMatch?.[1] || "/";

    let preview = templateContent;
    for (const [, name, , defaultVal] of stateMatches) {
      const cleaned = defaultVal.replace(/"/g, "").trim();
      preview = preview.replace(new RegExp(`\\{${name}\\}`, "g"), cleaned);
    }

    preview = preview
      .replace(/\{#if\s+[^}]+\}/g, "<!-- if -->")
      .replace(/\{:else(?:\s+if\s+[^}]+)?\}/g, "")
      .replace(/\{\/if\}/g, "<!-- /if -->")
      .replace(/\{#each\s+[^}]+\}/g, "<!-- each -->")
      .replace(/\{\/each\}/g, "<!-- /each -->")
      .replace(/\{#match\s+[^}]+\}/g, "<!-- match -->")
      .replace(/\{:when\s+[^}]+\}/g, "")
      .replace(/\{\/match\}/g, "<!-- /match -->")
      .replace(/\{#can\s+"[^"]+"\}/g, "")
      .replace(/\{\/can\}/g, "")
      .replace(/\{@html\s+([^}]+)\}/g, "$1")
      .replace(/\{[^}]+\}/g, '<span style="opacity:0.5">{...}</span>');

    this.panel.webview.postMessage({
      type: "update",
      fileName,
      routePath,
      content: preview,
    });
  }

  private getBaseHtml(cssUri: vscode.Uri): string {
    const nonce = getNonce();
    return `<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${this.panel!.webview.cspSource}; script-src 'nonce-${nonce}';">
  <link rel="stylesheet" href="${cssUri}">
</head>
<body>
  <div class="preview-header">
    <h2 id="fileName">Preview</h2>
    <span class="route-info" id="routePath">/</span>
  </div>
  <div class="preview-content" id="content">
    <div class="empty-state">
      <p>Open an .adapto file to see preview</p>
    </div>
  </div>
  <script nonce="${nonce}">
    window.addEventListener('message', event => {
      const msg = event.data;
      if (msg.type === 'update') {
        document.getElementById('fileName').textContent = msg.fileName;
        document.getElementById('routePath').textContent = msg.routePath;
        document.getElementById('content').innerHTML = msg.content;
      }
    });
  </script>
</body>
</html>`;
  }

  dispose() {
    this.panel?.dispose();
    this.disposables.forEach((d) => d.dispose());
  }
}

function getNonce(): string {
  let text = "";
  const possible =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}

export function registerPreview(context: vscode.ExtensionContext) {
  const provider = new AdaptoPreviewProvider(context);

  context.subscriptions.push(
    vscode.commands.registerCommand("adapto.openPreview", () =>
      provider.show(),
    ),
  );
}
