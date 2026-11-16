use alloc::vec::Vec;
use crate::vga_buffer::{Color, ColorCode, WRITER, BUFFER_WIDTH};
use crate::keyboard::pop_key;
use crate::interrupts::ticks;
use pc_keyboard::{DecodedKey, KeyCode};

const BOARD_TOP: usize = 3;
const BOARD_BOTTOM: usize = 23;
const BOARD_LEFT: usize = 4;
const BOARD_RIGHT: usize = 75;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct SnakeGame {
    snake: Vec<(usize, usize)>,
    direction: Direction,
    food: (usize, usize),
    score: u32,
    high_score: u32,
    game_over: bool,
    speed_ticks: u64,
}

impl SnakeGame {
    pub fn new() -> Self {
        let mut game = SnakeGame {
            snake: Vec::new(),
            direction: Direction::Right,
            food: (30, 10),
            score: 0,
            high_score: 0,
            game_over: false,
            speed_ticks: 8,
        };
        game.reset();
        game
    }

    fn reset(&mut self) {
        self.snake.clear();
        let mid_x = 35;
        let mid_y = 12;
        self.snake.push((mid_x, mid_y));
        self.snake.push((mid_x - 1, mid_y));
        self.snake.push((mid_x - 2, mid_y));
        self.direction = Direction::Right;
        self.food = (20, 8);
        self.score = 0;
        self.speed_ticks = 8;
        self.game_over = false;
    }

    pub fn run(&mut self) {
        {
            let mut writer = WRITER.lock();
            writer.clear_screen();
        }
        self.draw_frame();
        self.draw_hud();
        self.draw_food();
        self.draw_snake();

        let mut last_tick = ticks();

        loop {
            // Process input
            if let Some(key) = pop_key() {
                match key {
                    DecodedKey::Unicode(c) => match c {
                        'w' | 'W' => {
                            if self.direction != Direction::Down {
                                self.direction = Direction::Up;
                            }
                        }
                        's' | 'S' => {
                            if self.direction != Direction::Up {
                                self.direction = Direction::Down;
                            }
                        }
                        'a' | 'A' => {
                            if self.direction != Direction::Right {
                                self.direction = Direction::Left;
                            }
                        }
                        'd' | 'D' => {
                            if self.direction != Direction::Left {
                                self.direction = Direction::Right;
                            }
                        }
                        'r' | 'R' => {
                            if self.game_over {
                                self.reset();
                                {
                                    let mut writer = WRITER.lock();
                                    writer.clear_screen();
                                }
                                self.draw_frame();
                                self.draw_hud();
                                self.draw_food();
                                self.draw_snake();
                            }
                        }
                        'q' | 'Q' => {
                            let mut writer = WRITER.lock();
                            writer.clear_screen();
                            break;
                        }
                        _ => {}
                    },
                    DecodedKey::RawKey(code) => match code {
                        KeyCode::ArrowUp => {
                            if self.direction != Direction::Down {
                                self.direction = Direction::Up;
                            }
                        }
                        KeyCode::ArrowDown => {
                            if self.direction != Direction::Up {
                                self.direction = Direction::Down;
                            }
                        }
                        KeyCode::ArrowLeft => {
                            if self.direction != Direction::Right {
                                self.direction = Direction::Left;
                            }
                        }
                        KeyCode::ArrowRight => {
                            if self.direction != Direction::Left {
                                self.direction = Direction::Right;
                            }
                        }
                        KeyCode::Escape => {
                            let mut writer = WRITER.lock();
                            writer.clear_screen();
                            break;
                        }
                        _ => {}
                    },
                }
            }

            let current_tick = ticks();

            if !self.game_over && current_tick.wrapping_sub(last_tick) >= self.speed_ticks {
                self.step();
                last_tick = current_tick;
            }

            x86_64::instructions::hlt();
        }
    }

