use adjacency_rules::AdjacencyRules;
use data::coord_2d::Coord2d;
use model::Model;
use wave_function::WaveFunction;

mod adjacency_rules;
mod data;
mod helpers;
mod image;
mod model;
mod wave_function;

// TODO:
// 1) overlap or tiled
// 2) snapshots for gif
// 3) reflections and rotations of tiles

fn main() {
    let grid_width = 30;
    let grid_height = 15;
    let tile_size = 3;

    // let input_img = "input/simple_input.png";
    let input_img = "input/island.png";
    // let input_img = "input/test.png";
    // let input_img = "input/knot.png";
    // let input_img = "input/house.png";
    let input = image::Image::from_png(input_img);

    let model = Model::new(tile_size, &input);
    let id_to_tile = model.id_to_tile.clone();
    let adjacency_rules = AdjacencyRules::from_model(&model);

    let mut wave_function = WaveFunction::new(grid_width, grid_height, adjacency_rules, model);

    let state = wave_function.run();

    let mut data =
        vec![vec![128; grid_width as usize * 3 * tile_size]; grid_height as usize * tile_size];

    for g_y in 0..grid_height as usize {
        for g_x in 0..grid_width as usize {
            let pixel = Coord2d {
                x: g_x as i32,
                y: g_y as i32,
            };
            let id = state[&pixel];
            let tile = &id_to_tile[&id];

            for n_y in 0..tile_size {
                for n_x in 0..tile_size {
                    let color = &tile.pixels[n_y][n_x];
                    let f_x = (g_x * tile_size * 3) + (n_x * 3);
                    let f_y = (g_y * tile_size) + n_y;
                    data[f_y][f_x] = color.0[0];
                    data[f_y][f_x + 1] = color.0[1];
                    data[f_y][f_x + 2] = color.0[2];
                }
            }
        }
    }

    image::output_image("final", data);
}
