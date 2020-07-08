from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name='mandelbrot',
    version="0.1",
    rust_extensions=[
        RustExtension(
            'mandelbrot.mandelbrot', binding=Binding.PyO3, debug=False,
            features=['py'])
    ],
    packages=["mandelbrot"],
    install_requires=[
        'pypng>=0.0.20',
    ],
    entry_points={
        'console_scripts': ['mandelbrot_py=mandelbrot:main'],
    },
    zip_safe=False,
)
