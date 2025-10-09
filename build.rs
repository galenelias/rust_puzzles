// https://doc.rust-lang.org/cargo/reference/build-script-examples.html

fn main() {
    cc::Build::new()
        .file("puzzle_drop/mines.c")
        .compile("mines");
    cc::Build::new().file("src/hello.c").compile("hello");
    println!("cargo::rerun-if-changed=src/hello.c");
}
