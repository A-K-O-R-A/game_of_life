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
    pub fn game_tick(&mut self, clicked_pos: Option<(usize, usize)>) {
        if !self.paused {
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
        } else {
            if let Some(pos) = clicked_pos {
                self.board[pos.0][pos.1] = cell::Cell(true);
            }
        }
    }

    pub fn paint(&mut self, painter: &Painter, mouse_pos: Option<Pos2>) {
        //Stop vector from reallocating
        let mut shapes: Vec<Shape> =
            Vec::with_capacity(consts::BOARD_SIZE * consts::BOARD_SIZE + 1);

        let clip_rect = painter.clip_rect();
        let _to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, clip_rect.square_proportions() / self.zoom_level),
            clip_rect,
        );

        let background = Shape::rect_filled(clip_rect, Rounding::none(), consts::BACKGROUND_COLOR);
        shapes.push(background);

        let hovered_cell = self.mouse_hover(mouse_pos);

        for x in 1..consts::BOARD_SIZE - 1 {
            for y in 1..consts::BOARD_SIZE - 1 {
                let cell = self.board[x][y];
                let rect = cell.to_rect(x as f32, y as f32, self.zoom_level);

                // culling
                if clip_rect.intersects(rect) {
                    let mut color = cell.color();

                    if hovered_cell == Some((x, y)) {
                        if cell.alive() {
                            color = Color32::RED;
                        } else {
                            color = Color32::DARK_RED;
                        }
                    }

                    let shape = Shape::rect_filled(rect, Rounding::none(), color);
                    shapes.push(shape);
                }
                //shapes.push(cell.to_shape(x, y))
            }
        }

        painter.extend(shapes);
    }

    ///Returns the board indices of a cell if it is hovered
    pub fn mouse_hover(&self, mouse_pos: Option<Pos2>) -> Option<(usize, usize)> {
        match mouse_pos {
            Some(pos) => {
                let x = (pos.x / (consts::CELL_SIZE + consts::CELL_BORDER_SIZE)) as usize;
                let y = (pos.y / (consts::CELL_SIZE + consts::CELL_BORDER_SIZE)) as usize;

                if x > consts::BOARD_SIZE || y > consts::BOARD_SIZE {
                    return None;
                }

                Some((x, y))
            }
            None => None,
        }
    }

    pub fn empty_board() -> Board {
        let mut board = [[cell::Cell::default(); consts::BOARD_SIZE]; consts::BOARD_SIZE];

        board[10][10] = cell::Cell(true);
        board[10][11] = cell::Cell(true);
        board[10][12] = cell::Cell(true);

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
