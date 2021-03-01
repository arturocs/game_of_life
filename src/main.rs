use std::time::{Duration, Instant};

use rand::{thread_rng, Rng};
use speedy2d::{
    color::Color,
    dimen::Vector2,
    shape::Rectangle,
    window::{KeyScancode, MouseButton, VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D, Window,
};

const BOARD_SIZE: Vector2<usize> = Vector2::new(128, 72);
const CELL_SIZE: usize = 10;
const GEN_TIME: u64 = 100;

pub struct Game {
    board: [[u8; BOARD_SIZE.y]; BOARD_SIZE.x],
    paused: bool,
    mouse_position: Vector2<f32>,
    last_update: Instant,
}
impl Game {
    pub fn new() -> Self {
        Self {
            board: [[0; BOARD_SIZE.y]; BOARD_SIZE.x],
            paused: true,
            mouse_position: Vector2::ZERO,
            last_update: Instant::now(),
        }
    }

    fn neighbors(&self, i: usize, j: usize) -> u8 {
        self.board[i][(j + 1).clamp(0, BOARD_SIZE.y - 1)]
            + self.board[(i + 1).clamp(0, BOARD_SIZE.x - 1)][(j + 1).clamp(0, BOARD_SIZE.y - 1)]
            + self.board[(i + 1).clamp(0, BOARD_SIZE.x - 1)][j]
            + self.board[(i + 1).clamp(0, BOARD_SIZE.x - 1)][(j.saturating_sub(1))]
            + self.board[i][(j.saturating_sub(1))]
            + self.board[(i.saturating_sub(1))][j]
            + self.board[(i.saturating_sub(1))][(j.saturating_sub(1))]
            + self.board[(i.saturating_sub(1))][(j + 1).clamp(0, BOARD_SIZE.y - 1)]
    }

    fn next_generation(&mut self) {
        let mut next_board = [[0; BOARD_SIZE.y]; BOARD_SIZE.x];
        for i in 0..BOARD_SIZE.x {
            for j in 0..BOARD_SIZE.y {
                match self.neighbors(i, j) {
                    3 => next_board[i][j] = 1,
                    2 => next_board[i][j] = self.board[i][j],
                    _ => next_board[i][j] = 0,
                }
            }
        }
        self.board = next_board;
    }

    fn randomize_board(&mut self) {
        for i in 0..BOARD_SIZE.x {
            for j in 0..BOARD_SIZE.y {
                self.board[i][j] = thread_rng().gen_bool(0.5) as u8;
            }
        }
    }
}

fn draw_cell(i: usize, j: usize, graphics: &mut Graphics2D) {
    graphics.draw_rectangle(
        Rectangle::new(
            Vector2::new((i * CELL_SIZE) as f32, (j * CELL_SIZE) as f32),
            Vector2::new(((i + 1) * CELL_SIZE) as f32, ((j + 1) * CELL_SIZE) as f32),
        ),
        Color::WHITE,
    )
}

impl WindowHandler for Game {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        if !self.paused && Instant::now() - self.last_update >= Duration::from_millis(GEN_TIME) {
            self.next_generation();
            self.last_update = Instant::now();
        }

        graphics.clear_screen(Color::BLACK);
        for i in 0..BOARD_SIZE.x {
            for j in 0..BOARD_SIZE.y {
                if self.board[i][j] == 1 {
                    draw_cell(i, j, graphics)
                }
            }
        }

        helper.request_redraw();
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton) {
        let x = (self.mouse_position.x / CELL_SIZE as f32) as usize;
        let y = (self.mouse_position.y / CELL_SIZE as f32) as usize;

        match button {
            MouseButton::Left => {
                let cell = &mut self.board[x][y];
                *cell = if *cell == 0 { 1 } else { 0 };
            }
            _ => {}
        }

        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper, position: Vector2<f32>) {
        self.mouse_position = position;
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: KeyScancode,
    ) {
        match virtual_key_code {
            Some(VirtualKeyCode::C) => self.board = [[0; BOARD_SIZE.y]; BOARD_SIZE.x],
            Some(VirtualKeyCode::R) => self.randomize_board(),
            Some(VirtualKeyCode::Space) => self.paused = !self.paused,
            _ => {}
        }
    }
}

fn main() {
    let window = Window::new_centered(
        "Game of Life",
        (
            (BOARD_SIZE.x * CELL_SIZE) as u32,
            (BOARD_SIZE.y * CELL_SIZE) as u32,
        ),
    )
    .unwrap();
    window.run_loop(Game::new());
}
