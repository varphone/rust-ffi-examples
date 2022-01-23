use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let bar_include = env::var("DEP_BAR_INCLUDE").expect("Pakcage [bar-sys] not found!");
    let baz_include = env::var("DEP_BAZ_INCLUDE").expect("Pakcage [baz-sys] not found!");

    cc::Build::new()
        .files(["src/foo.c"])
        .include("include")
        .include(&bar_include)
        .include(&baz_include)
        .compile("foo");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/foo/foo.h")
        .clang_arg(format!("-I{}", &bar_include))
        .clang_arg(format!("-I{}", &baz_include))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Export the DEP_FOO_DIR and DEP_FOO_INCLUDE env.
    println!("cargo:dir={}", env!("CARGO_MANIFEST_DIR"));
    println!("cargo:include={}/include", env!("CARGO_MANIFEST_DIR"));
}
