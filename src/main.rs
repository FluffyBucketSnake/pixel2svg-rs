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
    /// If given, overlap vector squares by 1px
    #[arg(long)]
    overlap: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args {
        image_filepath,
        square_size,
        overlap,
    } = Args::parse();
    let overlap = if overlap { 1 } else { 0 };
    let rect_size = to_px(square_size + overlap);

    let output_filepath = image_filepath.with_extension("svg");

    let image_file = image::open(image_filepath)?;
    let data = image_file.to_rgba8();

    let mut document = Document::new()
        .set("width", to_px(data.width() * square_size))
        .set("height", to_px(data.height() * square_size));
    let rectangles = data
        .enumerate_pixels()
        .filter(|(_, _, c)| c.0[3] > 0)
        .map(|(x, y, c)| {
            Rectangle::new()
                .set("x", to_px(x * square_size))
                .set("y", to_px(y * square_size))
                .set("width", rect_size.clone())
                .set("height", rect_size.clone())
                .set("fill", to_fill(&c.0))
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
fn to_fill(rgb: &[u8]) -> String {
    format!("rgb({},{},{})", rgb[0], rgb[1], rgb[2])
}
