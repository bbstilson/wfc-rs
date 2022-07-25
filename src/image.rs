use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::OutputInfo;

use crate::data::{color::Color, pixel::Pixel};

pub struct Image {
    pub width: i32,
    pub height: i32,
    pub pixels: HashMap<Pixel, Color>,
}

impl Image {
    pub fn from_png(path: &str) -> Image {
        let (info, buf) = read_image(path);
        let bytes_per_color = match info.color_type {
            png::ColorType::Rgb => 3,
            png::ColorType::Rgba => 4,
            _ => panic!("Unsupported color type"),
        };

        let bytes = buf[..info.buffer_size()].to_vec();
        let mut pixels = HashMap::new();
        let width = info.width as i32;
        let height = info.height as i32;
        for y in 0..height {
            for x in 0..width {
                let idx = get_position(x, y, width, bytes_per_color) as usize;
                let color = &bytes[idx..(idx + 3)].to_owned();
                pixels.insert(Pixel { x, y }, Color(color.clone()));
            }
        }

        Image {
            pixels,
            width,
            height,
        }
    }
}

fn read_image(path: &str) -> (OutputInfo, Vec<u8>) {
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    (info, buf)
}

pub fn output_image(width: i32, height: i32, name: &str, data: &[u8]) {
    let file_name = format!("output/{}.png", name);
    let path = Path::new(&file_name);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();
}

fn get_position(pos_x: i32, pos_y: i32, width: i32, bytes_per_color: i32) -> i32 {
    (pos_y * width + pos_x) * bytes_per_color
}
