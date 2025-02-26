fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/person.cpp")
        .std("c++20")
        .compile("cxx-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/person.cpp");
    println!("cargo:rerun-if-changed=include/person.hpp");
}
