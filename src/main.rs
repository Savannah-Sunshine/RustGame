use core::num;

use macroquad::prelude::*;
use macroquad_platformer::*;

// Size of the game grid
const SQUARES: i16 = 8;
type Point = (i16, i16);

// const GREENGRASS: Color = Color::from_rgba(173,167,73,255);
// const YELLOWGRASS: Color = Color::from_rgba(201,178,76,255);
// const LIGHTDIRT: Color = Color::from_rgba(199,134,80,255);
const DARKTILE: Color = Color::from_rgba(192, 190, 152, 255);
const LIGHTTILE: Color = Color::from_rgba(214, 200, 163, 255);
const HERO: Color = Color::from_rgba(211, 110, 103, 255);
const ENEMY: Color = Color::from_rgba(151, 38, 61, 255);

struct Character {
    head: Point,
    color: Color,
    dir: Point,
    collider: Actor,
    speed: i32,
}

struct Obstacle {
    collider: Solid,
    speed: i32,
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
        color: HERO,
        // random start position
        head: (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES)),
        dir: (1, 0),
        speed: 1, // initial speed
    };

    let mut enemy = Character {
        collider: world.add_actor(vec2(100.0, 80.0), 8, 8),
        color: ENEMY,
        head: (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES)),
        dir: (1, 0),
        speed: 1, // initial speed
    };

    let mut game_over = false;
    let mut last_update = get_time();
    let mut is_player_turn = true;

    let mut num_losses: f32 = 0.0;
    let mut num_moves_on_turn= 0;
    let up = (0, -1);
    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);

    // returns if move was successful
    fn update_character_position(
        character: &mut Character,
        dir: Point,
        last_update: &mut f64,
        num_moves_on_turn: &mut i32, 
    ) {
        character.dir = dir;
        // Check if the new position is within bounds
        if character.head.0 + dir.0 < 0
            || character.head.0 + dir.0 >= SQUARES
            || character.head.1 + dir.1 < 0
            || character.head.1 + dir.1 >= SQUARES
        {
            // If out of bounds
            return;
        }

        character.head.0 += dir.0;
        character.head.1 += dir.1;
        *last_update = get_time();
        *num_moves_on_turn += 1;
    }

    loop {
        // Does movement
        if !game_over && is_player_turn {
            if is_key_down(KeyCode::Right) {
                update_character_position(&mut player,right,&mut last_update, &mut num_moves_on_turn);
            } else if is_key_down(KeyCode::Left) {
                update_character_position(&mut player, left, &mut last_update, &mut num_moves_on_turn);
            } else if is_key_down(KeyCode::Up) {
                update_character_position(&mut player, up, &mut last_update, &mut num_moves_on_turn);
            } else if is_key_down(KeyCode::Down) {
                update_character_position(&mut player, down, &mut last_update, &mut num_moves_on_turn);
            }
            if num_moves_on_turn >= player.speed {
                is_player_turn = false;
                // Reset moves for next turn
                num_moves_on_turn = 0;
                // last_update = get_time();
            }
            

        } else if !game_over && !is_player_turn {
            // Enemy turn logic can be added here
            // For now, just switch back to player turn

            // pause 1 seconds before switching back to player turn
            if get_time() - last_update > 1.0 {
                // explode if player is next to enemy
                if (enemy.head.0 - player.head.0).abs() <= 1 && (enemy.head.1 - player.head.1).abs() <= 1 {
                    game_over = true;
                    num_losses += 1.0;
                    continue;
                }

                // if player is close, move towards player
                if (enemy.head.0 - player.head.0).abs() <= 5 && (enemy.head.1 - player.head.1).abs() <= 5
                {
                    // Check if player is to the right
                    if enemy.head.0 < player.head.0 {
                        update_character_position(
                            &mut enemy,
                            right,
                            &mut last_update,
                            &mut num_moves_on_turn,
                        );
                    } else if enemy.head.0 > player.head.0 {
                        update_character_position(
                            &mut enemy,
                            left,
                            &mut last_update,
                            &mut num_moves_on_turn,
                        );
                    } else if enemy.head.1 < player.head.1 {
                        update_character_position(
                            &mut enemy,
                            down,
                            &mut last_update,
                            &mut num_moves_on_turn,
                        );
                    } else if enemy.head.1 > player.head.1 {
                        update_character_position(
                            &mut enemy,
                            up,
                            &mut last_update,
                            &mut num_moves_on_turn,
                        );
                    }
                } else {
                    // Random AI: just move in a random direction
                    let directions = [up, down, left, right];
                    let random_index = rand::gen_range(0, directions.len());
                    let dir = directions[random_index];
                    update_character_position(
                        &mut enemy,
                        dir,
                        &mut last_update,
                        &mut num_moves_on_turn,
                    );
                }

                if num_moves_on_turn >= enemy.speed {
                    is_player_turn = true;
                    // Reset moves for next turn
                    num_moves_on_turn = 0;
                    // last_update = get_time();
                }
            }
        }

        // Draws the game
        if !game_over {
            if is_player_turn {
                clear_background(HERO);
            } else {
                clear_background(ENEMY);
            }

            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

            draw_rectangle(
                offset_x,
                offset_y,
                game_size - 20.,
                game_size - 20.,
                LIGHTTILE,
            );

            // draw horizontal lines
            for i in 1..SQUARES {
                draw_line(
                    offset_x,
                    offset_y + sq_size * i as f32,
                    screen_width() - offset_x,
                    offset_y + sq_size * i as f32,
                    2.,
                    DARKTILE,
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
                    DARKTILE,
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

            // Draw enemy
            draw_rectangle(
                offset_x + enemy.head.0 as f32 * sq_size,
                offset_y + enemy.head.1 as f32 * sq_size,
                sq_size,
                sq_size,
                enemy.color,
            );
        } else { //game over screen
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
                println!("Restarting game... You've died {} times", num_losses);
                // Desaturate the player color based on number of losses
                let alpha = (1.0 - num_losses * 0.25).clamp(0.2, 1.0);
                
                player = Character {
                    color: player.color.with_alpha(alpha),
                    collider: player.collider,
                    head: (0, 0),
                    dir: (1, 0),
                    speed: (num_losses as i32 + 1).clamp(1, 3),
                };
                last_update = get_time();
                game_over = false;
                is_player_turn = true;
            }
        }
        next_frame().await
    }
}
