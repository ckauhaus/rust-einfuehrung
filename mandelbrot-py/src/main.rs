use gumdrop::Options;
use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

mod mandelbrot;

fn write(opt: &Opt) -> Result<(), Box<dyn Error>> {
    let f = File::create(&opt.filename)?;
    let pxs = mandelbrot::render(opt.width, opt.height, opt.bottom_left, opt.top_right);
    PNGEncoder::new(f).encode(&pxs, opt.width as u32, opt.height as u32, ColorType::L8)?;
    Ok(())
}

/// Mandelbrot set generator
#[derive(Options, Debug)]
struct Opt {
    /// image width ins pixels
    #[options(short = 'W', meta = "INT", default = "1024")]
    width: usize,
    /// image height in pixels
    #[options(short = 'H', meta = "INT", default = "768")]
    height: usize,
    /// bottom left corner in the complex plane
    #[options(meta = "COMPLEX", default = "-1.2+0.2i")]
    bottom_left: Complex<f64>,
    /// top right corner in the complex plane
    #[options(meta = "COMPLEX", default = "-1.0+0.35i")]
    top_right: Complex<f64>,
    /// show command line usage
    help: bool,
    /// path to the PNG output file
    #[options(free, required)]
    filename: PathBuf,
}

fn main() {
    let opt = Opt::parse_args_default_or_exit();
    if let Err(e) = write(&opt) {
        eprintln!(
            "Error: Cannot write output to '{}': {}",
            opt.filename.display(),
            e
        );
        std::process::exit(1);
    }
}
