use std::{error::Error, path::PathBuf};

use clap::Parser;
use clap_derive::Parser;
use svg::{node::element::Rectangle, Document, Node};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    image_filepath: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { image_filepath } = Args::parse();

    let square_size = 1;
    let output_filepath = image_filepath.with_extension("svg");

    let image_file = image::open(image_filepath)?;
    let data = image_file.to_rgba8();

    let mut document = Document::new().set("viewport", (0, 0, data.width(), data.height()));
    for (x, y, c) in data.enumerate_pixels() {
        if c.0[3] == 0 {
            continue;
        }

        document.append(
            Rectangle::new()
                .set("x", format!("{}px", x))
                .set("y", format!("{}px", y))
                .set("width", format!("{}px", square_size))
                .set("height", format!("{}px", square_size))
                .set("fill", format!("#{:x}{:x}{:x}", c.0[0], c.0[1], c.0[2])),
        );
    }
    svg::save(output_filepath, &document)?;
    Ok(())
}
