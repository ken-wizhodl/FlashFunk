#![allow(dead_code, unused_imports, unused_variables)]

use std::env;
use std::path::PathBuf;
use std::io::Write;

/// 遍历命令
fn build(target: &str) {
    println!("Ready to compile interface for ctp");
    check_arch();
    add_search_path();
    add_llvm_path();

    // Tell cargo to invalidate the built crate whenever the wrapper changes

    cc::Build::new()
        .file("src/ctp/src/ctp.cpp")
        .cpp(true)
        .warnings(true)
        .flag("-std=c++11")
        .compile("bridge");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/ctp/wrapper.hpp")
        /* // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks)) */
        /* .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        }) */
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("./src/ctp").join("bindings.rs");
    println!("outpath : {:?}", out_path);
    let binding_output = bindings.to_string().replace("c_char", "c_uchar");
    let mut output_file = std::fs::File::create(out_path.as_path()).map_err(|e| format!("cannot create struct file, {}", e)).unwrap();
    output_file.write_all(binding_output.as_bytes()).map_err(|e| format!("cannot write struct file, {}", e));
    println!("Generate successful")
}

fn main() {
    let current_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = format!("{}/sdk_sources/ctp/lib", current_dir);
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-flags=-l thostmduserapi_se -L {}/thostmduserapi_se.dll", out_dir);
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=src/bridge/bridge.hpp");
    println!("cargo:rerun-if-changed=src/bridge/bridge.cpp");
    // build("ctp");
}


#[cfg(not(target_os = "windows"))]
fn add_search_path() {
    for path in std::env::var("LD_LIBRARY_PATH").unwrap_or_else(|_| "".to_string()).split(":") {
        if path.trim().len() == 0 {
            continue;
        }
        println!("cargo:rustc-link-search={}", path);
    }
}

#[cfg(target_os = "windows")]
fn add_search_path() {
    for path in std::env::var("PATH").unwrap_or_else(|_| "".to_string()).split(";") {
        if path.trim().len() == 0 {
            continue;
        }
        println!("cargo:rustc-link-search={}", path);
    }
}

#[cfg(target_arch = "x86_64")]
fn check_arch() {}

#[cfg(target_arch = "x86")]
fn check_arch() {
    unimplemented!("Not implemented for 32 bit system!");
}

fn add_llvm_path() {
    if let Some(llvm_config_path) = option_env!("LLVM_CONFIG_PATH") {
        println!("cargo:rustc-env=LLVM_CONFIG_PATH={}", llvm_config_path);
    }
}