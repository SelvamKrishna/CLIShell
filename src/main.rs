mod core;

use core::Shell;

fn main() {
    let mut shell = Shell::new().unwrap();
    shell.run().unwrap();
}
