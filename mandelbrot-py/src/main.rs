use gumdrop::Options;
use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::error::Error;
use std::path::{Path, PathBuf};

// Mandelbrot iteration, see https://en.wikipedia.org/wiki/Mandelbrot_set
fn escape_time(c: Complex<f64>) -> Option<u8> {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..u8::MAX {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

#[test]
fn should_escape() {
    assert_eq!(escape_time(Complex::new(0.5, 0.0)), Some(4));
    assert_eq!(escape_time(Complex::new(2.0, -0.1)), Some(0));
    assert_eq!(escape_time(Complex::new(-1.0, -0.4)), Some(6));
}

#[test]
fn should_not_escape() {
    assert_eq!(escape_time(Complex::default()), None);
    assert_eq!(escape_time(Complex::new(-2.0, 0.0)), None);
    assert_eq!(escape_time(Complex::new(0.0, 0.5)), None);
}

// Image dimensions both in pixels and in the complex plane
#[derive(Debug)]
struct Dimen {
    width: u32,
    height: u32,
    origin: Complex<f64>,
    window: Complex<f64>,
}

impl Dimen {
    fn new(width: u32, height: u32, bot_right: Complex<f64>, top_left: Complex<f64>) -> Self {
        Self {
            width,
            height,
            origin: bot_right,
            window: top_left - bot_right,
        }
    }

    fn px2complex(&self, x: u32, y: u32) -> Complex<f64> {
        Complex::new(
            self.origin.re + x as f64 * self.window.re / self.width as f64,
            self.origin.im + (self.height - y) as f64 * self.window.im / self.height as f64,
        )
    }
}

#[test]
fn coord_trans() {
    let d = Dimen::new(80, 40, Complex::new(-1.0, -1.0), Complex::new(1.0, 1.0));
    assert_eq!(d.px2complex(20, 30), Complex::new(-0.5, -0.5));
}

fn render(d: &Dimen) -> Vec<Vec<u8>> {
    let mut buf: Vec<Vec<u8>> = (0..d.height).map(|_| vec![0; d.width as usize]).collect();
    for y in 0..d.height {
        for x in 0..d.width {
            let point = d.px2complex(x, y);
            buf[y as usize][x as usize] = match escape_time(point) {
                Some(i) => u8::MAX - i,
                None => 0,
            }
        }
    }
    buf
}

fn write(pxs: &[u8], dimen: &Dimen, filename: &Path) -> Result<(), Box<dyn Error>> {
    let f = std::fs::File::create(filename)?;
    Ok(PNGEncoder::new(f).encode(pxs, dimen.width, dimen.height, ColorType::L8)?)
}

/// Mandelbrot set generator
#[derive(Options, Debug)]
struct Opt {
    /// Image width ins pixels
    #[options(default = "1024")]
    width: u32,
    /// Image height in pixels
    #[options(default = "768")]
    height: u32,
    /// Bottom left corner of the window into the complex plane
    #[options(meta = "COMPLEX", default = "-1.2+0.2i")]
    bottom_left: Complex<f64>,
    /// Top right corner of the window into the complex plane
    #[options(meta = "COMPLEX", default = "-1.0+0.35i")]
    top_right: Complex<f64>,
    /// Where to write the PNG file
    #[options(free)]
    filename: PathBuf,
    /// Show command line usage
    #[options(no_short, help_flag)]
    help: bool,
}

fn main() {
    let opt = Opt::parse_args_default_or_exit();
    let dimen = Dimen::new(opt.width, opt.height, opt.bottom_left, opt.top_right);
    let pixels = render(&dimen);
    if let Err(e) = write(&pixels.concat(), &dimen, &opt.filename) {
        eprintln!(
            "Error: Cannot write output to '{}': {}",
            opt.filename.display(),
            e
        );
        std::process::exit(1);
    }
}
