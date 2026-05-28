import * as vscode from "vscode";

let devServerTerminal: vscode.Terminal | undefined;
let statusBarItem: vscode.StatusBarItem;

export function registerCommands(context: vscode.ExtensionContext) {
  statusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Left,
    100,
  );
  statusBarItem.command = "adapto.toggleDevServer";
  updateStatusBar(false);
  statusBarItem.show();
  context.subscriptions.push(statusBarItem);

  context.subscriptions.push(
    vscode.commands.registerCommand("adapto.newProject", newProject),
    vscode.commands.registerCommand("adapto.startDevServer", () =>
      startDevServer(),
    ),
    vscode.commands.registerCommand("adapto.stopDevServer", stopDevServer),
    vscode.commands.registerCommand("adapto.toggleDevServer", () =>
      toggleDevServer(),
    ),
    vscode.commands.registerCommand("adapto.build", build),
    vscode.commands.registerCommand("adapto.check", check),
    vscode.commands.registerCommand(
      "adapto.generateResource",
      generateResource,
    ),
    vscode.commands.registerCommand("adapto.showRoutes", showRoutes),
    vscode.commands.registerCommand("adapto.doctor", doctor),
  );
}

async function newProject() {
  const name = await vscode.window.showInputBox({
    prompt: "Project name",
    placeHolder: "my-app",
  });
  if (!name) return;

  const folder = await vscode.window.showOpenDialog({
    canSelectFolders: true,
    canSelectFiles: false,
    openLabel: "Select parent directory",
  });
  if (!folder?.[0]) return;

  const terminal = vscode.window.createTerminal("Adapto");
  terminal.show();
  terminal.sendText(`cd "${folder[0].fsPath}" && adapto new ${name}`);
}

function startDevServer() {
  if (devServerTerminal) {
    devServerTerminal.show();
    return;
  }

  devServerTerminal = vscode.window.createTerminal({
    name: "Adapto Dev",
    iconPath: new vscode.ThemeIcon("server"),
  });
  devServerTerminal.show();
  devServerTerminal.sendText("adapto dev");
  updateStatusBar(true);

  vscode.window.onDidCloseTerminal((t) => {
    if (t === devServerTerminal) {
      devServerTerminal = undefined;
      updateStatusBar(false);
    }
  });
}

function stopDevServer() {
  if (devServerTerminal) {
    devServerTerminal.dispose();
    devServerTerminal = undefined;
    updateStatusBar(false);
  }
}

function toggleDevServer() {
  if (devServerTerminal) {
    stopDevServer();
  } else {
    startDevServer();
  }
}

function build() {
  const terminal = vscode.window.createTerminal("Adapto Build");
  terminal.show();
  terminal.sendText("adapto build --release");
}

function check() {
  const terminal = vscode.window.createTerminal("Adapto Check");
  terminal.show();
  terminal.sendText("adapto check");
}

async function generateResource() {
  const name = await vscode.window.showInputBox({
    prompt: "Resource name (PascalCase)",
    placeHolder: "Customer",
  });
  if (!name) return;

  const terminal = vscode.window.createTerminal("Adapto Generate");
  terminal.show();
  terminal.sendText(`adapto generate resource ${name}`);
}

function showRoutes() {
  const terminal = vscode.window.createTerminal("Adapto Routes");
  terminal.show();
  terminal.sendText("adapto routes");
}

function doctor() {
  const terminal = vscode.window.createTerminal("Adapto Doctor");
  terminal.show();
  terminal.sendText("adapto doctor");
}

function updateStatusBar(running: boolean) {
  if (running) {
    statusBarItem.text = "$(server) Adapto: Running";
    statusBarItem.tooltip = "Click to stop dev server";
  } else {
    statusBarItem.text = "$(circle-slash) Adapto: Stopped";
    statusBarItem.tooltip = "Click to start dev server";
  }
}
