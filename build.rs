extern crate cmake;

fn main() {
    let dst = cmake::Config::new("person-lib").build();

    let mut b = autocxx_build::Builder::new("src/main.rs", &[&"person-lib/include"])
        .build()
        .unwrap();

    b.std("c++20")
        .include("person-lib/include")
        .compile("autocxx-rust-cpp-integration");

    // !! Link after the autocxx bindings have been compiled !!
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=person");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=person-lib/include/person.hpp");
    println!("cargo:rerun-if-changed=person-lib/src/person.cpp");
}
