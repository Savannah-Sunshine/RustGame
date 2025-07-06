use macroquad::prelude::*;
use macroquad_platformer::*;

// Size of the game grid
const SQUARES: i16 = 16;
type Point = (i16, i16);

struct Character {
    head: Point,
    color: Color,
    dir: Point,
    collider: Actor,
}

struct Obstacle {
    collider: Solid,
    speed: f32,
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut world = World::new();
    // Load a Tiled map?

    // let mut static_colliders = vec![];
    // for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
    //     static_colliders.push(if tile.is_some() {
    //         Tile::Solid
    //     } else {
    //         Tile::Empty
    //     });
    // }
    // world.add_static_tiled_layer(static_colliders, 8., 8., 40, 1);

    let mut player = Character {
        collider: world.add_actor(vec2(50.0, 80.0), 8, 8),
        color: BLUE,
        head: (0, 0),
        dir: (1, 0),
    };

    let mut enemy = Character {
        collider: world.add_actor(vec2(100.0, 80.0), 8, 8),
        color: DARKGREEN,
        head: (1, 0),
        dir: (1, 0),
    };

    let mut game_over = false;
    let mut last_update = get_time();
    let mut is_player_turn = true;

    let up = (0, -1);
    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);
    fn update_character_position(
        character: &mut Character,
        dir: Point,
        is_player_turn: &mut bool,
        last_update: &mut f64,
    ) {
        character.dir = dir;
        character.head.0 += dir.0;
        character.head.1 += dir.1;
        *is_player_turn = !*is_player_turn;
        *last_update = get_time();
    }

    loop {
        // Does movement
        if !game_over && is_player_turn {
            if is_key_down(KeyCode::Right) {
                update_character_position(&mut player, right, &mut is_player_turn, &mut last_update);
            } else if is_key_down(KeyCode::Left) {
                update_character_position(&mut player, left, &mut is_player_turn, &mut last_update);
            } else if is_key_down(KeyCode::Up) {
                update_character_position(&mut player, up, &mut is_player_turn, &mut last_update);
            } else if is_key_down(KeyCode::Down) {
                update_character_position(&mut player, down, &mut is_player_turn, &mut last_update);
            }
        } else if !game_over && !is_player_turn {
            // Enemy turn logic can be added here
            // For now, just switch back to player turn

            // pause 1 seconds before switching back to player turn
            if get_time() - last_update > 1.0 {
                // Random AI: just move in a random direction
                let directions = [up, down, left, right];
                let random_index = rand::gen_range(0, directions.len());
                let dir = directions[random_index];
                update_character_position(&mut enemy, dir, &mut is_player_turn, &mut last_update);
            }
            // if is_key_down(KeyCode::Right) {
            //     update_character_position(&mut enemy, right, &mut is_player_turn, &mut last_update);
            // } else if is_key_down(KeyCode::Left) {
            //     update_character_position(&mut enemy, left, &mut is_player_turn, &mut last_update);
            // } else if is_key_down(KeyCode::Up) {
            //     update_character_position(&mut enemy, up, &mut is_player_turn, &mut last_update);
            // } else if is_key_down(KeyCode::Down) {
            //     update_character_position(&mut enemy, down, &mut is_player_turn, &mut last_update);
            // }
        }

        // Draws the game
        if !game_over {
            if is_player_turn {
                clear_background(BLUE);
            } else {
                clear_background(DARKGREEN);
            }

            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

            // draw horizontal lines
            for i in 1..SQUARES {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    LIGHTGRAY,
                );
            }

            // draw vertical lines
            for i in 1..SQUARES {
                draw_line(
                    offset_x + sq_size * i as f32,
                    offset_y,
                    offset_x + sq_size * i as f32,
                    screen_height() - offset_y,
                    2.,
                    LIGHTGRAY,
                );
            }

            // // Draw player
            // {
            //     // sprite id from tiled
            //     const PLAYER_SPRITE: u32 = 120;

            //     // let pos = world.actor_pos(player.collider);
            //     if player.speed.x >= 0.0 {
            //         tiled_map.spr("tileset", PLAYER_SPRITE, Rect::new(pos.x, pos.y, 8.0, 8.0));
            //     } else {
            //         tiled_map.spr(
            //             "tileset",
            //             PLAYER_SPRITE,
            //             Rect::new(pos.x + 8.0, pos.y, -8.0, 8.0),
            //         );
            //     }
            // }
            draw_rectangle(
                offset_x + player.head.0 as f32 * sq_size,
                offset_y + player.head.1 as f32 * sq_size,
                sq_size,
                sq_size,
                player.color,
            );

            draw_rectangle(
                offset_x + enemy.head.0 as f32 * sq_size,
                offset_y + enemy.head.1 as f32 * sq_size,
                sq_size,
                sq_size,
                enemy.color,
            );

            draw_text(
                format!("SCORE: {is_player_turn}").as_str(),
                10.,
                20.,
                20.,
                WHITE,
            );
        } else {
            clear_background(WHITE);
            let text = "Game Over. Press [enter] to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                player = Character {
                    color: DARKGREEN,
                    collider: world.add_actor(vec2(50.0, 80.0), 8, 8),
                    head: (0, 0),
                    dir: (1, 0),
                };
                last_update = get_time();
                game_over = false;
            }
        }
        next_frame().await
    }
}
