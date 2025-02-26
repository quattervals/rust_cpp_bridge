fn main() -> miette::Result<()> {

    // This assumes all your C++ bindings are in main.rs
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&"src_cpp"])
        .build()
        .unwrap();

    b.flag_if_supported("-std=c++20")
        .file("src_cpp/person.cpp")
        .compile("autocxx-rust-cpp-integration");

     println!("cargo:rerun-if-changed=src_cpp/person.hpp");
     println!("cargo:rerun-if-changed=src_cpp/person.cpp");

    Ok(())
}
