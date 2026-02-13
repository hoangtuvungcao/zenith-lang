import * as vscode from 'vscode';

export class ZenithFormatter implements vscode.DocumentFormattingEditProvider {
    provideDocumentFormattingEdits(
        document: vscode.TextDocument,
        options: vscode.FormattingOptions,
        token: vscode.CancellationToken
    ): vscode.TextEdit[] {
        const text = document.getText();
        const formattedText = this.formatZenithCode(text, options);

        const fullRange = new vscode.Range(
            document.positionAt(0),
            document.positionAt(text.length)
        );

        return [vscode.TextEdit.replace(fullRange, formattedText)];
    }

    private formatZenithCode(text: string, options: vscode.FormattingOptions): string {
        const lines = text.split('\n');
        const indentSize = options.insertSpaces ? options.tabSize : 4;
        const indentChar = options.insertSpaces ? ' ' : '\t';

        let currentIndent = 0;
        const formattedLines: string[] = [];

        for (let line of lines) {
            let trimmedLine = line.trim();

            if (trimmedLine === '') {
                formattedLines.push('');
                continue;
            }

            // Protect literals (strings and comments)
            const literals: { placeholder: string, value: string }[] = [];
            let placeholderCount = 0;

            // Protect comments first
            trimmedLine = trimmedLine.replace(/\/\/.*/g, (match) => {
                const placeholder = `__ZENITH_LIT_${placeholderCount++}__`;
                literals.push({ placeholder, value: match });
                return placeholder;
            });

            // Protect strings
            trimmedLine = trimmedLine.replace(/"([^"\\]|\\.)*"/g, (match) => {
                const placeholder = `__ZENITH_LIT_${placeholderCount++}__`;
                literals.push({ placeholder, value: match });
                return placeholder;
            });

            // Check for closing braces at the START of the code on this line
            // We strip literals to check this accurately
            let codeOnly = trimmedLine;
            for (const lit of literals) {
                codeOnly = codeOnly.replace(lit.placeholder, '');
            }

            if (codeOnly.trim().startsWith('}')) {
                currentIndent = Math.max(0, currentIndent - 1);
            }

            // Apply formatting rules to the line (with protected literals)
            let formattedLine = this.applySpacing(trimmedLine);
            formattedLine = this.applyBrackets(formattedLine);

            // Restore literals (in reverse order to handle nesting if any, though regexes are flat here)
            for (let i = literals.length - 1; i >= 0; i--) {
                formattedLine = formattedLine.replace(literals[i].placeholder, literals[i].value);
            }

            // Apply indentation
            const indentation = indentChar.repeat(currentIndent * (options.insertSpaces ? indentSize : 1));
            const finalLine = indentation + formattedLine;
            formattedLines.push(finalLine);

            // Check for opening braces to increase next line's indent
            if (codeOnly.trim().endsWith('{')) {
                currentIndent++;
            }
        }

        return formattedLines.join('\n');
    }

    private applySpacing(text: string): string {
        // Simple approach: Add spaces around operators, then fix common issues
        let result = text
            .replace(/\s*([=+\-*/%<>!&|]+)\s*/g, ' $1 ') // Operators
            .replace(/\s*([,;:]{1})\s*/g, '$1 ')           // Punctuation
            .replace(/\s+/g, ' ');

        // Fix multi-char operators that might have been broken by a generic regex
        // but here we used [=+\-*/%<>!&|]+ so they might stay together.
        // Let's ensure common Zenith multi-char operators are correct
        result = result
            .replace(/\. \./g, '..')
            .replace(/- >/g, '->');

        // Fix unary operators: "!identifier", "-42", "-identifier"
        // These often follow an open paren, comma, or start of line
        result = result
            .replace(/(!|~)\s+/g, '$1')
            .replace(/(^|[(,;:=+\-*/%<>!&|])\s*([-+])\s*([a-zA-Z0-9_])/g, '$1$2$3');

        // Cleanup: remove space before closing paren/bracket, add after opening if needed (optional)
        result = result
            .replace(/\s+\)/g, ')')
            .replace(/\(\s+/g, '(')
            .replace(/\s+\]/g, ']')
            .replace(/\[\s+/g, '[');

        return result.trim();
    }

    private applyBrackets(text: string): string {
        return text
            .replace(/\s*\{/g, ' {')
            .replace(/\}\s*/g, '} ')
            .replace(/\s+/g, ' ')
            .trim();
    }

    dispose() {
    }
}
