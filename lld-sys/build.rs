extern crate regex;
extern crate semver;

use std::{
    ffi::OsStr,
    io::Read,
    io::{self, ErrorKind},
    path::PathBuf,
    process::{Command, Stdio},
};

use regex::Regex;
use semver::Version;

fn main() {
    #[cfg(not(debug_assertions))]
    let link_type = "static";

    #[cfg(debug_assertions)]
    let link_type = "dylib";

    let mut llvm_libdir = Command::new(locate_llvm_config().unwrap())
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

// https://gitlab.com/taricorp/llvm-sys.rs/-/blob/master/build.rs
fn locate_llvm_config() -> Option<PathBuf> {
    let prefix = PathBuf::new();
    for binary_name in llvm_config_binary_names() {
        let binary_name = prefix.join(binary_name);
        match llvm_version(&binary_name) {
            Ok(ref version) => return Some(binary_name),
            Err(ref e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => panic!("Failed to search PATH for llvm-config: {}", e),
        }
    }

    None
}

fn llvm_config_binary_names() -> std::vec::IntoIter<String> {
    let mut base_names: Vec<String> = vec![
        "llvm-config".into(),
        "llvm-config-11".into(),
        "llvm-config-11.0".into(),
        "llvm-config110".into(),
    ];

    // On Windows, also search for llvm-config.exe
    if cfg!(target_os = "windows") {
        let mut exe_names = base_names.clone();
        for name in exe_names.iter_mut() {
            name.push_str(".exe");
        }
        base_names.extend(exe_names);
    }

    base_names.into_iter()
}

/// Get the LLVM version using llvm-config.
fn llvm_version<S: AsRef<OsStr>>(binary: &S) -> io::Result<Version> {
    let version_str = llvm_config_ex(binary.as_ref(), "--version")?;

    // LLVM isn't really semver and uses version suffixes to build
    // version strings like '3.8.0svn', so limit what we try to parse
    // to only the numeric bits.
    let re = Regex::new(r"^(?P<major>\d+)\.(?P<minor>\d+)(?:\.(?P<patch>\d+))??").unwrap();
    let c = re
        .captures(&version_str)
        .expect("Could not determine LLVM version from llvm-config.");

    // some systems don't have a patch number but Version wants it so we just append .0 if it isn't
    // there
    let s = match c.name("patch") {
        None => format!("{}.0", &c[0]),
        Some(_) => c[0].to_string(),
    };
    Ok(Version::parse(&s).unwrap())
}

/// Invoke the specified binary as llvm-config.
///
/// Explicit version of the `llvm_config` function that bubbles errors
/// up.
fn llvm_config_ex<S: AsRef<OsStr>>(binary: S, arg: &str) -> io::Result<String> {
    Command::new(binary).arg(arg).output().map(|output| {
        String::from_utf8(output.stdout).expect("Output from llvm-config was not valid UTF-8")
    })
}
