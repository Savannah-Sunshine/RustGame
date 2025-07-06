use macroquad::prelude::*;

// Size of the game grid
const SQUARES: i16 = 16;
type Point = (i16, i16);

struct Snake {
    head: Point,
    // body: LinkedList<Point>,
    dir: Point,
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut snake = Snake {
        head: (0, 0),
        dir: (1, 0),
    };

    let mut game_over = false;
    // navigation lock to prevent multiple direction changes in one frame
    let mut navigation_lock = false;
    let mut last_update = get_time();
    let speed = 0.3;

    let up = (0, -1);
    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);


    loop {
        // Does movement
        if !game_over {
            if is_key_down(KeyCode::Right) && snake.dir != left && !navigation_lock {
                snake.dir = right;
                navigation_lock = true;
            } else if is_key_down(KeyCode::Left) && snake.dir != right && !navigation_lock {
                snake.dir = left;
                navigation_lock = true;
            } else if is_key_down(KeyCode::Up) && snake.dir != down && !navigation_lock {
                snake.dir = up;
                navigation_lock = true;
            } else if is_key_down(KeyCode::Down) && snake.dir != up && !navigation_lock {
                snake.dir = down;
                navigation_lock = true;
            }

            if get_time() - last_update > speed {
                last_update = get_time();
                // snake.body.push_front(snake.head);
                snake.head = (snake.head.0 + snake.dir.0, snake.head.1 + snake.dir.1);
                
                // wrap around the screen
                if snake.head.0 < 0 {
                    snake.head.0 = SQUARES - 1;
                } else if snake.head.0 >= SQUARES {
                    snake.head.0 = 0;
                }
                if snake.head.1 < 0 {
                    snake.head.1 = SQUARES - 1;
                } else if snake.head.1 >= SQUARES {
                    snake.head.1 = 0;
                }
                navigation_lock = false;
            }
        }

        // Draws the game
        if !game_over {
            clear_background(LIGHTGRAY);

            let game_size = screen_width().min(screen_height());
            let offset_x = (screen_width() - game_size) / 2. + 10.;
            let offset_y = (screen_height() - game_size) / 2. + 10.;
            let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

            draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

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

            draw_rectangle(
                offset_x + snake.head.0 as f32 * sq_size,
                offset_y + snake.head.1 as f32 * sq_size,
                sq_size,
                sq_size,
                DARKGREEN,
            );

            draw_text(format!("SCORE: ").as_str(), 10., 20., 20., DARKGRAY);
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
                snake = Snake {
                    head: (0, 0),
                    dir: (1, 0),
                };
                last_update = get_time();
                game_over = false;
            }
        }

        // clear_background(RED);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        // draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}