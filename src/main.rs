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
    let output_width = 10;
    let output_height = 10;

    let input = image::Image::from_png("simple_input.png");

    let wave_function_info = WaveInfo::init(&input.grid);

    let adjacency_rules =
        adjacency_rules::init(input.width, input.height, &&wave_function_info.pixel_to_id);

    let mut wave_function = WaveFunction::init(
        output_width,
        output_height,
        &adjacency_rules,
        &wave_function_info,
    );

    let state = wave_function.collapse();

    let mut output_bytes = Vec::new();
    for w in 0..output_width {
        for h in 0..output_height {
            let pixel = Pixel { x: w, y: h };
            let color = &state
                .get(&pixel)
                .map(|id| wave_function_info.id_to_color.get(id))
                .flatten()
                .unwrap()
                .0;
            output_bytes.append(&mut color.clone())
        }
    }

    image::output_image(output_width, output_height, &output_bytes.as_slice());
}
