use macroquad::prelude::*;

const CELL_SIZE: f32 = 30.0;
const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 20;

#[derive(Clone, Copy)]

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Food {
    x: i32,
    y: i32,
    size: i32,
}

fn spawn_food(snake: &Vec<(i32, i32)>) -> Food {
    loop {
        let x = rand::gen_range(0, GRID_WIDTH);
        let y = rand::gen_range(0, GRID_HEIGHT);
        let size = rand::gen_range(1, 4);

        let mut overlap = false;
        for dx in 0..size {
            for dy in 0..size {
                if snake.contains(&(x + dx, y + dy)) || x + dx >= GRID_WIDTH || y + dy >= GRID_HEIGHT {
                    overlap = true;
                    break;
                }
            }
            if overlap {
                break;
            }
            if !overlap {
                return Food {x, y, size};
            }
        }
    }
}

#[macroquad::main(window_conf)]

async fn main() {

    let mut snake: Vec<(i32, i32)> = vec![
        (5, 5),
        (4, 5),
        (3, 5),
    ];

    let mut food = spawn_food(&snake);
    let mut dir = Direction::Right;
    let mut game_over = false;
    let mut score: i32 = 0;
    let mut timer = 0.0;
    let mut eat_anim_timer = 0.0;
    let mut eating = false;
    let eat_anim_duration = 0.15;
    let speed = 0.15;

    loop {
        let dt = get_frame_time();
        timer += dt;

        if eating {
            eat_anim_timer -= dt;
            if eat_anim_timer <= 0.0 {
                eating = false;
            }
        }

        if is_key_pressed(KeyCode::Up) && !matches!(dir, Direction::Down){
            dir = Direction::Up;
        }
        if is_key_pressed(KeyCode::Down) && !matches!(dir, Direction::Up){
            dir = Direction::Down;
        }
        if is_key_pressed(KeyCode::Left) && !matches!(dir, Direction::Right){
            dir = Direction::Left;
        }
        if is_key_pressed(KeyCode::Right) && !matches!(dir, Direction::Left){
            dir = Direction::Right;
        }

        if timer >= speed {
            timer = 0.0;

            let (hx, hy) = snake[0];

            let new_head = match dir {
                Direction::Up => (hx, hy - 1),
                Direction::Down => (hx, hy + 1),
                Direction::Left => (hx - 1, hy),
                Direction::Right => (hx + 1, hy),
            }; 

            if new_head. 0 < 0 || new_head.0 >= GRID_WIDTH || new_head.1 < 0 || new_head.1 >= GRID_HEIGHT || snake.contains(&new_head) {
                game_over = true;
            }

            snake.insert(0, new_head);

            let mut ate_food = false;
            for dx in 0..food.size {
                for dy in 0..food.size {
                    if new_head == (food.x + dx, food.y + dy) {
                        ate_food = true;
                        break;
                    }
                }
                if ate_food {
                    break;
                }
            }

            if ate_food {
                let growth = if food.size >= 2 {
                    3
                } else {
                    1
                };
                let score_add = if food.size >= 2 {
                    3
                } else {
                    1
                };

                food = spawn_food(&snake);
                score += score_add;

                for _ in 0..growth {
                    snake.push(*snake.last().unwrap());
                }

                eating = true;
                eat_anim_timer = eat_anim_duration;
            } else {
                snake.pop();
            }
        }

        clear_background(BLACK);

        for (i, (x, y)) in snake.iter().enumerate() {
            let color = if i == 0 { Color::new(0.0, 0.5, 0.0, 1.0) } else { GREEN };
            draw_rectangle(
                *x as f32 * CELL_SIZE,
                *y as f32 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                color,
            );
        }

        let scale = if eating {
            1.3
        } else {
            1.0
        };
        let food_color = if food.size >= 2 {
            ORANGE
        } else {
            RED
        };

        for dx in 0..food.size {
            for dy in 0..food.size {
                draw_rectangle(
                    (food.x + dx) as f32 * CELL_SIZE,
                    (food.y + dy) as f32 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    food_color,
                );
            }
        }

        draw_text(
            &format!("Score : {}", score),
            10.0,
            30.0,
            30.0,
            WHITE,
        );

        if game_over {
            clear_background(BLACK);
            draw_text(
                "GAME OVER! (ENTER)",
                100.0,
                300.0,
                50.0,
                RED,
            );
            
            if is_key_pressed(KeyCode::Enter) {
                snake = vec![
                    (5, 5),
                    (4, 5),
                    (3, 5),
                ];
                dir = Direction::Right;
                food = spawn_food(&snake);
                score = 0;
                game_over = false;
            }
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_string(),
        window_width: (GRID_WIDTH as f32 * CELL_SIZE) as i32,
        window_height: (GRID_HEIGHT as f32 * CELL_SIZE) as i32,
        ..Default::default()
    }
}
