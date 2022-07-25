use pixel::Pixel;
use wave_function::WaveFunction;
use wave_info::WaveInfo;

mod adjacency_rules;
mod color;
mod color_type;
mod direction;
mod grid;
mod helpers;
mod id;
mod image;
mod pixel;
mod wave_function;
mod wave_info;

fn main() {
    let output_width = 50;
    let output_height = 10;

    // let input_img = format!("input/simple_input.png");
    // let input_img = format!("input/knot.png");
    // let input_img = format!("input/island.png");
    let input_img = format!("input/test.png");
    // let input_img = format!("input/test2.png");
    let input = image::Image::from_png(&input_img);

    let wave_function_info = WaveInfo::init(&input.grid);

    let adjacency_rules =
        adjacency_rules::init(input.width, input.height, &&wave_function_info.pixel_to_id);

    let mut wave_function = WaveFunction::init(
        output_width,
        output_height,
        &adjacency_rules,
        &wave_function_info,
    );

    let state = wave_function.run();

    let mut data: Vec<u8> = vec![];

    for y in 0..output_height {
        for x in 0..output_width {
            let pixel = Pixel { x, y };
            let id = state[&pixel];
            let color = &wave_function_info.id_to_color[&id];
            data.append(&mut color.0.clone());
        }
    }

    image::output_image(output_width, output_height, "final", &data)
}
