use std::os::raw::{c_char, c_double, c_float, c_int, c_void};

// use libc::{c_int, c_void};
unsafe extern "C" {
    static thegame: GameFFI;
}

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
pub struct midend {
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
            dr: *mut DrawingFFI,
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
pub struct DrawingFFI {
    drawing_api: *const DrawingApiFFI,
    handle: *mut Drawing,
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
        game: *const GameFFI,
        drapi: *const DrawingApiFFI,
        drhandle: *mut Drawing,
    ) -> *mut MidendFFI;

    // void midend_redraw(midend *me);
    fn midend_redraw(me: *mut MidendFFI);
    // void fatal(const char *fmt, ...);

    // void midend_size(midend *me, int *x, int *y, bool user_size, double device_pixel_ratio);
    fn midend_size(
        me: *mut MidendFFI,
        x: *mut c_int,
        y: *mut c_int,
        user_size: bool,
        device_pixel_ratio: c_double,
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn frontend_default_colour(fe: *mut Frontend, output: *mut c_float) {
    unsafe {
        let out_slice = std::slice::from_raw_parts_mut(output, 3);
        out_slice[0] = 0.1;
        out_slice[1] = 0.1;
        out_slice[2] = 0.1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fatal(_fmt: *const c_char, ...) {
    println!("Fatal error!");
}

// void get_random_seed(void **randseed, int *randseedsize)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_random_seed(randseed: *mut *mut c_void, randseedsize: *mut c_int) {
    unsafe {
        let seed_value: i32 = 42;
        let seed_ptr = Box::into_raw(Box::new(seed_value)) as *mut c_void;
        *randseed = seed_ptr;
        *randseedsize = std::mem::size_of::<i32>() as c_int;
    }
}

// void deactivate_timer(frontend *fe);
// void activate_timer(frontend *fe);
pub unsafe extern "C" fn deactivate_timer(fe: *mut Frontend) {
    println!("Deactivate timer called");
    // Implementation for deactivating the timer
}

pub unsafe extern "C" fn activate_timer(fe: *mut Frontend) {
    println!("Activate timer called");
    // Implementation for activating the timer
}

// struct RustPuzzleInteroperability {
//     drawing_api: Drawing,
// }

#[repr(C)] // Needed?
struct Drawing {}

impl Drawing {
    fn new() -> Self {
        Drawing {}
    }

    fn draw_rectangle(self: &mut Drawing, x: c_int, y: c_int, w: c_int, h: c_int, colour: c_int) {
        println!(
            "Drawing rectangle at ({}, {}) with width {} and height {} and colour {}",
            x, y, w, h, colour
        );
    }
}

unsafe extern "C" fn draw_rectangle_wrap(
    target: *mut DrawingFFI,
    x: c_int,
    y: c_int,
    w: c_int,
    h: c_int,
    colour: c_int,
) {
    unsafe {
        (*(*target).handle).draw_rectangle(x, y, w, h, colour);
    }
}

// impl RustPuzzleInteroperability {
//     fn new() -> Self {
//         Self {
//             drawing_api: DrawingApi::new(),
//         }
//     }
// }

#[repr(C)]
pub struct Frontend {
    midend: *mut MidendFFI,
    drawing_ffi: DrawingApiFFI,
    drawing: Drawing,
    // drawing_api: DrawingApi,
}

impl Frontend {
    pub fn new() -> Self {
        Self {
            midend: std::ptr::null_mut(),
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
            drawing: Drawing::new(),
            // drawing_api: DrawingApi::new(),
        }
    }

    fn new_game(&mut self, game: *const GameFFI) {
        unsafe {
            let midend = midend_new(self, game, &self.drawing_ffi, &mut self.drawing);
        }
    }

    pub fn new_mines(&mut self) {
        unsafe {
            let midend = midend_new(self, &thegame, &self.drawing_ffi, &mut self.drawing);

            // print 'midend' as a pointer value
            println!("midend: {:p}", midend);

            self.midend = midend;
        }
    }

    pub fn set_size(&mut self) {
        let mut x_val: c_int = 500;
        let mut y_val: c_int = 500;
        unsafe {
            midend_size(
                self.midend,
                &mut x_val,
                &mut y_val,
                false, /*user_size*/
                1.0,   /*device_pixel_ratio*/
            );
        }

        println!("Post set_size: x = {}, y = {}", x_val, y_val);
    }

    pub fn redraw(&mut self) {
        unsafe {
            midend_redraw(self.midend);
        }
    }
}
