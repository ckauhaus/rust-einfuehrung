#![cfg(feature = "py")]
use num::Complex;
use pyo3::prelude::*;

mod mandelbrot;

/// Simple mandelbrot set generator
#[pymodule]
pub fn mandelbrot(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "render")]
    pub fn render(
        _py: Python,
        width: usize,
        height: usize,
        bot_left: Complex<f64>,
        top_right: Complex<f64>,
    ) -> Vec<u8> {
        mandelbrot::render(width, height, bot_left, top_right)
    }

    Ok(())
}
