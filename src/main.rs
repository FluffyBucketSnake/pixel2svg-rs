use std::{error::Error, path::PathBuf};

use clap::Parser;
use clap_derive::Parser;
use svg::{node::element::Rectangle, Document};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    image_filepath: PathBuf,
    /// Width and heigt of vector squares in pixels
    #[arg(long = "squaresize", default_value_t = 40)]
    square_size: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        image_filepath,
        square_size,
    } = Args::parse();

    let output_filepath = image_filepath.with_extension("svg");

    let image_file = image::open(image_filepath)?;
    let data = image_file.to_rgba8();

    let mut document = Document::new().set(
        "viewport",
        (
            0,
            0,
            data.width() * square_size,
            data.height() * square_size,
        ),
    );
    let rectangles = data
        .enumerate_pixels()
        .filter(|(_, _, c)| c.0[3] > 0)
        .map(|(x, y, c)| {
            Rectangle::new()
                .set("x", to_px(x * square_size))
                .set("y", to_px(y * square_size))
                .set("width", to_px(square_size))
                .set("height", to_px(square_size))
                .set("fill", to_rgb_hex(&c.0))
                .into()
        });
    document.get_children_mut().extend(rectangles);
    svg::save(output_filepath, &document)?;
    Ok(())
}

#[inline]
fn to_px(length: u32) -> String {
    format!("{}px", length)
}

#[inline]
fn to_rgb_hex(rgb: &[u8]) -> String {
    format!("#{:x}{:x}{:x}", rgb[0], rgb[1], rgb[2])
}
