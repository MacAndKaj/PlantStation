use prost_build::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    // Print cargo instructions to rebuild if proto files change
    println!("cargo:rerun-if-changed=proto/ps.proto");
    println!("cargo:rerun-if-changed=build.rs");

    // Get output directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Configure protobuf compilation
    let mut config = Config::new();

    // Set output directory for generated code
    config.out_dir(&out_dir);

    // Compile protobuf files
    match config.compile_protos(&["proto/ps.proto"], &["proto"]) {
        Ok(_) => {
            println!("cargo:warning=Protocol Buffers compiled successfully to {:?}", out_dir);
        }
        Err(e) => {
            eprintln!("Failed to compile protocol buffers: {}", e);
            eprintln!("Make sure protoc is installed: apt-get install protobuf-compiler");
            panic!("Protobuf compilation failed: {}", e);
        }
    }
}

