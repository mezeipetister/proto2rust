extern crate protoc_rust;

use protoc_rust::Customize;
use std::env;
use std::path::Path;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input : &str = &args[1][..];
    let outdir : &str = &args[2][..];

    // Check if path does exist
    if !Path::new(outdir).exists() {
        // Create path
        fs::create_dir(outdir).unwrap();
    }
    
    protoc_rust::run(protoc_rust::Args {
        out_dir: outdir,
        input: &[input],
        includes: &[],
        customize: Customize {
        ..Default::default()
        },
    }).expect("protoc");
}
