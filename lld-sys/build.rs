use std::{
    io::Read,
    process::{Command, Stdio},
};

fn main() {
    #[cfg(not(debug_assertions))]
    let link_type = "static";

    #[cfg(debug_assertions)]
    let link_type = "dylib";

    let mut llvm_libdir = Command::new("llvm-config")
        .arg("--libdir")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .unwrap();
    let mut llvm_libdir_buffer = String::new();
    llvm_libdir.read_to_string(&mut llvm_libdir_buffer).unwrap();
    println!("cargo:rustc-link-search={}", llvm_libdir_buffer);

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
