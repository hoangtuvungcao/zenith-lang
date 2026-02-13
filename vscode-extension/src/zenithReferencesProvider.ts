import * as vscode from 'vscode';

export class ZenithReferencesProvider implements vscode.ReferenceProvider {
    provideReferences(
        document: vscode.TextDocument,
        position: vscode.Position,
        context: vscode.ReferenceContext,
        token: vscode.CancellationToken
    ): vscode.Location[] {
        const word = document.getText(document.getWordRangeAtPosition(position));
        const locations: vscode.Location[] = [];
        
        // Find all references to the word
        const text = document.getText();
        const lines = text.split('\n');

        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            const regex = new RegExp('\\b' + word + '\\b', 'g');
            let match;
            
            while ((match = regex.exec(line)) !== null) {
                // Skip the definition if we're looking for references only
                if (context.includeDeclaration || !this.isDefinition(line, match.index, word)) {
                    const range = new vscode.Range(i, match.index, i, match.index + word.length);
                    locations.push(new vscode.Location(document.uri, range));
                }
            }
        }

        return locations;
    }

    private isDefinition(line: string, index: number, word: string): boolean {
        const before = line.substring(0, index).trim();
        
        // Check if it's a function definition
        if (before.endsWith('func ')) {
            return true;
        }
        
        // Check if it's a variable definition
        if (before.endsWith('var ')) {
            return true;
        }
        
        // Check if it's a parameter definition
        if (before.includes('(') && before.includes(',') && !before.includes(')')) {
            return true;
        }
        
        return false;
    }
}
