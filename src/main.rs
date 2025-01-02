#![allow(dead_code)]
#![allow(warnings)]

mod lc3;
use lc3::vm::vm;
use lc3::sys::file;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: lc3 [image-file1] [image-file2] ...");
        std::process::exit(2);
    }
    //init the vm
    let mut vm = vm::LC3::new();
        // Load the provided image files into the VM's memory
        for image_file in &args[1..] {
            match vm.load_image(image_file) {
                Ok(_) => println!("Loaded: {}", image_file),
                Err(e) => eprintln!("Failed to load {}: {}", image_file, e),
            }
        }

    vm.run();
}
