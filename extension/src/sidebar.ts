import * as vscode from "vscode";
import * as path from "path";

export class AdaptoTreeProvider
  implements vscode.TreeDataProvider<AdaptoTreeItem>
{
  private _onDidChangeTreeData = new vscode.EventEmitter<
    AdaptoTreeItem | undefined
  >();
  readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

  constructor(private workspaceRoot: string | undefined) {}

  refresh(): void {
    this._onDidChangeTreeData.fire(undefined);
  }

  getTreeItem(element: AdaptoTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: AdaptoTreeItem): Promise<AdaptoTreeItem[]> {
    if (!this.workspaceRoot) {
      return [];
    }

    if (!element) {
      return [
        new AdaptoTreeItem(
          "Routes",
          vscode.TreeItemCollapsibleState.Collapsed,
          "routes",
        ),
        new AdaptoTreeItem(
          "Components",
          vscode.TreeItemCollapsibleState.Collapsed,
          "components",
        ),
        new AdaptoTreeItem(
          "Resources",
          vscode.TreeItemCollapsibleState.Collapsed,
          "resources",
        ),
      ];
    }

    if (element.category === "components") {
      return this.findAdaptoFiles();
    }

    return [];
  }

  private async findAdaptoFiles(): Promise<AdaptoTreeItem[]> {
    const pattern = new vscode.RelativePattern(
      this.workspaceRoot!,
      "**/*.adapto",
    );
    const files = await vscode.workspace.findFiles(pattern);

    return files.map((file) => {
      const rel = path.relative(this.workspaceRoot!, file.fsPath);
      const item = new AdaptoTreeItem(
        rel,
        vscode.TreeItemCollapsibleState.None,
        "file",
      );
      item.command = {
        command: "vscode.open",
        title: "Open",
        arguments: [file],
      };
      item.resourceUri = file;
      item.iconPath = new vscode.ThemeIcon("file-code");
      return item;
    });
  }
}

export class AdaptoTreeItem extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState,
    public readonly category: string,
  ) {
    super(label, collapsibleState);

    switch (category) {
      case "routes":
        this.iconPath = new vscode.ThemeIcon("symbol-interface");
        break;
      case "components":
        this.iconPath = new vscode.ThemeIcon("symbol-class");
        break;
      case "resources":
        this.iconPath = new vscode.ThemeIcon("database");
        break;
    }
  }
}

export function registerSidebar(context: vscode.ExtensionContext) {
  const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  const treeProvider = new AdaptoTreeProvider(workspaceRoot);

  vscode.window.registerTreeDataProvider("adaptoExplorer", treeProvider);

  context.subscriptions.push(
    vscode.commands.registerCommand("adapto.refreshExplorer", () =>
      treeProvider.refresh(),
    ),
  );

  const watcher = vscode.workspace.createFileSystemWatcher("**/*.adapto");
  watcher.onDidCreate(() => treeProvider.refresh());
  watcher.onDidDelete(() => treeProvider.refresh());
  watcher.onDidChange(() => treeProvider.refresh());
  context.subscriptions.push(watcher);
}
