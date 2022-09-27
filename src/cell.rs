use egui::*;

use crate::game;

#[derive(Debug, Copy, Clone)]
pub struct Cell(pub bool);
impl Cell {
    pub fn alive(&self) -> bool {
        self.0
    }

    pub fn to_shape(&self, x: usize, y: usize, size: usize) -> Shape {
        Shape::rect_filled(
            Rect::from_two_pos(
                Pos2 {
                    x: (x * size) as f32,
                    y: (y * size) as f32,
                },
                Pos2 {
                    x: (x + size) as f32,
                    y: (y + size) as f32,
                },
            ),
            Rounding::none(),
            if self.alive() {
                Color32::LIGHT_GRAY
            } else {
                Color32::BLACK
            },
        )
    }

    ///Returns if a cell will die depending on how many of its neighbours are alive
    /*
    Any live cell with two or three live alive_neighbours survives.
    Any dead cell with three live alive_neighbours becomes a live cell.
    All other live cells die in the next generation. Similarly, all other dead cells stay dead.
    */
    pub fn will_stay_alive(alive: bool, alive_neighbours: u8) -> bool {
        match alive_neighbours {
            3 => true,
            2 => alive,
            _ => false,
        }
    }

    ///Returns the amount of alive neighbours for a given position
    pub fn alive_neighbours(board: &game::Board, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;

        count += board[x - 1][y - 1].alive() as u8; //NW
        count += board[x][y - 1].alive() as u8; //N
        count += board[x + 1][y - 1].alive() as u8; //NE

        count += board[x - 1][y].alive() as u8; //W
        count += board[x + 1][y].alive() as u8; //E

        count += board[x - 1][y + 1].alive() as u8; //SW
        count += board[x][y + 1].alive() as u8; //S
        count += board[x + 1][y + 1].alive() as u8; //SE

        count
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell(false)
    }
}
