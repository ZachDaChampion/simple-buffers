fn main() {
    // Use `lazy_static` in older versions of rustc that don't support `LazyCell`.
    println!("cargo:rustc-check-cfg=cfg(use_lazy_static)");
    if let Ok(version) = rustc_version::version() {
        if version < rustc_version::Version::parse("1.80.0").unwrap() {
            println!("cargo:rustc-cfg=use_lazy_static");
        }
    } else {
        println!("cargo:warning=Failed to check rustc version; cannot verify compatibility.");
    }
}
