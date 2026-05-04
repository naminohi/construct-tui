fn main() {
    // Proto types are now provided by construct-engine.
    // No need to compile them separately — just re-export from the engine crate.
    println!("cargo:rerun-if-changed=build.rs");
}
