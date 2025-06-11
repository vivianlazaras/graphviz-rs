use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to link statically
    pkg_config::Config::new()
        .statik(true)
        .cargo_metadata(true)
        .probe("libgvc")
        .expect("Could not find libgvc");

    // Re-run build if the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function("gv.*")
        .allowlist_function("ag.*")
        .allowlist_type("Agraph_t")
        .allowlist_type("GVC_t")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}