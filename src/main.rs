use std::{
    error::Error,
    ffi::OsStr,
    fs::{File, OpenOptions},
    io::{stdout, Stdout, Write},
    path::PathBuf,
};

use clap::Parser;
use clap_derive::{Parser, ValueEnum};
use svg::{node::element::Rectangle, Document, Node};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    image_filepath: PathBuf,
    /// Width and height of vector squares in pixels.
    #[arg(long = "squaresize", default_value_t = 40)]
    square_size: u32,
    /// If given, overlaps vector squares by 1px.
    #[arg(long)]
    overlap: bool,
    /// If given, strips all additional namespaces.
    #[arg(long)]
    strip_namespaces: bool,
    /// If given, strips unrecommended attributes.
    #[arg(long)]
    strip_extra_attrs: bool,
    /// If given, translucent pixels will be included.
    #[arg(long)]
    allow_opacity: bool,
    /// By default, the output filepath will be the input filepath replaced with a "svg" extension.
    /// If '-' is given, outputs results to standart output.
    #[arg(short = 'O', long = "output")]
    output_filepath: Option<PathBuf>,
    /// If given, determines the fill color format.
    #[arg(value_enum, short = 'C', long, default_value_t = ColorFormat::RgbFunction)]
    color_format: ColorFormat,
}

#[derive(Clone, Copy, ValueEnum)]
enum ColorFormat {
    /// Colors are outputted as `rgb()` CSS functions.
    RgbFunction,
    /// Colors are outputted as RRGGBB hex codes.
    RgbHex,
}

impl ColorFormat {
    #[inline]
    fn fmt(&self, color: &[u8]) -> String {
        match self {
            ColorFormat::RgbFunction => format!("rgb({},{},{})", color[0], color[1], color[2]),
            ColorFormat::RgbHex => format!("#{:02x}{:02x}{:02x}", color[0], color[1], color[2]),
        }
    }
}

enum OutputFile {
    File(File),
    Stdout(Stdout),
}

impl Write for OutputFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            OutputFile::File(f) => f.write(buf),
            OutputFile::Stdout(stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            OutputFile::File(f) => f.flush(),
            OutputFile::Stdout(stdout) => stdout.flush(),
        }
    }
}

impl OutputFile {
    fn new(path: PathBuf) -> std::io::Result<Self> {
        if path.as_os_str() == OsStr::new("-") {
            Ok(OutputFile::Stdout(stdout()))
        } else {
            Ok(OutputFile::File(
                OpenOptions::new().create_new(true).open(path)?,
            ))
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        image_filepath,
        square_size,
        overlap,
        strip_namespaces,
        strip_extra_attrs,
        allow_opacity,
        output_filepath,
        color_format,
    } = Args::parse();
    let overlap = if overlap { 1 } else { 0 };
    let rect_size = to_px(square_size + overlap);

    let output_filepath = output_filepath.unwrap_or_else(|| image_filepath.with_extension("svg"));

    let image_file = image::open(image_filepath)?;
    let data = image_file.to_rgba8();

    let mut document = Document::new()
        .set("width", to_px(data.width() * square_size))
        .set("height", to_px(data.height() * square_size));
    if !strip_namespaces {
        document.assign("xmlns:ev", "http://www.w3.org/2001/xml-events");
        document.assign("xmlns:xlink", "http://www.w3.org/1999/xlink");
    }
    if !strip_extra_attrs {
        document.assign("baseProfile", "full");
        document.assign("version", "1.1");
    }

    let rectangles = data.enumerate_pixels().filter(|(_, _, c)| {
        if allow_opacity {
            c[3] > 0
        } else {
            c[3] >= 0xFF
        }
    });

    for (x, y, c) in rectangles {
        let mut rectangle = Rectangle::new()
            .set("x", to_px(x * square_size))
            .set("y", to_px(y * square_size))
            .set("width", rect_size.clone())
            .set("height", rect_size.clone())
            .set("fill", color_format.fmt(&c.0));
        if c[3] < 0xFF {
            rectangle.assign("opacity", (c[3] as f32) / 255.0);
        }
        document.append(rectangle);
    }
    svg::write(OutputFile::new(output_filepath)?, &document)?;
    Ok(())
}

#[inline]
fn to_px(length: u32) -> String {
    format!("{}px", length)
}
