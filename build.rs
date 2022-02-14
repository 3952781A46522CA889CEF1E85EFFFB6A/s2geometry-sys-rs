fn main() {
    // cc::Build::new().cpp(true)
    //     .include("/root/app/include/")
    //     .static_flag(true)
    //     .file("src/s2wrapper.cpp").compile("s2wrapper");
    let bindings = bindgen::Builder::default()
        .header("src/s2wrapper.hpp")
        .allowlist_type("S2LatLng*")
        .allowlist_function("S2LatLng*")
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");
    bindings.write_to_file("src/bindings.rs").expect("Unable to write bindings.rs");
    println!("cargo:rustc-link-search=native=/root/app/lib/");
    println!("cargo:rustc-link-lib=dylib=s2");
}