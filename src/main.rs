use macroquad::prelude::*;
use macroquad_platformer::{Actor, Tile, World};

// Source d'inspiration https://github.com/not-fl3/macroquad/blob/master/examples/platformer.rs

type Stage = Vec<[Tile; 16]>;

enum DuelError {
    PlayerInTheGround,
}

const BLOCK_SIZE: f32 = 16.;
const DELTA: f32 = 1.;
const PLAYER_WIDTH: f32 = 10.;
const PLAYER_HEIGHT: f32 = 15.;

// TODO player
// TODO Weapon
// TODO exit

fn init_stage() -> Stage {
    use Tile::*;
    vec![
        [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty,
        ],
        [
            Empty, Solid, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty,
        ],
        [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty,
        ],
        [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty,
        ],
        [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty,
        ],
        [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty,
        ],
        [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty,
        ],
        [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty,
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
            Solid, Solid, Empty, Solid,
        ],
        [
            Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid, Solid,
            Solid, Solid, Solid, Solid,
        ],
    ]
}

struct WorldScreen {
    screen_width: f32,
    screen_height: f32,
}

struct Player {
    collider: Actor,
    speed: Vec2,
}

impl WorldScreen {
    fn new() -> WorldScreen {
        WorldScreen {
            screen_width: screen_width(),
            screen_height: screen_height(),
        }
    }
}

fn draw_stage(stage: &Stage) {
    for (y, line) in stage.iter().enumerate() {
        for (x, block) in line.iter().enumerate() {
            draw_rectangle(
                x as f32 * BLOCK_SIZE,
                y as f32 * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
                {
                    match block {
                        Tile::Empty => BLUE,
                        Tile::Solid => BROWN,
                        Tile::JumpThrough => todo!(),
                        Tile::Collider => todo!(),
                    }
                },
            );
        }
    }
}

fn draw_mire(world: &WorldScreen, camera: &Camera2D) {
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

fn draw_player(player: &Player, world: &World) {
    let pos = world.actor_pos(player.collider);
    draw_rectangle_lines(pos.x, pos.y, PLAYER_WIDTH, PLAYER_HEIGHT, 1., GREEN)
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
    let mut world = World::new();

    let mut player = Player {
        collider: world.add_actor(vec2(50.0, 80.0), 8, 8),
        speed: vec2(0., 0.),
    };
    loop {
        clear_background(LIGHTGRAY);
        let world_screen = WorldScreen::new();

        world.add_static_tiled_layer(
            stage.iter().cloned().flatten().collect(),
            BLOCK_SIZE,
            BLOCK_SIZE,
            16,
            1,
        );
        draw_stage(&stage);
        draw_mire(&world_screen, &camera);
        draw_player(&player, &world);

        // player movement control
        {
            let pos = world.actor_pos(player.collider);
            let on_ground = world.collide_check(player.collider, pos + vec2(0., 1.));

            if !on_ground {
                player.speed.y += 500. * get_frame_time();
            }

            if is_key_down(KeyCode::Right) {
                player.speed.x = 100.0;
            } else if is_key_down(KeyCode::Left) {
                player.speed.x = -100.0;
            } else {
                player.speed.x = 0.;
            }

            if is_key_pressed(KeyCode::Space) && on_ground {
                player.speed.y = -120.;
            }

            world.move_h(player.collider, player.speed.x * get_frame_time());
            world.move_v(player.collider, player.speed.y * get_frame_time());
        }
        next_frame().await
    }
}
