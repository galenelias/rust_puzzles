use raqote::*;
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
            dr: *mut DrawingFFI,
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
            dr: *mut DrawingFFI,
            x1: c_int,
            y1: c_int,
            x2: c_int,
            y2: c_int,
            colour: c_int,
        ),
    >,
    pub draw_polygon: Option<
        unsafe extern "C" fn(
            dr: *mut DrawingFFI,
            coords: *const c_int,
            npoints: c_int,
            fillcolour: c_int,
            outlinecolour: c_int,
        ),
    >,
    pub draw_circle: Option<
        unsafe extern "C" fn(
            dr: *mut DrawingFFI,
            cx: c_int,
            cy: c_int,
            radius: c_int,
            fillcolour: c_int,
            outlinecolour: c_int,
        ),
    >,
    pub draw_update:
        Option<unsafe extern "C" fn(dr: *mut DrawingFFI, x: c_int, y: c_int, w: c_int, h: c_int)>,
    pub clip:
        Option<unsafe extern "C" fn(dr: *mut DrawingFFI, x: c_int, y: c_int, w: c_int, h: c_int)>,
    pub unclip: Option<unsafe extern "C" fn(dr: *mut DrawingFFI)>,
    pub start_draw: Option<unsafe extern "C" fn(dr: *mut DrawingFFI)>,
    pub end_draw: Option<unsafe extern "C" fn(dr: *mut DrawingFFI)>,
    pub status_bar: Option<unsafe extern "C" fn(dr: *mut DrawingFFI, text: *const c_char)>,
    pub blitter_new:
        Option<unsafe extern "C" fn(dr: *mut DrawingFFI, w: c_int, h: c_int) -> *mut blitter>,
    pub blitter_free: Option<unsafe extern "C" fn(dr: *mut DrawingFFI, bl: *mut blitter)>,
    pub blitter_save:
        Option<unsafe extern "C" fn(dr: *mut DrawingFFI, bl: *mut blitter, x: c_int, y: c_int)>,
    pub blitter_load:
        Option<unsafe extern "C" fn(dr: *mut DrawingFFI, bl: *mut blitter, x: c_int, y: c_int)>,
    pub begin_doc: Option<unsafe extern "C" fn(dr: *mut DrawingFFI, pages: c_int)>,
    pub begin_page: Option<unsafe extern "C" fn(dr: *mut DrawingFFI, number: c_int)>,
    pub begin_puzzle: Option<
        unsafe extern "C" fn(
            dr: *mut DrawingFFI,
            xm: c_float,
            xc: c_float,
            ym: c_float,
            yc: c_float,
            pw: c_int,
            ph: c_int,
            wmm: c_float,
        ),
    >,
    pub end_puzzle: Option<unsafe extern "C" fn(dr: *mut DrawingFFI)>,
    pub end_page: Option<unsafe extern "C" fn(dr: *mut DrawingFFI, number: c_int)>,
    pub end_doc: Option<unsafe extern "C" fn(dr: *mut DrawingFFI)>,
    pub line_width: Option<unsafe extern "C" fn(dr: *mut DrawingFFI, width: c_float)>,
    pub line_dotted: Option<unsafe extern "C" fn(dr: *mut DrawingFFI, dotted: bool)>,
    pub text_fallback: Option<
        unsafe extern "C" fn(
            dr: *mut DrawingFFI,
            strings: *const *const c_char,
            nstrings: c_int,
        ) -> *mut c_char,
    >,
    pub draw_thick_line: Option<
        unsafe extern "C" fn(
            dr: *mut DrawingFFI,
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

    // void midend_new_game(midend *me);
    fn midend_new_game(me: *mut MidendFFI);

    // float *midend_colours(midend *me, int *ncolours);
    fn midend_colours(me: *mut MidendFFI, ncolours: *mut c_int) -> *mut c_float;

    // sfree
    fn sfree(ptr: *mut c_void);

}

