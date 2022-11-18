use macroquad::prelude::*;

type Stage = Vec<[Block; 16]>;
enum Block {
    Air,
    Solid,
    //Liquid,
    Debug,
}

const BLOCK_SIZE: f32 = 16.;
const PLAYER_WIDTH: f32 = 1.;
const PLAYER_HEIGHT: f32 = 15.;

// TODO player
// TODO Weapon
// TODO exit

fn init_stage() -> Stage {
    use Block::*;
    vec![
        [
            Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air,
        ],
        [
            Air, Solid, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air,
        ],
        [
            Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air,
        ],
        [
            Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air,
        ],
        [
            Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air,
        ],
        [
            Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air,
        ],
        [
            Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air,
        ],
        [
            Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air, Air,
        ],
        [
            Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid,
            Solid, Solid, Solid, Solid,
        ],
        [
            Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid,
            Solid, Solid, Solid, Solid,
        ],
        [
            Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid,
            Solid, Solid, Solid, Solid,
        ],
        [
            Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid,
            Solid, Solid, Solid, Solid,
        ],
        [
            Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid,
            Solid, Solid, Solid, Solid,
        ],
        [
            Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid,
            Solid, Solid, Solid, Solid,
        ],
        [
            Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid,
            Solid, Solid, Air, Solid,
        ],
        [
            Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid,
            Solid, Solid, Solid, Solid,
        ],
    ]
}

struct World {
    screen_width: f32,
    screen_height: f32,
}

struct Player {
    x: f32,
    y: f32,
}

impl World {
    fn new() -> World {
        World {
            screen_width: screen_width(),
            screen_height: screen_height(),
        }
    }
}

fn draw_stage(stage: &Stage) {
    for (y, line) in stage.iter().enumerate() {
        for (x, block) in line.iter().enumerate() {
            draw_rectangle(
                (x as f32 * BLOCK_SIZE) as f32,
                (y as f32 * BLOCK_SIZE) as f32,
                BLOCK_SIZE,
                BLOCK_SIZE,
                {
                    match block {
                        Block::Air => BLUE,
                        Block::Solid => BROWN,
                        Block::Debug => RED,
                    }
                },
            );
        }
    }
}

fn draw_mire(world: &World, camera: &Camera2D) {
    let middle_height = world.screen_height / 2.;
    let middle_width = world.screen_width / 2.;

    let Vec2 {
        x: middle_width_w,
        y: middle_height_w,
    } = camera.screen_to_world(Vec2::from_array([middle_width, middle_height]));

    draw_line(
        middle_height_w + BLOCK_SIZE,
        middle_width_w,
        middle_height_w - BLOCK_SIZE,
        middle_width_w,
        1.,
        RED,
    );

    draw_line(
        middle_height_w,
        middle_width_w + BLOCK_SIZE,
        middle_height_w,
        middle_width_w - BLOCK_SIZE,
        1.,
        RED,
    );
}

#[macroquad::main("Duel")]
async fn main() {
    let stage = init_stage();
    let camera = Camera2D::from_display_rect(Rect::new(
        0.,
        0.,
        BLOCK_SIZE * stage[0].len() as f32,
        BLOCK_SIZE * stage.len() as f32,
    ));
    set_camera(&camera);
    loop {
        clear_background(LIGHTGRAY);
        let world = World::new();

        draw_stage(&stage);
        draw_mire(&world, &camera);
        next_frame().await
    }
}
