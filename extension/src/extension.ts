import * as path from "path";
import * as fs from "fs";
import * as os from "os";
import * as vscode from "vscode";
import { workspace, ExtensionContext } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";
import { registerCommands } from "./commands";
import { registerSidebar } from "./sidebar";
import { registerPreview } from "./preview";
import { registerBlockIndicators } from "./block-indicators";
import { registerControlFlowDecorations } from "./control-flow-decorations";
import { registerSecurityLens } from "./security-lens";
import { registerBlockSeparators } from "./block-separators";

let client: LanguageClient | undefined;

function getConfig(): vscode.WorkspaceConfiguration {
  return workspace.getConfiguration("adapto");
}

function getServerPath(context: ExtensionContext): string {
  const config = getConfig();
  const customPath = config.get<string>("lsp.path", "");
  if (customPath) return customPath;

  const platform = os.platform();
  const arch = os.arch();

  let binaryName: string;
  if (platform === "darwin" && arch === "arm64") {
    binaryName = "adapto-lsp-darwin-arm64";
  } else if (platform === "darwin") {
    binaryName = "adapto-lsp-darwin-x64";
  } else if (platform === "linux") {
    binaryName = "adapto-lsp-linux-x64";
  } else if (platform === "win32") {
    binaryName = "adapto-lsp-win32-x64.exe";
  } else {
    binaryName = "adapto-lsp";
  }

  const bundled = path.join(context.extensionPath, "bin", binaryName);
  try {
    fs.accessSync(bundled, fs.constants.X_OK);
    return bundled;
  } catch {
    return "adapto-lsp";
  }
}

export function activate(context: ExtensionContext) {
  const config = getConfig();

  if (config.get<boolean>("lsp.enabled", true)) {
    const serverPath = getServerPath(context);
    const serverOptions: ServerOptions = { command: serverPath, args: [] };
    const clientOptions: LanguageClientOptions = {
      documentSelector: [{ scheme: "file", language: "adapto" }],
      synchronize: {
        fileEvents: workspace.createFileSystemWatcher("**/*.adapto"),
      },
    };

    client = new LanguageClient(
      "adapto-lsp",
      "Adapto Language Server",
      serverOptions,
      clientOptions,
    );
    client.start();
  }

  registerCommands(context);
  registerSidebar(context);
  registerPreview(context);

  if (config.get<boolean>("blockIndicators.enabled", true)) {
    registerBlockIndicators(context);
  }
  if (config.get<boolean>("controlFlowHighlight.enabled", true)) {
    registerControlFlowDecorations(context);
  }
  if (config.get<boolean>("securityLens.enabled", true)) {
    registerSecurityLens(context);
  }
  if (config.get<boolean>("blockSeparators.enabled", true)) {
    registerBlockSeparators(context);
  }

  if (config.get<boolean>("format.onSave", false)) {
    context.subscriptions.push(
      workspace.onWillSaveTextDocument((e) => {
        if (e.document.languageId === "adapto") {
          e.waitUntil(
            vscode.commands.executeCommand("editor.action.formatDocument"),
          );
        }
      }),
    );
  }

  if (config.get<boolean>("preview.autoOpen", false)) {
    context.subscriptions.push(
      vscode.window.onDidChangeActiveTextEditor((editor) => {
        if (editor?.document.languageId === "adapto") {
          vscode.commands.executeCommand("adapto.openPreview");
        }
      }),
    );
  }
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
