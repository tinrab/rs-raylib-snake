use std::collections::VecDeque;

use crate::{
    input::Input,
    math::Vector2i,
    raylib::{self, Color, KeyboardKey},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Game {
    size: Vector2i,
    snake: VecDeque<Vector2i>,
    direction: Direction,
    food: Vector2i,
    pub game_over: bool,
    pub score: u32,
}

impl Game {
    pub fn new(size: Vector2i) -> Self {
        let mut snake = VecDeque::new();
        snake.push_back(Vector2i::new((size.x as f32 * 0.1) as i32, size.y / 2));

        Game {
            size,
            snake,
            direction: Direction::Right,
            food: Vector2i {
                x: rand::random::<i32>().abs() % size.x,
                y: rand::random::<i32>().abs() % size.y,
            },
            game_over: false,
            score: 0,
        }
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        if Input::is_key_down(KeyboardKey::KeyUp) && self.direction != Direction::Down {
            println!("UP");
            self.direction = Direction::Up;
        }
        if Input::is_key_down(KeyboardKey::KeyDown) && self.direction != Direction::Up {
            self.direction = Direction::Down;
        }
        if Input::is_key_down(KeyboardKey::KeyLeft) && self.direction != Direction::Right {
            self.direction = Direction::Left;
        }
        if Input::is_key_down(KeyboardKey::KeyRight) && self.direction != Direction::Left {
            self.direction = Direction::Right;
        }

        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Vector2i {
                x: head.x,
                y: head.y - 1,
            },
            Direction::Down => Vector2i {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Vector2i {
                x: head.x - 1,
                y: head.y,
            },
            Direction::Right => Vector2i {
                x: head.x + 1,
                y: head.y,
            },
        };

        if new_head.x < 0
            || new_head.x >= self.size.x
            || new_head.y < 0
            || new_head.y >= self.size.y
        {
            self.game_over = true;
            return;
        }

        if self
            .snake
            .iter()
            .any(|pos| pos.x == new_head.x && pos.y == new_head.y)
        {
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        if new_head.x == self.food.x && new_head.y == self.food.y {
            self.score += 1;
            loop {
                self.food = Vector2i {
                    x: rand::random::<i32>().abs() % self.size.x,
                    y: rand::random::<i32>().abs() % self.size.y,
                };
                if !self
                    .snake
                    .iter()
                    .any(|p| p.x == self.food.x && p.y == self.food.y)
                {
                    break;
                }
            }
        } else {
            self.snake.pop_back();
        }
    }

    pub fn render(&self, scale: i32) {
        // Render snake
        for p in self.snake.iter() {
            raylib::draw_rectangle(p.x * scale, p.y * scale, scale, scale, Color::BLUE);
        }

        // Render food
        raylib::draw_rectangle(
            self.food.x * scale,
            self.food.y * scale,
            scale,
            scale,
            Color::RED,
        );

        // Render score
        if !self.game_over {
            raylib::draw_text(format!("Score: {}", self.score), 10, 10, 20, Color::WHITE);
        }
    }
}
