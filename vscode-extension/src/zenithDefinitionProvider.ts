import * as vscode from 'vscode';

export class ZenithDefinitionProvider implements vscode.DefinitionProvider {
    provideDefinition(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): vscode.Location | vscode.Location[] | undefined {
        const word = document.getText(document.getWordRangeAtPosition(position));
        
        // Look for function definitions
        const functionDefinitions = this.findFunctionDefinitions(document, word);
        if (functionDefinitions.length > 0) {
            return functionDefinitions;
        }
        
        // Look for variable definitions
        const variableDefinitions = this.findVariableDefinitions(document, word);
        if (variableDefinitions.length > 0) {
            return variableDefinitions;
        }
        
        return undefined;
    }

    private findFunctionDefinitions(document: vscode.TextDocument, functionName: string): vscode.Location[] {
        const locations: vscode.Location[] = [];
        const text = document.getText();
        const lines = text.split('\n');

        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            const match = line.match(/func\s+(' + functionName + ')\s*\(/);
            if (match) {
                const range = new vscode.Range(i, match.index!, i, match.index! + functionName.length);
                locations.push(new vscode.Location(document.uri, range));
            }
        }

        return locations;
    }

    private findVariableDefinitions(document: vscode.TextDocument, variableName: string): vscode.Location[] {
        const locations: vscode.Location[] = [];
        const text = document.getText();
        const lines = text.split('\n');

        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            const match = line.match(/var\s+(' + variableName + ')\s*(?::\s*\w+)?\s*=/);
            if (match) {
                const range = new vscode.Range(i, match.index!, i, match.index! + variableName.length);
                locations.push(new vscode.Location(document.uri, range));
            }
        }

        return locations;
    }
}