#[unsafe(no_mangle)]
pub extern "C" fn frontend_default_colour(fe: *mut Frontend, output: *mut c_float) {
    unsafe {
        let out_slice = std::slice::from_raw_parts_mut(output, 3);
        out_slice[0] = 0.8;
        out_slice[1] = 0.8;
        out_slice[2] = 0.8;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn deactivate_timer(fe: *mut Frontend) {
    println!("Deactivate timer called");
    // Implementation for deactivating the timer
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn activate_timer(fe: *mut Frontend) {
    println!("Activate timer called");
    // Implementation for activating the timer
}

// struct RustPuzzleInteroperability {
//     drawing_api: Drawing,
// }

#[repr(C)] // Needed?
struct Drawing {
    dt: DrawTarget,
    colours: Vec<PuzColor>,
}

impl Drawing {
    fn new(width: u32, height: u32) -> Self {
        Drawing {
            dt: DrawTarget::new(width as i32, height as i32),
            colours: Vec::new(),
        }
    }

    /// Gain access to the underlying pixels
    pub fn frame(&self) -> &[u32] {
        self.dt.get_data()
    }

    fn start_draw(self: &mut Drawing) {
        // println!("start_draw called");

        self.dt.clear(SolidSource {
            g: self.colours[0].g,
            r: self.colours[0].r,
            b: self.colours[0].b,
            a: 0xff,
        });
    }

    fn end_draw(self: &mut Drawing) {
        println!("end_draw called");
    }

    fn draw_rect(self: &mut Drawing, x: c_int, y: c_int, w: c_int, h: c_int, colour: c_int) {
        println!(
            "Drawing rectangle at ({}, {}) with width {} and height {} and colour {}",
            x, y, w, h, colour
        );

        if w > 200 {
            return;
        }

        self.dt.fill_rect(
            x as f32,
            y as f32,
            w as f32,
            h as f32,
            &Source::Solid(SolidSource {
                g: self.colours[(colour) as usize].g,
                r: self.colours[(colour) as usize].r,
                b: self.colours[(colour) as usize].b,
                a: 0xff,
            }),
            &DrawOptions::new(),
        );
    }

    fn draw_polygon(
        self: &mut Drawing,
        points: *const c_int,
        num_points: c_int,
        fillcolour: c_int,
        outlinecolour: c_int,
    ) {
        let mut pb = PathBuilder::new();

        let get_point = |i: c_int| unsafe {
            let x = *points.offset(i as isize * 2);
            let y = *points.offset(i as isize * 2 + 1);
            (x as f32, y as f32)
        };

        let start = get_point(0);
        pb.move_to(start.0, start.1);

        // for i in 0..num_points {
        //     let pt = get_point(i);
        //     println!("Polygon pt {}: {},{}", i, pt.0, pt.1);
        // }

        for i in 0..num_points {
            let pt = get_point(i);
            pb.line_to(pt.0, pt.1);
        }

        let path = pb.finish();

        self.dt.fill(
            &path,
            &Source::Solid(SolidSource {
                g: self.colours[(outlinecolour) as usize].g,
                r: self.colours[(outlinecolour) as usize].r,
                b: self.colours[(outlinecolour) as usize].b,
                a: 0xff,
            }),
            &DrawOptions::new(),
        );

        // self.dt.stroke(
        //     &path,
        //     &Source::Solid(SolidSource {
        //         g: self.colours[(outlinecolour + 1) as usize].g,
        //         r: self.colours[(outlinecolour + 1) as usize].r,
        //         b: self.colours[(outlinecolour + 1) as usize].b,
        //         a: 0xff,
        //     }),
        //     &StrokeStyle {
        //         cap: LineCap::Round,
        //         join: LineJoin::Round,
        //         width: 10.,
        //         miter_limit: 2.,
        //         dash_array: vec![10., 18.],
        //         dash_offset: 16.,
        //     },
        //     &DrawOptions::new(),
        // );

        println!(
            "Drawing polygon with {} points and fill colour {} and outline colour {}",
            num_points, fillcolour, outlinecolour
        );
    }
}

unsafe extern "C" fn draw_rect_wrap(
    target: *mut DrawingFFI,
    x: c_int,
    y: c_int,
    w: c_int,
    h: c_int,
    colour: c_int,
) {
    unsafe {
        (*(*target).handle).draw_rect(x, y, w, h, colour);
    }
}

unsafe extern "C" fn start_draw_wrap(target: *mut DrawingFFI) {
    unsafe {
        (*(*target).handle).start_draw();
    }
}

unsafe extern "C" fn end_draw_wrap(target: *mut DrawingFFI) {
    unsafe {
        (*(*target).handle).end_draw();
    }
}

unsafe extern "C" fn draw_polygon_wrap(
    target: *mut DrawingFFI,
    coords: *const c_int,
    npoints: c_int,
    fillcolour: c_int,
    outlinecolour: c_int,
) {
    unsafe {
        (*(*target).handle).draw_polygon(coords, npoints, fillcolour, outlinecolour);
    }
}

#[derive(Debug, Clone)]
pub struct PuzColor {
    r: u8,
    g: u8,
    b: u8,
}

#[repr(C)]
pub struct Frontend {
    midend: *mut MidendFFI,
    drawing_ffi: DrawingApiFFI,
    drawing: Drawing,
    colours: Vec<PuzColor>,
}

impl Frontend {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            midend: std::ptr::null_mut(),
            drawing_ffi: DrawingApiFFI {
                version: 1,
                draw_text: None,
                draw_rect: Some(draw_rect_wrap),
                draw_line: None,
                draw_polygon: Some(draw_polygon_wrap),
                draw_circle: None,
                draw_update: None,
                clip: None,
                unclip: None,
                start_draw: Some(start_draw_wrap),
                end_draw: Some(end_draw_wrap),
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
            drawing: Drawing::new(width, height),
            colours: Vec::new(),
            // drawing_api: DrawingApi::new(),
        }
    }

    /// Gain access to the underlying pixels
    pub fn frame(&self) -> &[u32] {
        self.drawing.frame()
    }

    // fn new_game(&mut self, game: *const GameFFI) {
    //     unsafe {
    //         let midend = midend_new(self, game, &self.drawing_ffi, &mut self.drawing);
    //     }
    // }

    pub fn new_mines(&mut self) {
        unsafe {
            let midend = midend_new(self, &thegame, &self.drawing_ffi, &mut self.drawing);

            // print 'midend' as a pointer value
            println!("midend: {:p}", midend);

            self.midend = midend;
            self.sync_colors();
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        let mut x_val = width as c_int;
        let mut y_val = height as c_int;
        unsafe {
            midend_size(
                self.midend,
                &mut x_val,
                &mut y_val,
                false, /*user_size*/
                2.0,   /*device_pixel_ratio*/
            );
        }

        println!("Post set_size: x = {}, y = {}", x_val, y_val);
    }

    fn sync_colors(&mut self) {
        let mut num_colours: c_int = 0;
        let colours: *mut c_float;

        unsafe {
            colours = midend_colours(self.midend, &mut num_colours);
        }

        unsafe {
            for i in 0..num_colours * 3 {
                println!("colours[{}] = {}", i, (*colours.offset(i as isize)))
            }
        }

        self.colours = Vec::with_capacity(num_colours as usize);
        unsafe {
            for i in 0..num_colours as isize {
                let r = (*colours.offset(i * 3) * 255.) as u8;
                let g = (*colours.offset(i * 3 + 1) * 255.) as u8;
                let b = (*colours.offset(i * 3 + 2) * 255.) as u8;
                self.colours.push(PuzColor { r, g, b });
            }
        }

        self.drawing.colours = self.colours.clone();

        unsafe {
            sfree(colours as *mut c_void);
        }
        println!("Synced {} colors", self.colours.len());
    }

    pub fn new_game(&mut self) {
        unsafe {
            midend_new_game(self.midend);
        }
    }

    pub fn redraw(&mut self) {
        unsafe {
            midend_redraw(self.midend);
        }
    }
}
