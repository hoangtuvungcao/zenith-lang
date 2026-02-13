import * as vscode from 'vscode';

export class ZenithCodeLensProvider implements vscode.CodeLensProvider {
    private _onDidChangeCodeLenses: vscode.EventEmitter<void> = new vscode.EventEmitter<void>();
    readonly onDidChangeCodeLenses: vscode.Event<void> = this._onDidChangeCodeLenses.event;

    provideCodeLenses(document: vscode.TextDocument): vscode.CodeLens[] {
        const codeLenses: vscode.CodeLens[] = [];
        
        if (document.languageId !== 'zenith') {
            return codeLenses;
        }

        const text = document.getText();
        const lines = text.split('\n');

        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            
            // Add code lens for functions
            const funcMatch = line.match(/func\s+(\w+)\s*\(/);
            if (funcMatch) {
                const range = new vscode.Range(i, 0, i, line.length);
                codeLenses.push(new vscode.CodeLens(range, {
                    title: '▶ Run',
                    command: 'zenith.runFunction',
                    arguments: [funcMatch[1]]
                }));
            }

            // Add code lens for main function
            if (line.includes('func main(')) {
                const range = new vscode.Range(i, 0, i, line.length);
                codeLenses.push(new vscode.CodeLens(range, {
                    title: '▶ Run Main',
                    command: 'zenith.runFile'
                }));
            }
        }

        return codeLenses;
    }

    refresh(): void {
        this._onDidChangeCodeLenses.fire();
    }
}
