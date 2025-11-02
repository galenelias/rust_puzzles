// https://doc.rust-lang.org/cargo/reference/build-script-examples.html

fn main() {
    cc::Build::new()
        .file("puzzles/mines.c")
        .include("puzzles_inc")
        // .define("COMBINED", None)
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter")
        .compile("mines");
    cc::Build::new().file("src/hello.c").compile("hello");
    println!("cargo::rerun-if-changed=src/hello.c");
}
