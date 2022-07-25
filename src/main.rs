use data::pixel::Pixel;
use model::Model;
use wave_function::WaveFunction;

mod adjacency_rules;
mod data;
mod helpers;
mod image;
mod model;
mod wave_function;

fn main() {
    let output_width = 50;
    let output_height = 10;
    let n = 3;
    let take_snapshots = false;

    // let input_img = format!("input/simple_input.png");
    // let input_img = format!("input/knot.png");
    // let input_img = format!("input/island.png");
    let input_img = format!("input/test.png");
    // let input_img = format!("input/test2.png");
    let input = image::Image::from_png(&input_img);

    let model = Model::init(&input.pixels);

    let adjacency_rules = adjacency_rules::init(input.width, input.height, &&model.pixel_to_id);

    let mut wave_function = WaveFunction::init(
        take_snapshots,
        output_width,
        output_height,
        &adjacency_rules,
        &model,
    );

    let state = wave_function.run();

    let mut data: Vec<u8> = vec![];

    for y in 0..output_height {
        for x in 0..output_width {
            let pixel = Pixel { x, y };
            let id = state[&pixel];
            let color = &model.id_to_color[&id];
            data.append(&mut color.0.clone());
        }
    }

    image::output_image(output_width, output_height, "final", &data)
}
