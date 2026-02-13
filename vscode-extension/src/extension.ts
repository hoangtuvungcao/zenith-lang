import * as vscode from 'vscode';
import * as path from 'path';
import { ZenithLanguageServer } from './zenithLanguageServer';
import { ZenithFormatter } from './zenithFormatter';
import { ZenithSymbolProvider } from './zenithSymbols';
import { ZenithDefinitionProvider } from './zenithDefinitionProvider';
import { ZenithReferencesProvider } from './zenithReferencesProvider';
import { ZenithCodeLensProvider } from './zenithCodeLensProvider';

let outputChannel: vscode.OutputChannel;
let statusBarItem: vscode.StatusBarItem;
let languageServer: ZenithLanguageServer;
let formatter: ZenithFormatter;
let symbolProvider: ZenithSymbolProvider;
let definitionProvider: ZenithDefinitionProvider;
let referencesProvider: ZenithReferencesProvider;
let codeLensProvider: ZenithCodeLensProvider;

export function activate(context: vscode.ExtensionContext) {
    console.log('Zenith Professional Extension is now active!');

    // Create output channel
    outputChannel = vscode.window.createOutputChannel('Zenith');
    context.subscriptions.push(outputChannel);

    // Create status bar item
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = '$(play) Zenith';
    statusBarItem.tooltip = 'Run Zenith File';
    statusBarItem.command = 'zenith.runFile';
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);

    // Initialize language server
    languageServer = new ZenithLanguageServer(outputChannel);
    context.subscriptions.push(languageServer);

    // Initialize formatter
    formatter = new ZenithFormatter();
    context.subscriptions.push(formatter);

    // Initialize providers
    symbolProvider = new ZenithSymbolProvider();
    definitionProvider = new ZenithDefinitionProvider();
    referencesProvider = new ZenithReferencesProvider();
    codeLensProvider = new ZenithCodeLensProvider();

    // Register language features
    const completionProvider = vscode.languages.registerCompletionItemProvider(
        { scheme: 'file', language: 'zenith' },
        new ZenithCompletionProvider(),
        '.', '(', '"', "'"
    );
    context.subscriptions.push(completionProvider);

    const hoverProvider = vscode.languages.registerHoverProvider(
        { scheme: 'file', language: 'zenith' },
        new ZenithHoverProvider()
    );
    context.subscriptions.push(hoverProvider);

    const formattingProvider = vscode.languages.registerDocumentFormattingEditProvider(
        { scheme: 'file', language: 'zenith' },
        formatter
    );
    context.subscriptions.push(formattingProvider);

    const symbolProviderReg = vscode.languages.registerDocumentSymbolProvider(
        { scheme: 'file', language: 'zenith' },
        symbolProvider
    );
    context.subscriptions.push(symbolProviderReg);

    const definitionProviderReg = vscode.languages.registerDefinitionProvider(
        { scheme: 'file', language: 'zenith' },
        definitionProvider
    );
    context.subscriptions.push(definitionProviderReg);

    const referencesProviderReg = vscode.languages.registerReferenceProvider(
        { scheme: 'file', language: 'zenith' },
        referencesProvider
    );
    context.subscriptions.push(referencesProviderReg);

    const codeLensProviderReg = vscode.languages.registerCodeLensProvider(
        { scheme: 'file', language: 'zenith' },
        codeLensProvider
    );
    context.subscriptions.push(codeLensProviderReg);

    // Register commands
    const runFileCommand = vscode.commands.registerCommand('zenith.runFile', () => {
        runZenithFile();
    });

    const runFunctionCommand = vscode.commands.registerCommand('zenith.runFunction', (functionName: string) => {
        runZenithFunction(functionName);
    });

    const debugFunctionCommand = vscode.commands.registerCommand('zenith.debugFunction', (functionName: string) => {
        debugZenithFunction(functionName);
    });

    const showModuleDocsCommand = vscode.commands.registerCommand('zenith.showModuleDocs', (moduleName: string) => {
        showModuleDocumentation(moduleName);
    });

    // Register all commands
    context.subscriptions.push(
        runFileCommand,
        runFunctionCommand,
        debugFunctionCommand,
        showModuleDocsCommand
    );

    // File watcher for real-time diagnostics
    const fileWatcher = vscode.workspace.createFileSystemWatcher('**/*.zn');
    fileWatcher.onDidChange(uri => {
        const document = vscode.workspace.textDocuments.find(doc => doc.uri.fsPath === uri.fsPath);
        if (document) {
            languageServer.provideDiagnostics(document);
        }
    });
    fileWatcher.onDidCreate(uri => {
        outputChannel.appendLine(`File created: ${uri.fsPath}`);
    });
    fileWatcher.onDidDelete(uri => {
        outputChannel.appendLine(`File deleted: ${uri.fsPath}`);
    });
    context.subscriptions.push(fileWatcher);

    // Update diagnostics when document changes
    vscode.workspace.onDidChangeTextDocument(event => {
        if (event.document.languageId === 'zenith') {
            languageServer.provideDiagnostics(event.document);
        }
    });

    // Initial diagnostics for all open documents
    vscode.workspace.textDocuments.forEach(doc => {
        if (doc.languageId === 'zenith') {
            languageServer.provideDiagnostics(doc);
        }
    });

    outputChannel.appendLine('Zenith Professional Extension activated successfully!');
}

