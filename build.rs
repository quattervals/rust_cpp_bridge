fn main() {
    cxx_build::bridge("src/main.rs")
        .file("cpp_src/person.cpp")
        .std("c++17")
        .compile("rust-with-cpp-cxx");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cpp_src/person.cpp");
    println!("cargo:rerun-if-changed=include/person.hpp");
}
