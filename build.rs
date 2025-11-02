// https://doc.rust-lang.org/cargo/reference/build-script-examples.html

// Common sources from puzzles project:
// combi.c divvy.c draw-poly.c drawing.c dsf.c findloop.c grid.c
// latin.c laydomino.c loopgen.c malloc.c matching.c midend.c misc.c
// penrose.c penrose-legacy.c ps.c random.c sort.c tdq.c tree234.c
// version.c

fn main() {
    cc::Build::new()
        .file("puzzles/mines.c")
        // common_libs
        .file("puzzles/drawing.c")
        .file("puzzles/malloc.c")
        .file("puzzles/midend.c")
        .file("puzzles/misc.c")
        .file("puzzles/random.c")
        .file("puzzles/tree234.c")
        .include("puzzles_inc")
        // .define("COMBINED", None)
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-parameter")
        .compile("mines");
    // println!("cargo::rerun-if-changed=src/hello.c");
}
