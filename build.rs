fn main() {
    let gvc_cflags = std::process::Command::new("pkg-config")
        .args(&["--cflags", "libgvc"])
        .output()
        .expect("Failed to run pkg-config")
        .stdout;

    let cflags = String::from_utf8(gvc_cflags).unwrap();
    let clang_args = cflags
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h");

    for arg in &clang_args {
        bindings = bindings.clang_arg(arg);
    }

    let bindings = bindings
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("FP_SUBNORMAL")
        .generate()
        .expect("Unable to generate bindings");

        println!("cargo:rustc-link-lib=static=gvc");
        println!("cargo:rustc-link-lib=static=cgraph");
        println!("cargo:rustc-link-lib=static=cdt");
        println!("cargo:rustc-link-lib=static=pathplan");
        println!("cargo:rustc-link-lib=static=xdot");
    
        // Add any required system libs
        println!("cargo:rustc-link-lib=dylib=z");         // zlib
        println!("cargo:rustc-link-lib=dylib=m");         // math
        println!("cargo:rustc-link-lib=dylib=stdc++");    // if C++ symbols needed
        println!("cargo:rustc-link-lib=dylib=c");         // libc (should be implicit)

        println!("cargo:rustc-link-lib=dylib=expat");
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}