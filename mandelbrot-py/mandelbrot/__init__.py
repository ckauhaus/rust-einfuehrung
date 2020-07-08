import argparse
import sys
import png

from .mandelbrot import render


def write(args):
    with open(args.filename, 'wb') as out:
        pixbuf = render(args.width, args.height,
                        args.bottom_left, args.top_right)
        writer = png.Writer(args.width, args.height, greyscale=True)
        writer.write_array(out, pixbuf)


def main():
    a = argparse.ArgumentParser()
    a.add_argument('-W', '--width', metavar='INT', type=int, default=1024,
                   help='Image width in pixels')
    a.add_argument('-H', '--height', metavar='INT', type=int, default=768,
                   help='Image height in pixels')
    a.add_argument('-b', '--bottom-left', metavar='COMPLEX', type=complex,
                   default='-1.2+0.2j',
                   help='Bottom left corner in the complex plane')
    a.add_argument('-t', '--top-right', metavar='COMPLEX', type=complex,
                   default='-1.0+0.35j',
                   help='Top right corner in the complex plane')
    a.add_argument('filename', help='File name of the PNG output file')
    args = a.parse_args()
    try:
        write(args)
    except EnvironmentError as e:
        print(f"Error: Cannot write output file: {e}",
              file=sys.stderr)
        sys.exit(1)
