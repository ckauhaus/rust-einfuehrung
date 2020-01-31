#![allow(unused_imports)]

use crossbeam::{channel::bounded, thread};
use image::png::PNGEncoder;
use num::Complex;
use std::path::{Path, PathBuf};
use std::u8::MAX;
use structopt::StructOpt;

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

    // Parallel rendering: render single line
    #[cfg(feature = "parallel")]
    fn render_line(&self, y: u32, line: &mut [u8]) {
        for x in 0..self.width {
            let point = self.px2complex(x, y);
            let i = escape_time(point);
            line[x as usize] = MAX - i;
        }
    }

    // Parallel rendering: buffer & thread management, IPC
    #[cfg(feature = "parallel")]
    fn render(&self) -> Vec<u8> {
        // pixel buffer
        let mut pxs = vec![0; (self.width * self.height) as usize];
        // yet unprocessed part of the pixel buffer
        let mut pxs_rest: &mut [u8] = &mut pxs;
        let cpus = num_cpus::get();
        let (y_tx, y_rx) = bounded::<(u32, &mut [u8])>(cpus);
        thread::scope(move |sc| {
            for _ in 0..cpus {
                let y_rx = y_rx.clone();
                sc.spawn(move |_| {
                    // receive mutable borrow over channel and fill it
                    for (y, mut line) in y_rx {
                        self.render_line(y, &mut line);
                    }
                });
            }
            for y in 0..self.height {
                // chop first line off and hand it as mutable slice to worker thread
                let (head_line, tail) = pxs_rest.split_at_mut(self.width as usize);
                y_tx.send((y, head_line)).expect("IPC channel broken");
                pxs_rest = tail;
            }
        })
        .expect("thread panic");
        pxs
    }

    // Sequential rendering
    #[cfg(not(feature = "parallel"))]
    fn render(&self) -> Vec<u8> {
        let mut pxs = vec![0; (self.width * self.height) as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                let point = self.px2complex(x, y);
                let i = escape_time(point);
                pxs[(y * self.width + x) as usize] = MAX - i;
            }
        }
        pxs
    }
}

// Mandelbrot iteration, see https://en.wikipedia.org/wiki/Mandelbrot_set
fn escape_time(c: Complex<f64>) -> u8 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return i;
        }
    }
    MAX
}

fn write(pxs: &[u8], dimen: &Dimen, filename: &Path) -> Result<(), std::io::Error> {
    let f = std::fs::File::create(filename)?;
    PNGEncoder::new(f).encode(pxs, dimen.width, dimen.height, image::ColorType::Gray(8))
}

/// Mandelbrot set generator
#[derive(StructOpt, Debug)]
struct Opt {
    /// Image width ins pixels
    #[structopt(short, long, default_value = "640")]
    width: u32,
    /// Image height in pixels
    #[structopt(short, long, default_value = "480")]
    height: u32,
    /// Bottom left corner of the window into the complex plane
    #[structopt(short, long)]
    bottom_left: Complex<f64>,
    /// Top right corner of the window into the complex plane
    #[structopt(short, long)]
    top_right: Complex<f64>,
    /// Where to write the PNG file
    #[structopt(parse(from_os_str))]
    filename: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let dimen = Dimen::new(opt.width, opt.height, opt.bottom_left, opt.top_right);
    let data = dimen.render();
    if let Err(e) = write(&data, &dimen, &opt.filename) {
        eprintln!(
            "Error: Cannot write output to '{}': {}",
            opt.filename.display(),
            e
        );
        std::process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_escape() {
        assert_eq!(escape_time(Complex::new(0.5, 0.0)), 4);
        assert_eq!(escape_time(Complex::new(2.0, -0.1)), 0);
        assert_eq!(escape_time(Complex::new(-1.0, -0.4)), 6);
    }

    #[test]
    fn should_not_escape() {
        assert_eq!(escape_time(Complex::default()), MAX);
        assert_eq!(escape_time(Complex::new(-2.0, 0.0)), MAX);
        assert_eq!(escape_time(Complex::new(0.0, 0.5)), MAX);
    }

    #[test]
    fn coord_trans() {
        let d = Dimen::new(80, 40, Complex::new(-1.0, -1.0), Complex::new(1.0, 1.0));
        assert_eq!(d.px2complex(20, 30), Complex::new(-0.5, -0.5));
    }
}
