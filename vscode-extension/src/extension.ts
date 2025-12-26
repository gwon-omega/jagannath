import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  // Get server path from configuration or use default
  const config = vscode.workspace.getConfiguration("jagannath");
  let serverPath = config.get<string>("serverPath");

  if (!serverPath) {
    // Default to bundled server
    serverPath = context.asAbsolutePath(path.join("server", "jagannath-lsp"));
  }

  // Server options
  const serverOptions: ServerOptions = {
    run: {
      command: serverPath,
      transport: TransportKind.stdio,
    },
    debug: {
      command: serverPath,
      transport: TransportKind.stdio,
    },
  };

  // Client options
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "jagannath" }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher(
        "**/*.{jag,jagannath}"
      ),
    },
  };

  // Create and start client
  client = new LanguageClient(
    "jagannathLanguageServer",
    "Jagannath Language Server",
    serverOptions,
    clientOptions
  );

  // Register commands
  context.subscriptions.push(
    vscode.commands.registerCommand("jagannath.build", buildCommand),
    vscode.commands.registerCommand("jagannath.run", runCommand),
    vscode.commands.registerCommand("jagannath.format", formatCommand)
  );

  // Start the client
  client.start();

  console.log("Jagannath extension activated");
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}

async function buildCommand() {
  const terminal = vscode.window.createTerminal("Jagannath Build");
  const editor = vscode.window.activeTextEditor;

  if (editor && editor.document.languageId === "jagannath") {
    const filePath = editor.document.fileName;
    const config = vscode.workspace.getConfiguration("jagannath");
    const guna = config.get<string>("guna", "rajas");

    terminal.sendText(`jagannath build --${guna} "${filePath}"`);
    terminal.show();
  } else {
    vscode.window.showErrorMessage("No Jagannath file is open");
  }
}

async function runCommand() {
  const terminal = vscode.window.createTerminal("Jagannath Run");
  const editor = vscode.window.activeTextEditor;

  if (editor && editor.document.languageId === "jagannath") {
    const filePath = editor.document.fileName;
    terminal.sendText(`jagannath run "${filePath}"`);
    terminal.show();
  } else {
    vscode.window.showErrorMessage("No Jagannath file is open");
  }
}

async function formatCommand() {
  const editor = vscode.window.activeTextEditor;

  if (editor && editor.document.languageId === "jagannath") {
    await vscode.commands.executeCommand("editor.action.formatDocument");
  } else {
    vscode.window.showErrorMessage("No Jagannath file is open");
  }
}