    fn step(&mut self) {
        let head = self.snake[0];
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1.saturating_sub(1)),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0.saturating_sub(1), head.1),
            Direction::Right => (head.0 + 1, head.1),
        };

        // Wall collision
        if new_head.0 <= BOARD_LEFT || new_head.0 >= BOARD_RIGHT || new_head.1 <= BOARD_TOP || new_head.1 >= BOARD_BOTTOM {
            self.game_over = true;
            self.draw_game_over();
            return;
        }

        // Self collision
        for seg in self.snake.iter().skip(1) {
            if *seg == new_head {
                self.game_over = true;
                self.draw_game_over();
                return;
            }
        }

        // Move
        self.snake.insert(0, new_head);

        // Check food
        if new_head == self.food {
            self.score += 10;
            if self.score > self.high_score {
                self.high_score = self.score;
            }
            if self.speed_ticks > 3 && self.score % 50 == 0 {
                self.speed_ticks -= 1;
            }
            self.spawn_food();
            self.draw_hud();
            self.draw_food();
        } else {
            // Remove tail
            if let Some(tail) = self.snake.pop() {
                let mut writer = WRITER.lock();
                writer.write_at(tail.1, tail.0, b' ', ColorCode::new(Color::Black, Color::Black));
            }
        }

        self.draw_snake();
    }

    fn spawn_food(&mut self) {
        let t = ticks() as usize;
        let w = BOARD_RIGHT - BOARD_LEFT - 4;
        let h = BOARD_BOTTOM - BOARD_TOP - 4;
        let fx = BOARD_LEFT + 2 + (t * 7 + 13) % w;
        let fy = BOARD_TOP + 2 + (t * 3 + 7) % h;
        self.food = (fx, fy);
    }

    fn draw_food(&self) {
        let mut writer = WRITER.lock();
        writer.write_at(self.food.1, self.food.0, b'*', ColorCode::new(Color::Yellow, Color::Black));
    }

    fn draw_snake(&self) {
        let mut writer = WRITER.lock();
        for (i, seg) in self.snake.iter().enumerate() {
            let color = if i == 0 {
                ColorCode::new(Color::LightGreen, Color::Black)
            } else {
                ColorCode::new(Color::Green, Color::Black)
            };
            let char_byte = if i == 0 { b'O' } else { b'o' };
            writer.write_at(seg.1, seg.0, char_byte, color);
        }
    }

    fn draw_frame(&self) {
        let mut writer = WRITER.lock();
        let border_color = ColorCode::new(Color::Cyan, Color::Black);

        for col in BOARD_LEFT..=BOARD_RIGHT {
            writer.write_at(BOARD_TOP, col, b'#', border_color);
            writer.write_at(BOARD_BOTTOM, col, b'#', border_color);
        }
        for row in BOARD_TOP..=BOARD_BOTTOM {
            writer.write_at(row, BOARD_LEFT, b'#', border_color);
            writer.write_at(row, BOARD_RIGHT, b'#', border_color);
        }
    }

    fn draw_hud(&self) {
        let mut writer = WRITER.lock();
        let title_color = ColorCode::new(Color::White, Color::DarkGray);

        for c in 0..BUFFER_WIDTH {
            writer.write_at(0, c, b' ', title_color);
            writer.write_at(1, c, b' ', ColorCode::new(Color::LightGray, Color::Black));
        }

        let title = b"  RustOS Arcade: Snake Classic  ";
        for (i, &b) in title.iter().enumerate() {
            writer.write_at(0, i, b, ColorCode::new(Color::Yellow, Color::DarkGray));
        }

        let hud_str = alloc::format!("Score: {:04}    High Score: {:04}    Speed Level: {}",
            self.score, self.high_score, 9 - self.speed_ticks);
        for (i, b) in hud_str.bytes().enumerate() {
            writer.write_at(0, 34 + i, b, title_color);
        }

        let inst = b"Controls: [W/A/S/D or Arrows] Turn  [R] Restart  [Q] Exit to Shell";
        for (i, &b) in inst.iter().enumerate() {
            writer.write_at(1, 4 + i, b, ColorCode::new(Color::LightCyan, Color::Black));
        }
    }

    fn draw_game_over(&self) {
        let mut writer = WRITER.lock();
        let row = 12;
        let text1 = b"========================================";
        let text2 = b"             GAME OVER !               ";
        let text3 = b"     Press [R] Restart | [Q] Exit       ";
        let color = ColorCode::new(Color::White, Color::Red);

        for (i, &b) in text1.iter().enumerate() {
            writer.write_at(row - 1, 20 + i, b, color);
            writer.write_at(row, 20 + i, text2[i], color);
            writer.write_at(row + 1, 20 + i, text3[i], color);
            writer.write_at(row + 2, 20 + i, b, color);
        }
    }
}
