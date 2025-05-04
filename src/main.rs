mod core;

use core::Shell;

fn main() {
    match Shell::new() {
        Ok(mut shell) => {
            if let Err(e) = shell.run() {
                eprintln!("Shell error: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to start shell: {}", e),
    }
}
