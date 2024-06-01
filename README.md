# pixel2svg - rewritten in Rust

"pixel2svg converts pixel art to SVG - pixel by pixel." - Original [pixel2svg](https://florian-berger.de/en/software/pixel2svg/) description.

Helps you avoid weird pixel art upscaling artifacts, mantaining the expected crispy clean image. Useful for web icons.

Some extra features were added compared to the original:

- Stripping unrecommended attributes (according to [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg)) used in the original script;
- Stripping unused XML namespaces (e.g. `xmlns:ev` & `xmlns:xlink`);
- Manually configurable output path;
- Translucent pixels;

## Usage

```
pixel2svg-riir [OPTIONS] <IMAGE_FILEPATH>

Arguments:
  <IMAGE_FILEPATH>

Options:
      --squaresize <SQUARE_SIZE>  Width and height of vector squares in pixels [default: 40]
      --overlap                   If given, overlaps vector squares by 1px
      --strip-namespaces          If given, strips all additional namespaces
      --strip-extra-attrs         If given, strips unrecommended attributes
      --allow-opacity             If given, translucent pixels will be included
  -O, --output <OUTPUT_FILEPATH>  By default, the output file will be the input file replaced with a "svg" extension
  -h, --help                      Print help
  -V, --version                   Print version
```

