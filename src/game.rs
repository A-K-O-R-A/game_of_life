use egui::*;

use crate::cell;
use crate::consts;

pub type Board = [[cell::Cell; consts::BOARD_SIZE]; consts::BOARD_SIZE];

#[allow(dead_code)]
pub struct Game {
    board: Board,
    pub tps: usize,
    pub tick: u128,
    pub zoom_level: f32,
    pub paused: bool,
}

impl Game {
    pub fn game_tick(&mut self) {
        self.tick += 1;

        let mut new_board = Game::empty_board();

        //Optimisation, replace with for loop i=0
        for x in 1..consts::BOARD_SIZE - 1 {
            for y in 1..consts::BOARD_SIZE - 1 {
                let is_alive = self.board[x][y].0;
                let alive_neighbours = cell::Cell::alive_neighbours(&self.board, x, y);
                let next_state = cell::Cell::will_stay_alive(is_alive, alive_neighbours);
                new_board[x][y] = cell::Cell(next_state);
            }
        }

        self.board = new_board;
    }

    pub fn paint(&mut self, painter: &Painter) {
        let mut shapes: Vec<Shape> = Vec::new();

        let clip_rect = painter.clip_rect();
        let _to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, clip_rect.square_proportions() / self.zoom_level),
            clip_rect,
        );

        let background = Shape::rect_filled(clip_rect, Rounding::none(), consts::BACKGROUND_COLOR);
        shapes.push(background);

        for x in 1..consts::BOARD_SIZE - 1 {
            for y in 1..consts::BOARD_SIZE - 1 {
                let cell = self.board[x][y];
                let rect = cell.to_rect(x as f32, y as f32, self.zoom_level);

                // culling
                if clip_rect.intersects(rect) {
                    let shape = Shape::rect_filled(rect, Rounding::none(), cell.color());
                    shapes.push(shape);
                }
                //shapes.push(cell.to_shape(x, y))
            }
        }

        painter.extend(shapes);
    }

    pub fn empty_board() -> Board {
        let mut board = [[cell::Cell::default(); consts::BOARD_SIZE]; consts::BOARD_SIZE];

        board[20][20] = cell::Cell(true);
        board[20][21] = cell::Cell(true);
        board[20][22] = cell::Cell(true);

        board
    }
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: Game::empty_board(),
            tps: 1,
            tick: 0,
            zoom_level: 1.,
            paused: false,
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two() {
        1 + 1;
        ()
    }
}
*/
