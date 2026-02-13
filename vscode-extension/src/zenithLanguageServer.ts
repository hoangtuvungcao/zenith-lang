import * as vscode from 'vscode';
import * as path from 'path';

export class ZenithLanguageServer {
    private outputChannel: vscode.OutputChannel;
    private diagnostics: vscode.DiagnosticCollection;

    constructor(outputChannel: vscode.OutputChannel) {
        this.outputChannel = outputChannel;
        this.diagnostics = vscode.languages.createDiagnosticCollection('zenith');
    }

    async provideDiagnostics(document: vscode.TextDocument): Promise<void> {
        if (document.languageId !== 'zenith') {
            return;
        }

        const fileName = document.fileName;
        const diagnostics: vscode.Diagnostic[] = [];

        // We need to save the file first or pass content via stdin (if supported).
        // Since sema takes a file path, we rely on the file being saved or use a temporary file.
        // For simplicity in this iteration, we run partially on saved files, 
        // OR we can write the current content to a temp file.
        // Let's use a temp file for accuracy with unsaved changes.

        const fs = require('fs');
        const path = require('path');
        const os = require('os');
        const cp = require('child_process');

        const tempDir = os.tmpdir();
        const tempFile = path.join(tempDir, `temp_${Date.now()}.zn`);

        try {
            fs.writeFileSync(tempFile, document.getText());

            // Execute zenith sema
            // We assume 'zenith' is in the PATH. If not, we might need configuration.
            // Using a Promise wrapper for exec
            const execPromise = (cmd: string): Promise<{ stdout: string, stderr: string }> => {
                return new Promise((resolve, reject) => {
                    cp.exec(cmd, (error: any, stdout: string, stderr: string) => {
                        // We resolve even on error because exit code 1 is expected for semantic errors
                        // If it's a real system error (like command not found), stdout will likely be empty.
                        resolve({ stdout, stderr });
                    });
                });
            };

            const config = vscode.workspace.getConfiguration('zenith');
            const compilerPath = config.get<string>('compilerPath') || 'zenith';
            const check = await execPromise(`${compilerPath} sema "${tempFile}"`);

            // Parse output
            // Output format looks like:
            // Error Semantic errors found:
            //   1. Undefined variable 'Missing' at line 2, column 23

            const lines = check.stdout.split('\n');
            let parsingErrors = false;

            // Regex to match error lines: "  1. Error message at line X, column Y"
            const errorRegex = /^\s*\d+\.\s+(.+?)\s+at\s+line\s+(\d+),\s+column\s+(\d+)/;

            for (const line of lines) {
                if (line.trim().startsWith('Error Semantic errors found:')) {
                    parsingErrors = true;
                    continue;
                }

                if (parsingErrors) {
                    const match = line.match(errorRegex);
                    if (match) {
                        const msg = match[1];
                        const lineNum = parseInt(match[2]) - 1; // VS Code is 0-indexed
                        const colNum = parseInt(match[3]) - 1;

                        // Create diagnostic
                        const range = new vscode.Range(lineNum, colNum, lineNum, colNum + 100); // Highlight rest of line
                        const diagnostic = new vscode.Diagnostic(
                            range,
                            msg,
                            vscode.DiagnosticSeverity.Error
                        );
                        diagnostics.push(diagnostic);
                    }
                }
            }

        } catch (e) {
            console.error("Error executing zenith sema:", e);
        } finally {
            // Cleanup temp file
            if (fs.existsSync(tempFile)) {
                fs.unlinkSync(tempFile);
            }
        }

        this.diagnostics.set(document.uri, diagnostics);
    }

    dispose() {
        this.diagnostics.dispose();
    }
}
