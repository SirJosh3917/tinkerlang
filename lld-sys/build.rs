use std::{
    env,
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
};

extern crate bindgen;

fn main() {
    let llvm_project_path = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("./scripts/llvm-project-llvmorg-11.0.1");

    let lld_path = llvm_project_path.join("lld");

    let lld_install_path = lld_path
        .join("build/lib")
        .into_os_string()
        .into_string()
        .unwrap();

    #[cfg(not(debug_assertions))]
    println!("cargo:rustc-link-search=native={}", lld_install_path);

    #[cfg(not(debug_assertions))]
    let link_type = "static";
    #[cfg(debug_assertions)]
    let link_type = "dylib";

    let llvm_path = llvm_project_path.join("llvm");

    let llvm_libs = llvm_path
        .join("build/lib")
        .into_os_string()
        .into_string()
        .unwrap();

    #[cfg(not(debug_assertions))]
    println!("cargo:rustc-link-search=native={}", llvm_libs);

    println!("cargo:rustc-link-lib={}=lldELF", link_type);
    #[cfg(not(debug_assertions))]
    println!("cargo:rustc-link-lib={}=LLVMSupport", link_type);
    println!("cargo:rustc-link-lib={}=lldCommon", link_type);
    println!("cargo:rustc-link-lib={}=lldDriver", link_type);
    #[cfg(debug_assertions)]
    println!("cargo:rustc-link-lib=dylib=tinfo");
    #[cfg(debug_assertions)]
    println!("cargo:rustc-link-lib=dylib=z");
}
