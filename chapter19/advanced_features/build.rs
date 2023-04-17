// build.rs
fn main() {
    println!("cargo:rustc-link-search=native=/Users/artemmikheev/Documents/Projects_(personal)/IT/Rust/RustLearning/chapter19/c_library");
    println!("cargo:rustc-link-lib=dylib=add");
} // end main()