
#[cfg(feature = "bindings")]
fn generate_bindings() {
    use std::process::Command;

    // Emit dynamic link flags for all required Graphviz libraries
    for lib_name in &["gvc", "cgraph", "cdt", "xdot"] {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }

    // Optionally add required system libs
    println!("cargo:rustc-link-lib=dylib=z");
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-lib=dylib=expat");

    // Generate bindgen flags from pkg-config --cflags
    let cflags_output = Command::new("pkg-config")
        .args(&["--cflags", "libgvc"])
        .output()
        .expect("Failed to run pkg-config --cflags for libgvc");

    let cflags = String::from_utf8_lossy(&cflags_output.stdout);
    let mut bindings = bindgen::Builder::default().header("wrapper.h");

    for flag in cflags.split_whitespace() {
        bindings = bindings.clang_arg(flag);
    }

    // Ignore conflicting C macros from system headers
    bindings = bindings
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_SUBNORMAL");

    let bindings = bindings.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    #[cfg(feature = "bindings")]
    generate_bindings();
}
