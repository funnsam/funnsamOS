use bootloader_api::info::{*, PixelFormat::*};

pub static mut FRAMEBUFFER: *mut u8 = core::ptr::null_mut();
pub static mut FRAMEBUFFER_SIZE: usize = 0;
pub static mut FRAMEBUFFER_INFO: Option<FrameBufferInfo> = None;

pub unsafe fn init(boot_info: &'static bootloader_api::BootInfo) {
    FRAMEBUFFER_INFO = Some(boot_info.framebuffer.as_ref().unwrap().info());
    let fb = boot_info.framebuffer.as_ref().unwrap().buffer();
    FRAMEBUFFER      = fb.as_ptr() as *mut u8;
    FRAMEBUFFER_SIZE = fb.len();

    get_framebuffer().fill(0);

    init_console();
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub unsafe fn get_framebuffer() -> &'static mut [u8] {
    unsafe { core::slice::from_raw_parts_mut(FRAMEBUFFER, FRAMEBUFFER_SIZE) }
}

pub unsafe fn get_framebuffer_info() -> FrameBufferInfo {
    FRAMEBUFFER_INFO.unwrap()
}

unsafe fn plot_color(i: usize, pc: Color, fb: &mut [u8]) {
    let info = get_framebuffer_info();
    match info.pixel_format {
        Rgb => {
            fb[i + 0] = pc.r;
            fb[i + 1] = pc.g;
            fb[i + 2] = pc.b;
        },
        Bgr => {
            fb[i + 0] = pc.b;
            fb[i + 1] = pc.g;
            fb[i + 2] = pc.r;
        },
        _ => panic!("Format {:?} isn't supported!", info.pixel_format)
    }
}

pub unsafe fn draw_pixel(px: usize, py: usize, pc: Color) {
    let fb   = get_framebuffer();
    let info = get_framebuffer_info();
    let i = (px + info.stride * py) * info.bytes_per_pixel;
    plot_color(i, pc, fb);
}

pub unsafe fn batch_draw(px: usize, py: usize, sx: usize, sy: usize, pd: &[Color]) {
    let fb   = get_framebuffer();
    let info = get_framebuffer_info();
    let mut _i = (px + info.stride * py) * info.bytes_per_pixel;
    let mut j  = 0;

    for _y in 0..sy {
        let mut i = _i;
        for _x in 0..sx {
            plot_color(i, pd[j], fb);

            i += info.bytes_per_pixel;
            j += 1;
        }
        _i += info.stride * info.bytes_per_pixel;
    }
}


use crate::psf;
pub const FONT: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/Tamsyn8x16r.psf"));
pub static mut PSF_FONT: Option<psf::PSF2Font<'static>> = None;

unsafe fn init_console() {
    PSF_FONT = psf::PSF2Font::parse(FONT).ok();

    let s = PSF_FONT.as_ref().unwrap().glyph_size();
    CONSOLE_XSIZE = get_framebuffer_info().width  / s.0 as usize;
    CONSOLE_YSIZE = get_framebuffer_info().height / s.1 as usize;
}

use core::fmt::Write;

pub struct Console;
pub static CONSOLE: Console = Console;
pub static mut CONSOLE_X: usize = 0;
pub static mut CONSOLE_Y: usize = 0;
pub static mut CONSOLE_XSIZE: usize = 0;
pub static mut CONSOLE_YSIZE: usize = 0;

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for i in s.chars() {
            unsafe { putc(i); }
        }
        Ok(())
    }
}

pub unsafe fn puts(a: core::fmt::Arguments) -> core::fmt::Result {
    (&mut *(&CONSOLE as *const Console as *mut Console)).write_fmt(a)
}

pub unsafe fn putc(c: char) {
    match c {
        '\n' => new_line(),
        _ => {
            let s = PSF_FONT.as_ref().unwrap().glyph_size();
            putc_at(CONSOLE_X * s.0 as usize, CONSOLE_Y * s.1 as usize, c as u32);

            if CONSOLE_X >= CONSOLE_XSIZE { new_line(); } else { CONSOLE_X += 1; }
        }
    }
}

pub unsafe fn new_line() {
    CONSOLE_X = 0;
    CONSOLE_Y = (CONSOLE_Y+1) % CONSOLE_YSIZE;
}

pub unsafe fn putc_at(x: usize, y: usize, c: u32) {
    let s = PSF_FONT.as_ref().unwrap().glyph_size();

    let g = PSF_FONT.as_ref().unwrap().glyph(c);
    let g = match g {
        Some(g) => g,
        None => {
            PSF_FONT.as_ref().unwrap().glyph(0).unwrap()
        }
    };

    let stride = (s.0 as usize + 7) / 8;
    let mut f = alloc::vec![Color::default(); s.0 as usize * s.1 as usize];

    for y in 0..s.1     as usize {
		for x in 0..s.0 as usize {
			let byte = y * stride + (x / 8);
			let bit = 7 - (x % 8);
			
			if (g[byte] & (1 << bit)) != 0 {
                f[x + y * s.0 as usize] = Color::new(255, 255, 255);
			}
		}
	}

    batch_draw(x, y, s.0 as usize, s.1 as usize, &f);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        unsafe { $crate::drivers::video::puts(format_args!($($arg)*)).unwrap(); }
    };
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => { $crate::print!("{}\n", format_args!($($arg)*)) };
}
