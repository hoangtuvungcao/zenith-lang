import * as vscode from 'vscode';

export class ZenithSymbolProvider implements vscode.DocumentSymbolProvider {
    provideDocumentSymbols(
        document: vscode.TextDocument,
        token: vscode.CancellationToken
    ): vscode.DocumentSymbol[] {
        const symbols: vscode.DocumentSymbol[] = [];
        const text = document.getText();
        const lines = text.split('\n');

        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            const lineNumber = i;

            // Function definitions
            const funcMatch = line.match(/func\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*(\w+))?/);
            if (funcMatch) {
                const name = funcMatch[1];
                const params = funcMatch[2];
                const returnType = funcMatch[3] || 'void';
                
                const range = new vscode.Range(lineNumber, 0, lineNumber, line.length);
                const selectionRange = new vscode.Range(lineNumber, line.indexOf(name), lineNumber, line.indexOf(name) + name.length);
                
                const symbol = new vscode.DocumentSymbol(
                    name,
                    `func ${name}(${params}) -> ${returnType}`,
                    vscode.SymbolKind.Function,
                    range,
                    selectionRange
                );
                
                symbols.push(symbol);
            }

            // Variable declarations
            const varMatch = line.match(/var\s+(\w+)\s*(?::\s*(\w+))?\s*=\s*(.+);/);
            if (varMatch) {
                const name = varMatch[1];
                const type = varMatch[2] || 'auto';
                const value = varMatch[3];
                
                const range = new vscode.Range(lineNumber, 0, lineNumber, line.length);
                const selectionRange = new vscode.Range(lineNumber, line.indexOf(name), lineNumber, line.indexOf(name) + name.length);
                
                const symbol = new vscode.DocumentSymbol(
                    name,
                    `${name}: ${type} = ${value}`,
                    vscode.SymbolKind.Variable,
                    range,
                    selectionRange
                );
                
                symbols.push(symbol);
            }

            // Import statements
            const importMatch = line.match(/import\s+(.+);/);
            if (importMatch) {
                const module = importMatch[1];
                
                const range = new vscode.Range(lineNumber, 0, lineNumber, line.length);
                const selectionRange = new vscode.Range(lineNumber, line.indexOf('import'), lineNumber, line.indexOf('import') + 6);
                
                const symbol = new vscode.DocumentSymbol(
                    module,
                    `import ${module}`,
                    vscode.SymbolKind.Module,
                    range,
                    selectionRange
                );
                
                symbols.push(symbol);
            }
        }

        return symbols;
    }
}
