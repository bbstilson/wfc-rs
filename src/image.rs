use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, path::PathBuf};

use anyhow::{Ok, Result};
use png::OutputInfo;

use crate::data::{
    color::{self, RGB},
    vector2::Vector2,
};

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<RGB>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            pixels: vec![color::BLACK; width as usize * height as usize],
        }
    }

    pub fn set_color(&mut self, at: Vector2, color: RGB) {
        let idx = self.get_idx(at);
        self.pixels[idx] = color;
    }

    pub fn at(&self, at: Vector2) -> RGB {
        let idx = self.get_idx(at);
        self.pixels[idx]
    }

    fn get_idx(&self, at: Vector2) -> usize {
        ((at.y * self.width as i32) + at.x) as usize
    }

    pub fn from_png(path: PathBuf) -> Result<Image> {
        let (info, buf) = read_image(path)?;
        let bytes_per_color = match info.color_type {
            png::ColorType::Rgb => 3,
            png::ColorType::Rgba => 4,
            _ => panic!("Unsupported color type"),
        };

        let bytes = buf[..info.buffer_size()].to_vec();
        let mut pixels = vec![];
        let width = info.width;
        let height = info.height;
        for y in 0..height {
            for x in 0..width {
                let idx = get_position(x, y, width, bytes_per_color) as usize;
                let mut color: RGB = [0; 3];
                for i in 0..3 {
                    color[i] = bytes[idx + i];
                }
                pixels.push(color);
            }
        }

        Ok(Image {
            pixels,
            width,
            height,
        })
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let file_name = format!("{}.png", path);
        let path = Path::new(&file_name);
        let file = File::create(path)?;
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        let mut writer = encoder.write_header()?;

        let buf: Vec<u8> = self.pixels.iter().flat_map(|c| c.clone()).collect();
        writer.write_image_data(&buf.as_slice())?;

        Ok(())
    }
}

fn read_image(path: PathBuf) -> Result<(OutputInfo, Vec<u8>)> {
    let decoder = png::Decoder::new(File::open(path)?);
    let mut reader = decoder.read_info()?;
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf)?;
    Ok((info, buf))
}

fn get_position(pos_x: u32, pos_y: u32, width: u32, bytes_per_color: u32) -> usize {
    ((pos_y * width + pos_x) * bytes_per_color) as usize
}
