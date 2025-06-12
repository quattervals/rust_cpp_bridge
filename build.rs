extern crate cmake;

fn main() {
    let dst = cmake::Config::new("person-lib").build();

    cxx_build::bridge("src/main.rs")
        .include("person-lib/include")
        .include(format!("{}/include", dst.display()))
        .std("c++20")
        .compile("cxx-rust-cpp-integration");

    // !! Link after the cxx bindings have been compiled !!
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=person");

    println!("cargo::rerun-if-changed=src/main.rs");
    println!("cargo::rerun-if-changed=person-lib");
}
