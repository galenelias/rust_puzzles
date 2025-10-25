use font_kit::canvas::RasterizationOptions;
use font_kit::family_name::FamilyName;
use font_kit::hinting::HintingOptions;
use font_kit::loaders::core_text::Font;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::Vector2F;
use raqote::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_float, c_int, c_void};

unsafe extern "C" {
    static thegame: GameFFI;
}

const PIXEL_RATIO: f32 = 2.;

// Forward declarations for opaque types
#[repr(C)]
pub struct BlitterFFI {
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
        Option<unsafe extern "C" fn(dr: *mut DrawingFFI, w: c_int, h: c_int) -> *mut BlitterFFI>,
    pub blitter_free: Option<unsafe extern "C" fn(dr: *mut DrawingFFI, bl: *mut BlitterFFI)>,
    pub blitter_save:
        Option<unsafe extern "C" fn(dr: *mut DrawingFFI, bl: *mut BlitterFFI, x: c_int, y: c_int)>,
    pub blitter_load:
        Option<unsafe extern "C" fn(dr: *mut DrawingFFI, bl: *mut BlitterFFI, x: c_int, y: c_int)>,
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

    // int midend_process_key(midend *me, int x, int y, int button);
    fn midend_process_key(me: *mut MidendFFI, x: c_int, y: c_int, button: c_int) -> c_int;

}

