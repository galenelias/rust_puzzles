#![allow(improper_ctypes)]

use font_kit::canvas::RasterizationOptions;
use font_kit::family_name::FamilyName;
use font_kit::hinting::HintingOptions;
#[cfg(target_os = "macos")]
use font_kit::loaders::core_text::Font;
#[cfg(target_os = "windows")]
use font_kit::loaders::directwrite::Font;

use font_kit::properties::{Properties, Stretch};
use font_kit::source::SystemSource;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::Vector2F;
use raqote::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_double, c_float, c_int, c_void};
use std::time::Instant;
use winit::keyboard::KeyCode;

unsafe extern "C" {
    static thegame: GameFFI;
}

const PIXEL_RATIO: f32 = 2.;
const STATUS_BAR_HEIGHT: i32 = 30;

pub struct Blitter {
    dt: DrawTarget,
    width: i32,
    height: i32,
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
        Option<unsafe extern "C" fn(dr: *mut DrawingFFI, w: c_int, h: c_int) -> *mut Blitter>,
    pub blitter_free: Option<unsafe extern "C" fn(dr: *mut DrawingFFI, bl: *mut Blitter)>,
    pub blitter_save:
        Option<unsafe extern "C" fn(dr: *mut DrawingFFI, bl: *mut Blitter, x: c_int, y: c_int)>,
    pub blitter_load:
        Option<unsafe extern "C" fn(dr: *mut DrawingFFI, bl: *mut Blitter, x: c_int, y: c_int)>,
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

pub struct DrawingFFI {
    _drawing_api: *const DrawingApiFFI,
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

unsafe extern "C" {
    fn midend_new(
        fe: *mut Frontend,
        game: *const GameFFI,
        drapi: *const DrawingApiFFI,
        drhandle: *mut Drawing,
    ) -> *mut MidendFFI;
    fn midend_redraw(me: *mut MidendFFI);
    fn midend_timer(me: *mut MidendFFI, tplus: c_float);
    fn midend_size(
        me: *mut MidendFFI,
        x: *mut c_int,
        y: *mut c_int,
        user_size: bool,
        device_pixel_ratio: c_double,
    );
    fn midend_new_game(me: *mut MidendFFI);
    fn midend_colours(me: *mut MidendFFI, ncolours: *mut c_int) -> *mut c_float;
    fn sfree(ptr: *mut c_void);
    fn midend_process_key(me: *mut MidendFFI, x: c_int, y: c_int, button: c_int) -> c_int;
    fn midend_wants_statusbar(me: *mut MidendFFI) -> bool;
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn deactivate_timer(fe: *mut Frontend) {
    // Implementation for deactivating the timer
    unsafe {
        (*fe).is_timer_active = false;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn activate_timer(fe: *mut Frontend) {
    // println!("Activate timer called");
    unsafe {
        (*fe).timer_start = Instant::now();
        (*fe).is_timer_active = true;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn document_add_puzzle() {
    println!("document_add_puzzle called");
}

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

struct Drawing {
    dt: DrawTarget,
    colours: Vec<PuzColor>,
    colours_source: Vec<SolidSource>,
    end_draw_callback: Option<Box<dyn Fn() + 'static>>,
    status_text: Option<String>,
    width: u32,
    height: u32,
}

impl Drawing {
    fn new(width: u32, height: u32) -> Self {
        let drawing = Drawing {
            dt: DrawTarget::new(width as i32, height as i32),
            colours: Vec::new(),
            colours_source: Vec::new(),
            end_draw_callback: None,
            status_text: None,
            width,
            height,
        };
        return drawing;
    }

    fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.dt = DrawTarget::new(width as i32, height as i32);
    }

    /// Gain access to the underlying pixels
    pub fn frame(&self) -> &[u32] {
        self.dt.get_data()
    }

    /// Set a callback to be invoked when end_draw is called.
    ///
    /// This is an internal method used by the Frontend struct.
    fn set_end_draw_callback<F>(&mut self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.end_draw_callback = Some(Box::new(callback));
    }

    fn start_draw(self: &mut Drawing) {
        // Nothing to do?
    }

    fn end_draw(self: &mut Drawing) {
        // Draw status bar if there's status text
        if let Some(ref status_text) = self.status_text.clone() {
            self.draw_status_bar(status_text);
        }

        if let Some(ref callback) = self.end_draw_callback {
            callback();
        }
    }

    fn clear(self: &mut Drawing) {
        self.dt.clear(self.colours_source[0]);
    }

    fn draw_rect(self: &mut Drawing, x: c_int, y: c_int, w: c_int, h: c_int, colour: c_int) {
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

        for i in 1..num_points {
            let pt = get_point(i);
            pb.line_to(pt.0, pt.1);
        }

        pb.line_to(start.0, start.1);

        let path = pb.finish();

        if fillcolour != -1 {
            self.dt.fill(
                &path,
                &Source::Solid(self.colours_source[fillcolour as usize]),
                &DrawOptions::new(),
            );
        }

        self.dt.stroke(
            &path,
            &Source::Solid(self.colours_source[outlinecolour as usize]),
            &StrokeStyle::default(),
            &DrawOptions::new(),
        );
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
        outlinecolour: c_int,
    ) {
        let path = build_circle(radius as f32, x as f32, y as f32);

        if fillcolour != -1 {
            self.dt.fill(
                &path,
                &Source::Solid(self.colours_source[fillcolour as usize]),
                &DrawOptions::new(),
            );
        }

        self.dt.stroke(
            &path,
            &Source::Solid(self.colours_source[outlinecolour as usize]),
            &StrokeStyle::default(),
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

    fn clip(self: &mut Drawing, x: c_int, y: c_int, w: c_int, h: c_int) {
        self.dt.push_clip_rect(IntRect::new(
            IntPoint::new(x, y),
            IntPoint::new(x + w, y + h),
        ));
    }

    fn unclip(self: &mut Drawing) {
        self.dt.pop_clip();
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

    fn blitter_new(self: &mut Drawing, w: i32, h: i32) -> Box<Blitter> {
        Box::new(Blitter {
            dt: DrawTarget::new(w, h),
            width: w,
            height: h,
        })
    }

    fn blitter_save(self: &mut Drawing, blitter: &mut Blitter, x: i32, y: i32) {
        blitter.dt.copy_surface(
            &self.dt,
            IntRect::new(
                IntPoint::new(x, y),
                IntPoint::new(x + blitter.width, y + blitter.height),
            ),
            IntPoint::new(0, 0),
        );
    }

    fn blitter_load(self: &mut Drawing, blitter: &mut Blitter, x: i32, y: i32) {
        self.dt.copy_surface(
            &blitter.dt,
            IntRect::new(
                IntPoint::new(0, 0),
                IntPoint::new(blitter.width, blitter.height),
            ),
            IntPoint::new(x, y),
        );
    }

    fn draw_status_bar(&mut self, text: &str) {
        const FONT_SIZE: f32 = 16.0;
        const PADDING: f32 = 8.0;

        let status_bar_y = self.height as i32 - STATUS_BAR_HEIGHT;

        // Draw status bar background with a light gray color
        let status_bg = SolidSource::from_unpremultiplied_argb(0xff, 0xe0, 0xe0, 0xe0);

        self.dt.fill_rect(
            0.0,
            status_bar_y as f32,
            self.width as f32,
            STATUS_BAR_HEIGHT as f32,
            &Source::Solid(status_bg),
            &DrawOptions::new(),
        );

        // Draw a separator line at the top of the status bar
        let separator_color = SolidSource::from_unpremultiplied_argb(0xff, 0x80, 0x80, 0x80);
        self.dt.fill_rect(
            0.0,
            status_bar_y as f32,
            self.width as f32,
            1.0,
            &Source::Solid(separator_color),
            &DrawOptions::new(),
        );

        // Draw the status text
        let font = SystemSource::new()
            .select_best_match(
                &[FamilyName::Monospace],
                &Properties::new().stretch(Stretch::ULTRA_EXPANDED),
            )
            .unwrap()
            .load()
            .unwrap();

        // Use a dark color for the text
        let text_color = SolidSource::from_unpremultiplied_argb(0xff, 0x00, 0x00, 0x00);

        // Position text with padding from the left edge and vertically centered
        let text_y = status_bar_y as f32 + (STATUS_BAR_HEIGHT as f32 / 2.0) + (FONT_SIZE / 2.5);

        self.dt.draw_text(
            &font,
            FONT_SIZE,
            text,
            Point::new(PADDING, text_y),
            &Source::Solid(text_color),
            &DrawOptions::new(),
        );
    }

    fn set_status_text(&mut self, text: String) {
        self.status_text = Some(text);
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

unsafe extern "C" fn status_bar_wrap(target: *mut DrawingFFI, text: *const c_char) {
    let text_str = unsafe { CStr::from_ptr(text).to_string_lossy().into_owned() };
    let drawing = unsafe { &mut *((*target).handle as *mut Drawing) };
    drawing.set_status_text(text_str);
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

// void draw_thick_line(drawing *dr, float thickness, float x1, float y1, float x2, float y2, int colour);
unsafe extern "C" fn draw_thick_line_wrap(
    target: *mut DrawingFFI,
    thickness: c_float,
    x1: c_float,
    y1: c_float,
    x2: c_float,
    y2: c_float,
    colour: c_int,
) {
    println!(
        "draw_thick_line_wrap called: target={:?}, thickness={}, x1={}, y1={}, x2={}, y2={}, colour={}",
        target, thickness, x1, y1, x2, y2, colour
    );
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

unsafe extern "C" fn clip_wrap(target: *mut DrawingFFI, x: c_int, y: c_int, w: c_int, h: c_int) {
    unsafe {
        (*(*target).handle).clip(x, y, w, h);
    }
}

unsafe extern "C" fn unclip_wrap(target: *mut DrawingFFI) {
    unsafe {
        (*(*target).handle).unclip();
    }
}

unsafe extern "C" fn blitter_new_wrap(target: *mut DrawingFFI, w: c_int, h: c_int) -> *mut Blitter {
    let blitter = unsafe { (*(*target).handle).blitter_new(w, h) };
    Box::into_raw(blitter) as *mut Blitter
}

unsafe extern "C" fn blitter_free_wrap(_target: *mut DrawingFFI, blitter: *mut Blitter) {
    if !blitter.is_null() {
        unsafe {
            let _ = Box::from_raw(blitter);
        }
    }
}

unsafe extern "C" fn blitter_save_wrap(
    target: *mut DrawingFFI,
    blitter: *mut Blitter,
    x: c_int,
    y: c_int,
) {
    unsafe {
        (*(*target).handle).blitter_save(&mut *blitter, x, y);
    }
}

unsafe extern "C" fn blitter_load_wrap(
    target: *mut DrawingFFI,
    blitter: *mut Blitter,
    x: c_int,
    y: c_int,
) {
    unsafe {
        (*(*target).handle).blitter_load(&mut *blitter, x, y);
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

pub enum Key {
    Character(char),
    Special(KeyCode),
}

pub enum Input {
    MouseDown((MouseButton, (f32, f32))),
    MouseHeld((MouseButton, (f32, f32))),
    MouseUp((MouseButton, (f32, f32))),
    KeyDown(Key),
}

const LEFT_BUTTON: c_int = 0x200;
const RIGHT_BUTTON: c_int = 0x202;
const LEFT_DRAG: c_int = 0x203;
const RIGHT_DRAG: c_int = 0x205;
const LEFT_RELEASE: c_int = 0x206;
const RIGHT_RELEASE: c_int = 0x208;
const CURSOR_UP: c_int = 0x209;
const CURSOR_DOWN: c_int = 0x20A;
const CURSOR_LEFT: c_int = 0x20B;
const CURSOR_RIGHT: c_int = 0x20C;

// const ALIGN_VNORMAL: c_int = 0x000;
const ALIGN_VCENTRE: c_int = 0x100;

// const ALIGN_HLEFT: c_int = 0x000;
const ALIGN_HCENTRE: c_int = 0x001;
const ALIGN_HRIGHT: c_int = 0x002;

pub struct Frontend {
    midend: *mut MidendFFI,
    drawing_ffi: DrawingApiFFI,
    drawing: Drawing,
    colours: Vec<PuzColor>,
    is_timer_active: bool,
    timer_start: Instant,
}

impl Frontend {
    pub fn new() -> Self {
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
                clip: Some(clip_wrap),
                unclip: Some(unclip_wrap),
                start_draw: Some(start_draw_wrap),
                end_draw: Some(end_draw_wrap),
                status_bar: Some(status_bar_wrap),
                blitter_new: Some(blitter_new_wrap),
                blitter_free: Some(blitter_free_wrap),
                blitter_save: Some(blitter_save_wrap),
                blitter_load: Some(blitter_load_wrap),
                begin_doc: None,    // Printing specific. Not implemented.
                begin_page: None,   // Printing specific. Not implemented.
                begin_puzzle: None, // Printing specific. Not implemented.
                end_puzzle: None,   // Printing specific. Not implemented.
                end_page: None,     // Printing specific. Not implemented.
                end_doc: None,      // Printing specific. Not implemented.
                line_width: None,   // Printing specific. Not implemented.
                line_dotted: None,  // Printing specific. Not implemented.
                text_fallback: None,
                draw_thick_line: Some(draw_thick_line_wrap),
            },
            drawing: Drawing::new(1, 1),
            colours: Vec::new(),
            is_timer_active: false,
            timer_start: Instant::now(),
        }
    }

    /// Gain access to the underlying pixels
    pub fn frame(&self) -> &[u32] {
        self.drawing.frame()
    }

    /// Set a callback to be invoked when end_draw is called.
    pub fn set_end_draw_callback<F>(&mut self, callback: F)
    where
        F: Fn() + 'static,
    {
        self.drawing.set_end_draw_callback(callback);
    }

    pub fn new_midend(&mut self) {
        unsafe {
            let midend = midend_new(self, &thegame, &self.drawing_ffi, &mut self.drawing);

            self.midend = midend;
            self.sync_colors();
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) -> (u32, u32) {
        let mut x_val = width as c_int;
        let mut y_val = height as c_int;

        if self.wants_statusbar() {
            y_val -= STATUS_BAR_HEIGHT;
        }

        unsafe {
            midend_size(
                self.midend,
                &mut x_val,
                &mut y_val,
                false,              /*user_size*/
                PIXEL_RATIO as f64, /*device_pixel_ratio*/
            );
        }

        if self.wants_statusbar() {
            y_val += STATUS_BAR_HEIGHT;
        }

        self.drawing.set_size(x_val as u32, y_val as u32);
        self.drawing.clear();

        (x_val as u32, y_val as u32)
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

    pub fn wants_statusbar(&self) -> bool {
        unsafe { midend_wants_statusbar(self.midend) }
    }

    pub fn tick(&mut self) {
        // We can't call this with tiny elapsed times, otherwise the midend code doesn't
        // accumulate the time correctly, and the timer will not work properly.
        if self.is_timer_active && self.timer_start.elapsed().as_millis() >= 10 {
            unsafe {
                midend_timer(self.midend, self.timer_start.elapsed().as_secs_f32());
            }
            self.timer_start = Instant::now();
        }
    }

    pub fn is_timer_active(&self) -> bool {
        self.is_timer_active
    }

    pub fn process_input(&mut self, input: &Input) {
        let button = match input {
            Input::MouseDown((MouseButton::Left, _)) => LEFT_BUTTON,
            Input::MouseDown((MouseButton::Right, _)) => RIGHT_BUTTON,
            Input::MouseHeld((MouseButton::Left, _)) => LEFT_DRAG,
            Input::MouseHeld((MouseButton::Right, _)) => RIGHT_DRAG,
            Input::MouseUp((MouseButton::Left, _)) => LEFT_RELEASE,
            Input::MouseUp((MouseButton::Right, _)) => RIGHT_RELEASE,
            Input::KeyDown(Key::Special(KeyCode::ArrowLeft)) => CURSOR_LEFT,
            Input::KeyDown(Key::Special(KeyCode::ArrowDown)) => CURSOR_DOWN,
            Input::KeyDown(Key::Special(KeyCode::ArrowRight)) => CURSOR_RIGHT,
            Input::KeyDown(Key::Special(KeyCode::ArrowUp)) => CURSOR_UP,
            Input::KeyDown(Key::Character(character)) => *character as i32,
            _ => unreachable!(),
        };

        let (x, y) = match input {
            Input::MouseDown((_, (x, y)))
            | Input::MouseHeld((_, (x, y)))
            | Input::MouseUp((_, (x, y))) => (*x as c_int, *y as c_int),
            Input::KeyDown(_) => (0, 0),
        };

        // TODO: Handle retina resolution. Mac mouse events don't map to logical coordinates.
        #[cfg(target_os = "macos")]
        let (x, y) = ((x / 2), (y / 2));

        unsafe {
            midend_process_key(self.midend, x, y, button);
        }
    }
}
