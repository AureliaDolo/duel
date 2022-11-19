use macroquad::prelude::*;
use macroquad_platformer::{Actor, Tile, World};

// Source d'inspiration https://github.com/not-fl3/macroquad/blob/master/examples/platformer.rs

/// Alias de type: https://doc.rust-lang.org/reference/items/type-aliases.html
type Stage = Vec<[Tile; 16]>;

/// Constantes pour les tailles des elements
const BLOCK_SIZE: f32 = 16.;
const PLAYER_WIDTH: f32 = 10.;
const PLAYER_HEIGHT: f32 = 15.;

/// Initialisation de la carte
/// TODO choisir un format de fichier et le parser
/// TODO ajouter des plateformes
/// TODO ajouter plusieurs niveaux avec un début et une fin
fn init_stage() -> Stage {
    use Tile::*;
    vec![
        [
            Solid, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Solid,
        ],
        [
            Solid, Solid, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Solid,
        ],
        [
            Solid, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Solid,
        ],
        [
            Solid, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Solid,
        ],
        [
            Solid, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Solid,
        ],
        [
            Solid, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Solid,
        ],
        [
            Solid, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Solid,
        ],
        [
            Solid, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Solid,
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

/// Le joueur
struct Player {
    /// index du joueur dans le monde
    collider: Actor,
    /// vitesse dans l'axe x et l'axe y
    speed: Vec2,
}

/// dessine le decor
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

/// Dissine une mire au milieu de ce que voit la camera
fn draw_mire(camera: &Camera2D) {
    let middle_height = screen_height() / 2.;
    let middle_width = screen_width() / 2.;

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

/// dessiner le joueur
/// TODO utiliser un/des sprites
fn draw_player(player: &Player, world: &World) {
    let pos = world.actor_pos(player.collider);
    draw_rectangle_lines(pos.x, pos.y, PLAYER_WIDTH, PLAYER_HEIGHT, 1., GREEN)
}

// Une proc macro https://doc.rust-lang.org/reference/procedural-macros.html
// qui permet de générer le code du moteur de jeu
#[macroquad::main("Duel")]
async fn main() {
    let stage = init_stage();

    // Utiliser une camera permet d'éviter d'avoir à s'occuper des transformation
    // coordonnée de l'éran vs coordonnées du monde
    // TODO utiliser une camera 3D pour pourvoir gérer plusieurs plans qui défilent
    let camera = Camera2D::from_display_rect(Rect::new(
        0.,
        0.,
        BLOCK_SIZE * stage[0].len() as f32,
        BLOCK_SIZE * stage.len() as f32,
    ));
    set_camera(&camera);
    let mut world = World::new();

    let mut player = Player {
        collider: world.add_actor(vec2(50.0, 80.0), PLAYER_WIDTH as i32, PLAYER_HEIGHT as i32),
        speed: vec2(0., 0.),
    };

    // fin de l'initialisation
    // début de la boucle de jeu
    let static_collider = stage.iter().cloned().flatten().collect();
    world.add_static_tiled_layer(static_collider, BLOCK_SIZE, BLOCK_SIZE, stage.len(), 1);

    loop {
        clear_background(LIGHTGRAY);
        draw_stage(&stage);
        draw_mire(&camera);
        draw_player(&player, &world);

        // player movement control
        {
            let pos = world.actor_pos(player.collider);
            let on_ground = world.collide_check(player.collider, pos + vec2(0., 1.));

            if !on_ground {
                // TODO expérimenter en changeant cette valeur
                // Quelle mécanique change ?
                player.speed.y += 500. * get_frame_time();
            }

            if is_key_down(KeyCode::Right) {
                // TODO expérimenter en changeant cette valeur
                // Quelle mécanique change ?
                player.speed.x = 100.0;
            } else if is_key_down(KeyCode::Left) {
                // TODO expérimenter en changeant cette valeur
                // Quelle mécanique change ?
                player.speed.x = -100.0;
            } else {
                player.speed.x = 0.;
            }

            if is_key_pressed(KeyCode::Space) && on_ground {
                // TODO expérimenter en changeant cette valeur
                // Quelle mécanique change ?
                player.speed.y = -200.;
            }

            world.move_h(player.collider, player.speed.x * get_frame_time());
            world.move_v(player.collider, player.speed.y * get_frame_time());
        }
        next_frame().await
    }
}