#[unsafe(no_mangle)]
pub extern "C" fn frontend_default_colour(_fe: *mut Frontend, output: *mut c_float) {
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
pub unsafe extern "C" fn deactivate_timer(_fe: *mut Frontend) {
    println!("Deactivate timer called");
    // Implementation for deactivating the timer
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn activate_timer(_fe: *mut Frontend) {
    println!("Activate timer called");
    // Implementation for activating the timer
}

// struct RustPuzzleInteroperability {
//     drawing_api: Drawing,
// }

/// Create a raqote::Path for a circle
///
/// # Arguments
///
/// * `radius` - The radius of the circle
/// * `x` - The x coordinate of the center of the circle
/// * `y` - The y coordinate of the center of the circle
///
/// # Example
///
/// ```
/// use raqote_utils::build_circle;
///
/// let circle = build_circle(100.0, 100.0, 100.0);
/// ```
/// Source: https://github.com/sk337/raqote-utils/blob/main/src/lib.rs
pub fn build_circle(radius: f32, x: f32, y: f32) -> raqote::Path {
    let mut pb = PathBuilder::new();

    let x = x - radius;
    let y = y + radius;

    let offset = 0.5522847498 * radius;

    pb.move_to(x + radius, y);

    pb.cubic_to(
        x + radius + offset,
        y,
        x + (radius * 2.),
        y - radius + offset,
        x + (radius * 2.),
        y - radius,
    );

    pb.cubic_to(
        x + (radius * 2.),
        y - radius - offset,
        x + radius + offset,
        y - (radius * 2.),
        x + radius,
        y - (radius * 2.),
    );

    pb.cubic_to(
        x + radius - offset,
        y - (radius * 2.),
        x,
        y - radius - offset,
        x,
        y - radius,
    );

    pb.cubic_to(x, y - offset, x + radius - offset, y, x + radius, y);

    pb.finish()
}

#[repr(C)] // Needed?
struct Drawing {
    dt: DrawTarget,
    colours: Vec<PuzColor>,
    colours_source: Vec<SolidSource>,
}

impl Drawing {
    fn new(width: u32, height: u32) -> Self {
        let drawing = Drawing {
            dt: DrawTarget::new(width as i32, height as i32),
            colours: Vec::new(),
            colours_source: Vec::new(),
        };
        return drawing;
    }

    /// Gain access to the underlying pixels
    pub fn frame(&self) -> &[u32] {
        self.dt.get_data()
    }

    fn start_draw(self: &mut Drawing) {
        // println!("start_draw called");
        // self.dt.clear(self.colours_source[0]);
    }

    fn end_draw(self: &mut Drawing) {
        // println!("end_draw called");
    }

    fn clear(self: &mut Drawing) {
        self.dt.clear(self.colours_source[0]);
    }

    fn draw_rect(self: &mut Drawing, x: c_int, y: c_int, w: c_int, h: c_int, colour: c_int) {
        // if w > 200 {
        //     return;
        // }

        self.dt.fill_rect(
            x as f32,
            y as f32,
            w as f32,
            h as f32,
            &Source::Solid(self.colours_source[colour as usize]),
            &DrawOptions::new(),
        );
    }

    fn draw_polygon(
        self: &mut Drawing,
        points: *const c_int,
        num_points: c_int,
        fillcolour: c_int,
        _outlinecolour: c_int,
    ) {
        let mut pb = PathBuilder::new();

        let get_point = |i: c_int| unsafe {
            let x = *points.offset(i as isize * 2);
            let y = *points.offset(i as isize * 2 + 1);
            (x as f32, y as f32)
        };

        let start = get_point(0);
        pb.move_to(start.0, start.1);

        for i in 0..num_points {
            let pt = get_point(i);
            pb.line_to(pt.0, pt.1);
        }

        let path = pb.finish();

        self.dt.fill(
            &path,
            &Source::Solid(self.colours_source[fillcolour as usize]),
            &DrawOptions::new(),
        );

        // self.dt.stroke(
        //     &path,
        //     &Source::Solid(self.colours_source[colour as usize]),
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
    }

    fn draw_line(self: &mut Drawing, x1: c_int, y1: c_int, x2: c_int, y2: c_int, colour: c_int) {
        let mut pb = PathBuilder::new();

        pb.move_to(x1 as f32, y1 as f32);
        pb.line_to(x2 as f32, y2 as f32);
        let path = pb.finish();

        self.dt.stroke(
            &path,
            &Source::Solid(self.colours_source[colour as usize]),
            &StrokeStyle::default(),
            &DrawOptions::new(),
        );
    }

    fn draw_circle(
        self: &mut Drawing,
        x: c_int,
        y: c_int,
        radius: c_int,
        fillcolour: c_int,
        _outlinecolour: c_int, // TODO
    ) {
        let path = build_circle(radius as f32, x as f32, y as f32);

        self.dt.fill(
            &path,
            &Source::Solid(self.colours_source[fillcolour as usize]),
            &DrawOptions::new(),
        );
    }

    fn measure_text(self: &mut Drawing, font: &Font, point_size: f32, text: &str) -> Vector2F {
        let mut offset = Vector2F::new(0., 0.);
        let mut height = None;
        for c in text.chars() {
            let id = font.glyph_for_char(c).unwrap();
            offset += font.advance(id).unwrap() * point_size / 24. / 96.;

            if height.is_none() {
                let bounds = font.raster_bounds(
                    id,
                    point_size,
                    Transform2F::default(),
                    HintingOptions::None,
                    RasterizationOptions::GrayscaleAa,
                );

                height = Some(bounds.unwrap().height());
            }
        }
        Vector2F::new(offset.x(), height.unwrap_or(0) as f32)
    }

    fn draw_text(
        self: &mut Drawing,
        x: c_int,
        y: c_int,
        _fonttype: c_int,
        fontsize: c_int,
        align: c_int,
        colour: c_int,
        text: *const c_char,
    ) {
        let text = unsafe { CStr::from_ptr(text).to_string_lossy().into_owned() };

        // TODO: Cache somewhere
        let font = SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::new())
            .unwrap()
            .load()
            .unwrap();

        let render_size = self.measure_text(&font, fontsize as f32, &text);

        let align_x_offset = if (align & ALIGN_HCENTRE) != 0 {
            -(render_size.x() / 2.)
        } else if (align & ALIGN_HRIGHT) != 0 {
            -(render_size.x())
        } else {
            0.
        };

        let align_y_offset = if (align & ALIGN_VCENTRE) != 0 {
            render_size.y() / 2.
        } else {
            0.
        };

        self.dt.draw_text(
            &font,
            fontsize as f32,
            &text,
            Point::new(x as f32 + align_x_offset, y as f32 + align_y_offset),
            &Source::Solid(self.colours_source[colour as usize]),
            &DrawOptions::new(),
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

// void draw_circle(drawing *dr, int cx, int cy, int radius,
//                  int fillcolour, int outlinecolour);
unsafe extern "C" fn draw_circle_wrap(
    target: *mut DrawingFFI,
    cx: c_int,
    cy: c_int,
    radius: c_int,
    fillcolour: c_int,
    outlinecolour: c_int,
) {
    unsafe {
        (*(*target).handle).draw_circle(cx, cy, radius, fillcolour, outlinecolour);
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
//
// void (*draw_line)(drawing *dr, int x1, int y1, int x2, int y2, int colour);
unsafe extern "C" fn draw_line_wrap(
    target: *mut DrawingFFI,
    x1: c_int,
    y1: c_int,
    x2: c_int,
    y2: c_int,
    colour: c_int,
) {
    unsafe {
        (*(*target).handle).draw_line(x1, y1, x2, y2, colour);
    }
}

// void draw_text(drawing *dr, int x, int y, int fonttype, int fontsize, int align, int colour, const char *text);
unsafe extern "C" fn draw_text_wrap(
    target: *mut DrawingFFI,
    x: c_int,
    y: c_int,
    fonttype: c_int,
    fontsize: c_int,
    align: c_int,
    colour: c_int,
    text: *const c_char,
) {
    unsafe {
        (*(*target).handle).draw_text(x, y, fonttype, fontsize, align, colour, text);
    }
}

#[derive(Debug, Clone)]
pub struct PuzColor {
    r: u8,
    g: u8,
    b: u8,
}

pub enum MouseButton {
    Left,
    Right,
}

pub enum Input {
    MouseDown((MouseButton, (f32, f32))),
    MouseUp((MouseButton, (f32, f32))),
}

const LEFT_BUTTON: c_int = 0x200;
const RIGHT_BUTTON: c_int = 0x202;
const LEFT_RELEASE: c_int = 0x206;
const RIGHT_RELEASE: c_int = 0x207;

// const ALIGN_VNORMAL: c_int = 0x000;
const ALIGN_VCENTRE: c_int = 0x100;

// const ALIGN_HLEFT: c_int = 0x000;
const ALIGN_HCENTRE: c_int = 0x001;
const ALIGN_HRIGHT: c_int = 0x002;

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
                draw_text: Some(draw_text_wrap),
                draw_rect: Some(draw_rect_wrap),
                draw_line: Some(draw_line_wrap),
                draw_polygon: Some(draw_polygon_wrap),
                draw_circle: Some(draw_circle_wrap),
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

    pub fn new_mines(&mut self) {
        unsafe {
            let midend = midend_new(self, &thegame, &self.drawing_ffi, &mut self.drawing);

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
                false,              /*user_size*/
                PIXEL_RATIO as f64, /*device_pixel_ratio*/
            );
        }

        self.drawing.clear();
    }

    fn sync_colors(&mut self) {
        let mut num_colours: c_int = 0;
        let colours: *mut c_float;

        unsafe {
            colours = midend_colours(self.midend, &mut num_colours);
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

        self.drawing.colours_source = self
            .colours
            .iter()
            .map(|color| SolidSource {
                g: color.g,
                r: color.r,
                b: color.b,
                a: 0xff,
            })
            .collect();

        self.drawing.colours = self.colours.clone();

        unsafe {
            sfree(colours as *mut c_void);
        }
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

    pub fn process_input(&mut self, input: &Input) {
        let button = match input {
            Input::MouseDown((MouseButton::Left, _)) => LEFT_BUTTON,
            Input::MouseDown((MouseButton::Right, _)) => RIGHT_BUTTON,
            Input::MouseUp((MouseButton::Left, _)) => LEFT_RELEASE,
            Input::MouseUp((MouseButton::Right, _)) => RIGHT_RELEASE,
        };

        let (x, y) = match input {
            Input::MouseDown((_, (x, y))) | Input::MouseUp((_, (x, y))) => {
                ((*x / PIXEL_RATIO) as c_int, (*y / PIXEL_RATIO) as c_int)
            }
        };

        println!("midend_process_key: x={}, y={}", x, y);
        unsafe {
            midend_process_key(self.midend, x, y, button);
        }
    }
}
