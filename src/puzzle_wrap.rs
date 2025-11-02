#[repr(C)]
pub struct Frontend {}

use std::os::raw::{c_char, c_float, c_int, c_void};

// use libc::{c_int, c_void};

// Forward declarations for opaque types
#[repr(C)]
pub struct drawing {
    _private: [u8; 0],
}

#[repr(C)]
pub struct blitter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DrawingApiFFI {
    pub version: c_int,

    pub draw_text: Option<
        unsafe extern "C" fn(
            dr: *mut drawing,
            x: c_int,
            y: c_int,
            fonttype: c_int,
            fontsize: c_int,
            align: c_int,
            colour: c_int,
            text: *const c_char,
        ),
    >,
    pub draw_rect: Option<
        unsafe extern "C" fn(
            dr: *mut DrawingApi,
            x: c_int,
            y: c_int,
            w: c_int,
            h: c_int,
            colour: c_int,
        ),
    >,
    pub draw_line: Option<
        unsafe extern "C" fn(
            dr: *mut drawing,
            x1: c_int,
            y1: c_int,
            x2: c_int,
            y2: c_int,
            colour: c_int,
        ),
    >,
    pub draw_polygon: Option<
        unsafe extern "C" fn(
            dr: *mut drawing,
            coords: *const c_int,
            npoints: c_int,
            fillcolour: c_int,
            outlinecolour: c_int,
        ),
    >,
    pub draw_circle: Option<
        unsafe extern "C" fn(
            dr: *mut drawing,
            cx: c_int,
            cy: c_int,
            radius: c_int,
            fillcolour: c_int,
            outlinecolour: c_int,
        ),
    >,
    pub draw_update:
        Option<unsafe extern "C" fn(dr: *mut drawing, x: c_int, y: c_int, w: c_int, h: c_int)>,
    pub clip:
        Option<unsafe extern "C" fn(dr: *mut drawing, x: c_int, y: c_int, w: c_int, h: c_int)>,
    pub unclip: Option<unsafe extern "C" fn(dr: *mut drawing)>,
    pub start_draw: Option<unsafe extern "C" fn(dr: *mut drawing)>,
    pub end_draw: Option<unsafe extern "C" fn(dr: *mut drawing)>,
    pub status_bar: Option<unsafe extern "C" fn(dr: *mut drawing, text: *const c_char)>,
    pub blitter_new:
        Option<unsafe extern "C" fn(dr: *mut drawing, w: c_int, h: c_int) -> *mut blitter>,
    pub blitter_free: Option<unsafe extern "C" fn(dr: *mut drawing, bl: *mut blitter)>,
    pub blitter_save:
        Option<unsafe extern "C" fn(dr: *mut drawing, bl: *mut blitter, x: c_int, y: c_int)>,
    pub blitter_load:
        Option<unsafe extern "C" fn(dr: *mut drawing, bl: *mut blitter, x: c_int, y: c_int)>,
    pub begin_doc: Option<unsafe extern "C" fn(dr: *mut drawing, pages: c_int)>,
    pub begin_page: Option<unsafe extern "C" fn(dr: *mut drawing, number: c_int)>,
    pub begin_puzzle: Option<
        unsafe extern "C" fn(
            dr: *mut drawing,
            xm: c_float,
            xc: c_float,
            ym: c_float,
            yc: c_float,
            pw: c_int,
            ph: c_int,
            wmm: c_float,
        ),
    >,
    pub end_puzzle: Option<unsafe extern "C" fn(dr: *mut drawing)>,
    pub end_page: Option<unsafe extern "C" fn(dr: *mut drawing, number: c_int)>,
    pub end_doc: Option<unsafe extern "C" fn(dr: *mut drawing)>,
    pub line_width: Option<unsafe extern "C" fn(dr: *mut drawing, width: c_float)>,
    pub line_dotted: Option<unsafe extern "C" fn(dr: *mut drawing, dotted: bool)>,
    pub text_fallback: Option<
        unsafe extern "C" fn(
            dr: *mut drawing,
            strings: *const *const c_char,
            nstrings: c_int,
        ) -> *mut c_char,
    >,
    pub draw_thick_line: Option<
        unsafe extern "C" fn(
            dr: *mut drawing,
            thickness: c_float,
            x1: c_float,
            y1: c_float,
            x2: c_float,
            y2: c_float,
            colour: c_int,
        ),
    >,
}

#[repr(C)]
pub struct GameFFI {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct MidendFFI {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

// #[repr(C)]
// pub struct DrawingFFI {
//     _data: (),
//     _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
// }

unsafe extern "C" {
    // midend *midend_new(frontend *fe, const game *ourgame, const DrawingApi *drapi, void *drhandle);
    // struct frontend;
    // struct mident;
    // struct DrawingApi;
    // struct game;

    fn midend_new(
        fe: *mut Frontend,
        ourgame: *const GameFFI,
        drapi: *const DrawingApi,
        drhandle: *mut c_void,
    ) -> *mut MidendFFI;

}

struct RustPuzzleInteroperability {
    drawing_api: DrawingApi,
}

struct DrawingApi {
    drawing_ffi: DrawingApiFFI,
}

impl DrawingApi {
    fn new() -> Self {
        DrawingApi {
            drawing_ffi: DrawingApiFFI {
                version: 1,
                draw_text: None,
                draw_rect: Some(draw_rectangle_wrap),
                draw_line: None,
                draw_polygon: None,
                draw_circle: None,
                draw_update: None,
                clip: None,
                unclip: None,
                start_draw: None,
                end_draw: None,
                status_bar: None,
                blitter_new: None,
                blitter_free: None,
                blitter_save: None,
                blitter_load: None,
                begin_doc: None,
                begin_page: None,
                begin_puzzle: None,
                end_puzzle: None,
                end_page: None,
                end_doc: None,
                line_width: None,
                line_dotted: None,
                text_fallback: None,
                draw_thick_line: None,
            },
        }
    }

    fn draw_rectangle(
        self: &mut DrawingApi,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        colour: c_int,
    ) {
        println!(
            "Drawing rectangle at ({}, {}) with width {} and height {} and colour {}",
            x, y, w, h, colour
        );
    }
}

unsafe extern "C" fn draw_rectangle_wrap(
    target: *mut DrawingApi,
    x: c_int,
    y: c_int,
    w: c_int,
    h: c_int,
    colour: c_int,
) {
    (*target).draw_rectangle(x, y, w, h, colour);
}

impl RustPuzzleInteroperability {
    fn new() -> Self {
        Self {
            drawing_api: DrawingApi::new(),
        }
    }
}

// pub struct Midend {}

// impl Midend {
//     fn new(
//         fe: *mut frontend,
//         ourgame: *const game,
//         drapi: *const drawing_api,
//         drhandle: *mut c_void,
//     ) -> Self {
//         unsafe {
//             let midend = midend_new(fe, ourgame, drapi, drhandle);
//             Self {}
//         }
//     }
// }
