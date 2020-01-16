#![allow(unused_imports)]

use crossbeam::{channel::bounded, thread};
use num::Complex;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

const MAXITER: u32 = 0xff;

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

    #[cfg(feature = "parallel")]
    fn render_line(&self, y: u32) -> Vec<u8> {
        let mut line = vec![0; self.width as usize];
        for x in 0..self.width {
            let point = self.px2complex(x, y);
            if let Some(i) = escape_time(point) {
                line[x as usize] = (MAXITER - i) as u8
            }
        }
        line
    }

    #[cfg(feature = "parallel")]
    fn render(&self) -> Vec<u8> {
        let mut pxs = vec![0; (self.width * self.height) as usize];
        let (y_tx, y_rx) = bounded(8);
        thread::scope(|sc| {
            let threads: Vec<_> = (0..num_cpus::get())
                .map(|_| {
                    sc.spawn(|_| {
                        let mut lines = Vec::new();
                        for y in &y_rx {
                            lines.push((y, self.render_line(y)));
                        }
                        lines
                    })
                })
                .collect();
            for y in 0..self.height {
                y_tx.send(y).expect("IPC channel broken");
            }
            drop(y_tx);
            for t in threads {
                let res = t.join().expect("thread panic");
                for (y, line) in res {
                    pxs[(y * self.width) as usize..((y + 1) * self.width) as usize]
                        .copy_from_slice(&line);
                }
            }
        })
        .expect("thread panic");
        pxs
    }

    #[cfg(not(feature = "parallel"))]
    fn render(&self) -> Vec<u8> {
        let mut pxs = vec![0; (self.width * self.height) as usize];
        for y in 0..self.height {
            for x in 0..self.width {
                let point = self.px2complex(x, y);
                if let Some(i) = escape_time(point) {
                    pxs[(y * self.width + x) as usize] = (MAXITER - i) as u8;
                }
            }
        }
        pxs
    }
}

fn escape_time(c: Complex<f64>) -> Option<u32> {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAXITER {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

fn write(pxs: &[u8], dimen: &Dimen, filename: &Path) -> Result<(), std::io::Error> {
    let f = std::fs::File::create(filename)?;
    image::png::PNGEncoder::new(f).encode(
        pxs,
        dimen.width as u32,
        dimen.height as u32,
        image::ColorType::Gray(8),
    )
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

    #[test]
    fn coord_trans() {
        let d = Dimen::new(80, 40, Complex::new(-1.0, -1.0), Complex::new(1.0, 1.0));
        assert_eq!(d.px2complex(20, 30), Complex::new(-0.5, -0.5));
    }
}
