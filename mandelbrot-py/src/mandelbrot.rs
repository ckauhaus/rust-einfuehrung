use num::Complex;

type C = Complex<f64>;

/// Mandelbrot iteration, see https://en.wikipedia.org/wiki/Mandelbrot_set
fn escape_time(c: C) -> Option<u8> {
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
fn test_should_escape() {
    assert_eq!(escape_time(Complex::new(0.5, 0.0)), Some(4));
    assert_eq!(escape_time(Complex::new(2.0, -0.1)), Some(0));
    assert_eq!(escape_time(Complex::new(-1.0, -0.4)), Some(6));
}

#[test]
fn test_should_not_escape() {
    assert_eq!(escape_time(Complex::default()), None);
    assert_eq!(escape_time(Complex::new(-2.0, 0.0)), None);
    assert_eq!(escape_time(Complex::new(0.0, 0.5)), None);
}

/// Image dimensions both in pixels and in the complex plane
#[derive(Debug)]
pub struct Dimen {
    pub width: usize,
    pub height: usize,
    origin: C,
    window: C,
}

impl Dimen {
    /// Computes window dimensions from bottom right and top left point on the complex plane.
    pub fn new(width: usize, height: usize, bot_left: C, top_right: C) -> Self {
        Self {
            width,
            height,
            origin: bot_left,
            window: top_right - bot_left,
        }
    }

    /// Coordinate transformation from image pixels into the complex plane.
    fn px2complex(&self, x: usize, y: usize) -> C {
        Complex::new(
            self.origin.re + x as f64 * self.window.re / self.width as f64,
            self.origin.im + (self.height - y) as f64 * self.window.im / self.height as f64,
        )
    }
}

#[test]
fn test_px2complex() {
    let d = Dimen::new(80, 40, Complex::new(-1.0, -1.0), Complex::new(1.0, 1.0));
    assert_eq!(d.px2complex(20, 30), Complex::new(-0.5, -0.5));
}

fn render_line(line: &mut [u8], d: &Dimen, y: usize) {
    for (x, px) in line.iter_mut().enumerate() {
        let point = d.px2complex(x, y);
        *px = match escape_time(point) {
            Some(i) => u8::MAX - i,
            None => 0,
        }
    }
}

/// Renders Mandelbrot set as grayscale image. Result is a collection of luminance values in row
/// major order.
pub fn render(width: usize, height: usize, bot_left: C, top_right: C) -> Vec<u8> {
    let dimen = Dimen::new(width, height, bot_left, top_right);
    let mut buf = vec![0; width * height];
    for y in 0..height {
        let offset = y * width;
        render_line(&mut buf[offset..(offset + width)], &dimen, y)
    }
    buf
}
