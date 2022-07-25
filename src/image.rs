use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::OutputInfo;

use crate::color::Color;
use crate::color_type;
use crate::grid::Grid;
use crate::helpers;
use crate::pixel::Pixel;

pub struct Image {
    pub width: i32,
    pub height: i32,
    pub color_type: color_type::ColorType,
    pub grid: Grid,
}

impl Image {
    pub fn from_png(path: &str) -> Image {
        let (info, buf) = read_image(path);
        let color_type = match info.color_type {
            png::ColorType::Rgb => color_type::ColorType::RGB,
            png::ColorType::Rgba => color_type::ColorType::RGBA,
            _ => panic!("unsupported color type"),
        };

        // build grid
        let bytes_per_color = color_type.bytes_per_color();
        let bytes = buf[..info.buffer_size()].to_vec();
        let mut grid = Grid::new();
        for y in 0..(info.height as i32) {
            for x in 0..(info.width as i32) {
                let idx = helpers::get_position(x, y, info.width as i32, bytes_per_color) as usize;
                let color = &bytes[idx..(idx + 3)].to_owned();
                grid.insert(Pixel { x, y }, Color(color.clone()));
            }
        }

        Image {
            width: info.width as i32,
            height: info.height as i32,
            color_type: color_type,
            grid: grid,
        }
    }

    // pub fn get_bytes(&self) -> Vec<u8> {
    //     let mut bytes = Vec::new();
    //     for w in 0..self.width {
    //         for h in 0..self.height {
    //             let pixel = Pixel { x: w, y: h };
    //             let mut color = self.grid.get(pixel).unwrap().0.clone();
    //             bytes.append(&mut color);
    //         }
    //     }
    //     bytes
    // }
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
