fn main() {
    cxx_build::bridge("src/main.rs")
        .file("core_cpp/evaluator.cpp")
        .file("core_cpp/hacker.cpp")
        .flag_if_supported("-std=c++17")
        .compile("ayuda");

    println!("cargo:rustc-link-lib=ncurses");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=core_cpp/evaluator.cpp");
    println!("cargo:rerun-if-changed=core_cpp/hacker.cpp");
    println!("cargo:rerun-if-changed=core_cpp/include/evaluator.hpp");
    println!("cargo:rerun-if-changed=core_cpp/include/hacker.hpp");
}