function runZenithFile() {
    const editor = vscode.window.activeTextEditor;
    if (editor && editor.document.fileName.endsWith('.zn')) {
        const config = vscode.workspace.getConfiguration('zenith');
        const compilerPath = config.get<string>('compilerPath') || 'zenith';

        const terminal = vscode.window.createTerminal('Zenith Run');
        terminal.sendText(`${compilerPath} run "${editor.document.fileName}"`);
        terminal.show();
        outputChannel.appendLine(`Running: ${editor.document.fileName}`);

        // Show success message
        vscode.window.showInformationMessage(`Running ${path.basename(editor.document.fileName)}...`);
    } else {
        vscode.window.showErrorMessage('Please open a Zenith file (.zn) to run');
    }
}

function runZenithFunction(functionName: string) {
    const editor = vscode.window.activeTextEditor;
    if (editor && editor.document.fileName.endsWith('.zn')) {
        // Create a temporary file to run just the function
        const tempContent = `
// Temporary file to run function: ${functionName}
${editor.document.getText()}

// Call the function
${functionName}();
`;

        const tempPath = path.join(path.dirname(editor.document.fileName), `temp_${functionName}.zn`);
        const fs = require('fs');
        fs.writeFileSync(tempPath, tempContent);

        const config = vscode.workspace.getConfiguration('zenith');
        const compilerPath = config.get<string>('compilerPath') || 'zenith';

        const terminal = vscode.window.createTerminal('Zenith Function Run');
        terminal.sendText(`${compilerPath} run "${tempPath}"`);
        terminal.show();
        outputChannel.appendLine(`Running function: ${functionName}`);

        // Clean up temp file after a delay
        setTimeout(() => {
            try {
                fs.unlinkSync(tempPath);
            } catch (e) {
                // Ignore cleanup errors
            }
        }, 5000);
    }
}

function debugZenithFunction(functionName: string) {
    const editor = vscode.window.activeTextEditor;
    if (editor && editor.document.fileName.endsWith('.zn')) {
        const config = vscode.workspace.getConfiguration('zenith');
        const compilerPath = config.get<string>('compilerPath') || 'zenith';

        const terminal = vscode.window.createTerminal('Zenith Debug');
        terminal.sendText(`echo "Debugging function: ${functionName}"`);
        terminal.sendText(`${compilerPath} run "${editor.document.fileName}"`);
        terminal.show();
        outputChannel.appendLine(`Debugging function: ${functionName}`);
    }
}

function showModuleDocumentation(moduleName: string) {
    const panel = vscode.window.createWebviewPanel(
        'zenithModuleDocs',
        `Zenith Module: ${moduleName}`,
        vscode.ViewColumn.One,
        {}
    );

    panel.webview.html = `
    <!DOCTYPE html>
    <html>
    <head>
        <title>Zenith Module: ${moduleName}</title>
        <style>
            body { font-family: Arial, sans-serif; padding: 20px; }
            h1 { color: #007acc; }
            .function { background: #f0f0f0; padding: 10px; margin: 10px 0; border-radius: 5px; }
            .param { color: #666; }
            .return { color: #0066cc; }
        </style>
    </head>
    <body>
        <h1>� Zenith Module: ${moduleName}</h1>
        <div class="info">
            <p><strong>Module:</strong> ${moduleName}</p>
            <p><strong>Type:</strong> Standard Library</p>
            <p><strong>Status:</strong> Production Ready</p>
        </div>
        
        <h2>Available Functions</h2>
        <div class="function">
            <h3>main()</h3>
            <p class="return">Return: void</p>
            <p>Main entry point function</p>
        </div>
        
        <h2>Usage Example</h2>
        <pre><code>import ${moduleName}

// Use the module functions
main();</code></pre>
        
        <h2>Documentation</h2>
        <p>For more information, see the <a href="https://docs.zenith-lang.org">Zenith Documentation</a>.</p>
    </body>
    </html>
    `;
}

