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
        .file("puzzles/blackbox.c")
        .file("puzzles/bridges.c")
        .file("puzzles/cube.c")
        .file("puzzles/dominosa.c")
        .file("puzzles/fifteen.c")
        .file("puzzles/filling.c")
        .file("puzzles/flip.c")
        .file("puzzles/flood.c")
        .file("puzzles/galaxies.c")
        .file("puzzles/guess.c")
        .file("puzzles/inertia.c")
        .file("puzzles/keen.c")
        .file("puzzles/lightup.c")
        .file("puzzles/loopy.c") // Need draw_thick_line
        .file("puzzles/magnets.c")
        .file("puzzles/map.c")
        .file("puzzles/mines.c")
        .file("puzzles/mosaic.c")
        .file("puzzles/net.c")
        .file("puzzles/netslide.c")
        .file("puzzles/palisade.c")
        .file("puzzles/pattern.c")
        .file("puzzles/pearl.c")
        .file("puzzles/pegs.c")
        .file("puzzles/range.c")
        .file("puzzles/rect.c")
        .file("puzzles/samegame.c")
        .file("puzzles/signpost.c")
        .file("puzzles/singles.c")
        .file("puzzles/sixteen.c")
        .file("puzzles/slant.c")
        .file("puzzles/solo.c")
        .file("puzzles/tents.c")
        .file("puzzles/towers.c")
        .file("puzzles/tracks.c")
        .file("puzzles/twiddle.c")
        .file("puzzles/undead.c")
        .file("puzzles/unequal.c")
        .file("puzzles/unruly.c")
        .file("puzzles/untangle.c")
        // // common_libs
        .file("puzzles/combi.c")
        .file("puzzles/divvy.c")
        .file("puzzles/drawing.c")
        .file("puzzles/dsf.c")
        .file("puzzles/findloop.c")
        .file("puzzles/grid.c")
        .file("puzzles/hat.c")
        .file("puzzles/latin.c")
        .file("puzzles/laydomino.c")
        .file("puzzles/loopgen.c")
        .file("puzzles/malloc.c")
        .file("puzzles/matching.c")
        .file("puzzles/midend.c")
        .file("puzzles/midend.c")
        .file("puzzles/misc.c")
        .file("puzzles/penrose.c")
        .file("puzzles/penrose-legacy.c")
        .file("puzzles/random.c")
        .file("puzzles/sort.c")
        .file("puzzles/spectre.c")
        .file("puzzles/tdq.c")
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
