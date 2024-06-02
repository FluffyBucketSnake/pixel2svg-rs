# pixel2svg - rewritten in Rust

"pixel2svg converts pixel art to SVG - pixel by pixel." - Original [pixel2svg](https://florian-berger.de/en/software/pixel2svg/) description.

Helps you avoid weird pixel art upscaling artifacts, mantaining the expected crispy clean image. Useful for web icons.

Some extra features were added compared to the original:

- Stripping unrecommended attributes (according to [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/svg)) used in the original script;
- Stripping unused XML namespaces (e.g. `xmlns:ev` & `xmlns:xlink`);
- Manually configurable output path;
- Translucent pixels;
- Printing results to stardart output;
- Customizable color formats.

## Dependencies

- `clap` (& `clap-derive`);
- `image`;
- `svg`.

## Installation

```bash
cargo install svg2pix

```

## Usage

```
pixel2svg [OPTIONS] <IMAGE_FILEPATH>

Arguments:
  <IMAGE_FILEPATH>


Options:
      --squaresize <SQUARE_SIZE>
          Width and height of vector squares in pixels

          [default: 40]

      --overlap
          If given, overlaps vector squares by 1px

      --strip-namespaces
          If given, strips all additional namespaces

      --strip-extra-attrs
          If given, strips unrecommended attributes

      --allow-opacity
          If given, translucent pixels will be included

  -O, --output <OUTPUT_FILEPATH>
          By default, the output filepath will be the input filepath replaced with a "svg" extension. If '-' is given, outputs results to standart output

  -C, --color-format <COLOR_FORMAT>
          Sets the outputted color format

          [default: rgb-function]

          Possible values:
          - rgb-function: Colors are outputted as `rgb()` CSS functions
          - rgb-hex:      Colors are outputted as RRGGBB hex codes

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

