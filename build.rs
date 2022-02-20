#![allow(non_camel_case_types)]

fn main() {
    cc::Build::new().cpp(true)
        .flag("-D_GLIBCXX_USE_CXX11_ABI=1")
        .flag("-std=c++17")
        .static_flag(true)
        .file("src/s2wrapper.cpp").compile("s2wrapper");

    let bindings = bindgen::Builder::default()
        .header("src/s2wrapper.hpp")
        .allowlist_type("S2LatLng.*")
        .allowlist_type("S2Point.*")
        .allowlist_type("S2CellIdIndex.*")
        .allowlist_type("S2ClosestCellQuery.*")
        .allowlist_type("S2RegionTermIndexer.*")
        .allowlist_type("Options.*")
        .allowlist_type("ctx")
        .allowlist_function("S2Region.*")
        .blocklist_type("const_pointer")
        .opaque_type("absl.*")
        .opaque_type("std.*")
        .opaque_type("gtl.*")
        .layout_tests(false)
        .size_t_is_usize(true)
        .clang_arg("-std=c++17")
        .clang_arg("-D_GLIBCXX_USE_CXX11_ABI=1")
        .generate()
        .expect("Unable to generate bindings");
    bindings.write_to_file("src/bindings.rs").expect("Unable to write bindings.rs");
    println!("cargo:rustc-link-lib=dylib=s2");
}
