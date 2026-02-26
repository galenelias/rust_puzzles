// https://doc.rust-lang.org/cargo/reference/build-script-examples.html

// Common sources from puzzles project:
// combi.c divvy.c draw-poly.c drawing.c dsf.c findloop.c grid.c
// latin.c laydomino.c loopgen.c malloc.c matching.c midend.c misc.c
// penrose.c penrose-legacy.c ps.c random.c sort.c tdq.c tree234.c
// version.c

fn main() {
    let mut build = cc::Build::new();

    build
        // Games
        .file("puzzles/bridges.c")
        .file("puzzles/cube.c")
        .file("puzzles/flood.c")
        .file("puzzles/map.c")
        .file("puzzles/mines.c")
        .file("puzzles/samegame.c")
        .file("puzzles/tents.c")
        // // common_libs
        .file("puzzles/divvy.c")
        .file("puzzles/drawing.c")
        .file("puzzles/dsf.c")
        .file("puzzles/findloop.c")
        .file("puzzles/latin.c")
        .file("puzzles/laydomino.c")
        .file("puzzles/malloc.c")
        .file("puzzles/matching.c")
        .file("puzzles/midend.c")
        .file("puzzles/midend.c")
        .file("puzzles/misc.c")
        .file("puzzles/random.c")
        .file("puzzles/sort.c")
        .file("puzzles/tree234.c")
        .define("COMBINED", None)
        .file("puzzles/list.c")
        .include("puzzles_inc");

    if !cfg!(target_env = "msvc") {
        build.flag("-Wno-sign-compare");
        build.flag("-Wno-unused-parameter");
    }

    build.compile("mines");

    // .define("COMBINED", None)
    // println!("cargo::rerun-if-changed=src/hello.c");
}
