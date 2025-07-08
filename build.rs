#[cfg(all(feature = "bindings", not(target_arch = "wasm32")))]
fn generate_bindings() {
    use std::process::Command;

    // Run pkg-config to get include flags for libgvc
    let cflags_output = Command::new("pkg-config")
        .args(&["--cflags", "libgvc"])
        .output()
        .expect("Failed to run pkg-config --cflags for libgvc");

    let cflags = String::from_utf8_lossy(&cflags_output.stdout);

    // === Compile the wrapper.c ===
    let mut build = cc::Build::new();
    build.file("native/wrapper.c");

    for flag in cflags.split_whitespace() {
        if flag.starts_with("-I") {
            // If it's -I/some/path, pass it as include
            build.include(&flag[2..]);
        } else {
            // Pass other flags directly (rare for cflags)
            build.flag(flag);
        }
    }
    build.flag("-O3");
    build.compile("wrapper");

    // === Link required Graphviz + system libraries ===
    for lib_name in &["gvc", "cgraph", "cdt", "xdot"] {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }
    println!("cargo:rustc-link-lib=dylib=z");
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-lib=dylib=expat");

    // === Generate Rust bindings ===
    let mut bindings = bindgen::Builder::default()
        .header("native/wrapper.h");

    for flag in cflags.split_whitespace() {
        bindings = bindings.clang_arg(flag);
    }

    // Optionally blocklist noisy system macros
    bindings = bindings
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_SUBNORMAL");

    let bindings = bindings
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    let target = std::env::var("TARGET").expect("TARGET not set by cargo").trim().to_string();
    let bindings_enabled = std::env::var("CARGO_FEATURE_BINDINGS").is_ok();

    if bindings_enabled && !target.contains("wasm32") {
        generate_bindings();
    } else if bindings_enabled && target.starts_with("wasm32") {
        println!("target: {} bindings enabled: {}", target, bindings_enabled);
        //panic!("The 'bindings' feature is not supported on wasm32 targets");
    }
}