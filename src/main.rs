#[allow(dead_code)]
mod lc3;

use lc3::vm;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: lc3 [image-file1] [image-file2] ...");
        std::process::exit(2);
    }

    // Instantiate the LC-3 VM
    let mut vm = vm::LC3::new();
    vm.main_loop();
}
