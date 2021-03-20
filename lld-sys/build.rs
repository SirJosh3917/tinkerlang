fn main() {
    #[cfg(not(debug_assertions))]
    let link_type = "static";
    
    #[cfg(debug_assertions)]
    let link_type = "dylib";

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
