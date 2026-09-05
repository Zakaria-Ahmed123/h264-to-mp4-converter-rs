extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/decode.c");
    
    cc::Build::new()
        .file("src/decode.c")
        .compile("decode");
}