class ZenithCompletionProvider implements vscode.CompletionItemProvider {
    provideCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position
    ): vscode.CompletionItem[] {
        const linePrefix = document.lineAt(position).text.substring(0, position.character);
        const items: vscode.CompletionItem[] = [];

        // Import suggestions
        if (linePrefix.startsWith('import')) {
            const modules = [
                'std.array', 'std.math', 'std.string', 'std.json', 'std.http', 'std.crypto',
                'std.datetime', 'std.encoding', 'std.validation', 'std.database', 'std.config',
                'std.os', 'std.process', 'std.filesystem', 'std.io', 'std.net', 'std.web',
                'std.ai', 'std.graphics', 'std.game', 'std.machine_learning', 'std.image_processing',
                'std.physics', 'std.finance', 'std.chemistry', 'std.audio', 'std.statistics',
                'std.logging', 'std.security'
            ];
            modules.forEach(module => {
                const item = new vscode.CompletionItem(module, vscode.CompletionItemKind.Module);
                item.documentation = new vscode.MarkdownString(`Import ${module} module`);
                item.insertText = module;
                items.push(item);
            });
        }

        // Function suggestions
        if (linePrefix.includes('.')) {
            const functions = [
                'len', 'str', 'range', 'print', 'sum', 'min', 'max', 'abs', 'sqrt', 'pow',
                'sin', 'cos', 'tan', 'log', 'exp', 'floor', 'ceil', 'round', 'contains',
                'index_of', 'slice', 'reverse_arr', 'unique', 'sort', 'filter', 'map'
            ];
            functions.forEach(func => {
                const item = new vscode.CompletionItem(func, vscode.CompletionItemKind.Function);
                item.documentation = new vscode.MarkdownString(`Use ${func} function`);
                items.push(item);
            });
        }

        // Keyword suggestions
        const keywords = [
            'func', 'var', 'for', 'if', 'else', 'return', 'import', 'while', 'break', 'continue',
            'match', 'class', 'struct', 'enum', 'trait', 'impl', 'type', 'const', 'mod', 'use'
        ];
        keywords.forEach(keyword => {
            const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
            item.documentation = new vscode.MarkdownString(`Use ${keyword} keyword`);
            items.push(item);
        });

        // Type suggestions
        const types = ['string', 'int', 'float', 'bool', 'array', 'object', 'void'];
        types.forEach(type => {
            const item = new vscode.CompletionItem(type, vscode.CompletionItemKind.TypeParameter);
            item.documentation = new vscode.MarkdownString(`Use ${type} type`);
            items.push(item);
        });

        return items;
    }
}

class ZenithHoverProvider implements vscode.HoverProvider {
    provideHover(
        document: vscode.TextDocument,
        position: vscode.Position
    ): vscode.Hover | undefined {
        const word = document.getText(document.getWordRangeAtPosition(position));
        const documentation: { [key: string]: string } = {
            'func': 'Defines a function\n\n```zenith\nfunc name(params) -> return_type {\n    // function body\n}\n```',
            'var': 'Declares a variable\n\n```zenith\nvar name: type = value;\n```',
            'for': 'Loop construct\n\n```zenith\nfor item in iterable {\n    // loop body\n}\n```',
            'if': 'Conditional statement\n\n```zenith\nif condition {\n    // if body\n} else {\n    // else body\n}\n```',
            'import': 'Import module\n\n```zenith\nimport module_name;\n```',
            'print': 'Print to console\n\n```zenith\nprint("message");\n```',
            'len': 'Get length of array/string\n\n```zenith\nlen(array_or_string)\n```',
            'str': 'Convert to string\n\n```zenith\nstr(value)\n```',
            'range': 'Create range of numbers\n\n```zenith\nrange(start, end)\n```',
            'sum': 'Calculate sum of array\n\n```zenith\nsum(array)\n```',
            'min': 'Find minimum value\n\n```zenith\nmin(array)\n```',
            'max': 'Find maximum value\n\n```zenith\nmax(array)\n```',
            'abs': 'Absolute value\n\n```zenith\nabs(number)\n```',
            'sqrt': 'Square root\n\n```zenith\nsqrt(number)\n```'
        };

        if (documentation[word]) {
            return new vscode.Hover(new vscode.MarkdownString(documentation[word]));
        }

        return undefined;
    }
}

export function deactivate() {
    console.log('Zenith Professional Extension deactivated');
}
