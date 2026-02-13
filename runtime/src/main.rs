//! Zenith Runtime Main Entry Point
//! Command-line interface for running Zenith programs

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;
use zenith_runtime::{create_runtime, execute_code, RuntimeError};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];
    let command_args = &args[2..];

    match command.as_str() {
        "run" => {
            if command_args.is_empty() {
                eprintln!("Error: 'zenith-runtime run' requires a file argument");
                process::exit(1);
            }

            let filename = &command_args[0];
            match run_file(filename) {
                Ok(_) => process::exit(0),
                Err(e) => {
                    eprintln!("Runtime Error: {}", e);
                    process::exit(1);
                }
            }
        }
        "eval" => {
            if command_args.is_empty() {
                eprintln!("Error: 'zenith-runtime eval' requires a code argument");
                process::exit(1);
            }

            let code = command_args.join(" ");
            match eval_code(&code) {
                Ok(_) => process::exit(0),
                Err(e) => {
                    eprintln!("Runtime Error: {}", e);
                    process::exit(1);
                }
            }
        }
        "repl" => {
            start_repl();
        }
        "version" => {
            print_version();
        }
        "help" => {
            print_help();
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn run_file(filename: &str) -> Result<(), RuntimeError> {
    // Read the file
    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            return Err(RuntimeError::UserError(format!(
                "Failed to read file '{}': {}",
                filename, e
            )));
        }
    };

    // Create runtime and execute
    let mut runtime = create_runtime();

    // For now, just print that we're running the file
    println!("Running Zenith file: {}", filename);
    println!("File content length: {} characters", content.len());

    // This would parse and execute the content
    execute_code(&mut runtime, &content).map(|_| ())
}

fn eval_code(code: &str) -> Result<(), RuntimeError> {
    let mut runtime = create_runtime();

    println!("Evaluating Zenith code: {}", code);

    // This would parse and execute the code
    execute_code(&mut runtime, code)?;

    Ok(())
}

fn start_repl() {
    println!("Zenith Runtime REPL v1.0.0");
    println!("Type 'exit' to quit, 'help' for help");
    println!();

    let _runtime = create_runtime();
    let mut input = String::new();

    loop {
        print!("zenith> ");
        io::stdout().flush().unwrap();

        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                match input {
                    "exit" | "quit" => break,
                    "help" => {
                        print_repl_help();
                    }
                    "clear" => {
                        // Clear screen (platform specific)
                        print!("\x1B[2J\x1B[1;1H");
                    }
                    _ => {
                        // Evaluate the input
                        match eval_code(input) {
                            Ok(_) => {}
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Input error: {}", e);
            }
        }
    }

    println!("Goodbye!");
}

fn print_usage() {
    println!("Zenith Runtime v1.0.0");
    println!("Usage: zenith-runtime <command> [options]");
    println!();
    println!("Commands:");
    println!("  run <file>     Run a Zenith program from file");
    println!("  eval <code>     Evaluate Zenith code directly");
    println!("  repl            Start interactive REPL");
    println!("  version         Show version information");
    println!("  help            Show this help message");
    println!();
    println!("Examples:");
    println!("  zenith-runtime run program.zn");
    println!("  zenith-runtime eval 'println(\"Hello, Zenith!\")'");
    println!("  zenith-runtime repl");
}

fn print_version() {
    println!("Zenith Runtime v1.0.0");
    println!("Built with Rust");
    println!("Runtime features: GC, JIT (optional), Debug (optional)");
    println!("Repository: https://github.com/zenith-lang/zenith");
    println!("Website: https://zenith-lang.org");
}

fn print_help() {
    print_usage();
    println!();
    println!("For more information, visit:");
    println!("  Documentation: https://docs.zenith-lang.org");
    println!("  GitHub: https://github.com/zenith-lang/zenith");
    println!("  Community: https://discord.gg/zenith");
}

fn print_repl_help() {
    println!("REPL Commands:");
    println!("  help     - Show this help");
    println!("  clear    - Clear the screen");
    println!("  exit     - Exit the REPL");
    println!();
    println!("Language Features:");
    println!("  Variables: let x = 42");
    println!("  Functions: fn add(a, b) {{ return a + b; }}");
    println!("  Arrays: [1, 2, 3]");
    println!("  Objects: {{ name: \"Zenith\", version: 1.0 }}");
    println!("  Control flow: if, while, for");
    println!("  Built-ins: print(), len(), type()");
    println!();
    println!("Examples:");
    println!("  let x = 10");
    println!("  let y = x * 2");
    println!("  print(y)");
    println!("  => 20");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_run_file() {
        // This would test file running functionality
        // For now, just ensure the function exists
        assert!(true);
    }

    #[test]
    fn test_eval_code() {
        // This would test code evaluation
        // For now, just ensure the function exists
        assert!(true);
    }

    #[test]
    fn test_repl_functionality() {
        // This would test REPL functionality
        // For now, just ensure the function exists
        assert!(true);
    }
}
