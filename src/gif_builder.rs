use crate::image::Image;
use anyhow::{anyhow, Result};
use gif::{Frame, Repeat};
use std::fs::File;

const FRAME_SPEED: i32 = 1;

pub struct GifBuilder;

impl GifBuilder {
    pub fn make_gif(images: &Vec<Image>) -> Result<()> {
        println!("Generating gif with {} frames.", images.len());

        match &images[..] {
            [first, rest @ ..] => {
                let width = first.width as u16;
                let height = first.height as u16;

                let make_frame =
                    |pixels: Vec<u8>| Frame::from_rgb_speed(width, height, &pixels, FRAME_SPEED);

                // Get pixel data from some source
                let pixels: Vec<u8> = first.pixels.iter().flat_map(|f| f.clone()).collect();

                // Create first frame from data
                let frame = make_frame(pixels);

                // Create encoder
                let mut image = File::create("output.gif")?;
                let mut encoder = gif::Encoder::new(&mut image, frame.width, frame.height, &[])?;
                encoder.set_repeat(Repeat::Infinite)?;
                encoder.write_frame(&frame)?;

                for image in rest {
                    let pixels: Vec<u8> = image.pixels.iter().flat_map(|f| f.clone()).collect();
                    let frame = make_frame(pixels);
                    encoder.write_frame(&frame)?;
                }

                Ok(())
            }
            _ => Err(anyhow!(
                "Incorrect number of images. Must provide at least 2 images"
            )),
        }
    }
}
