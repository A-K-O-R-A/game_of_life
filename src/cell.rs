use egui::*;

use crate::consts;
use crate::game;

#[derive(Debug, Copy, Clone)]
pub struct Cell(pub bool);
impl Cell {
    pub fn alive(&self) -> bool {
        self.0
    }

    pub fn color(&self) -> Color32 {
        if self.alive() {
            Color32::LIGHT_GRAY
        } else {
            Color32::BLACK
        }
    }

    pub fn to_rect(&self, x: f32, y: f32, zoom_level: f32) -> Rect {
        /*
        let top_left = Pos2 {
            x: (x * (consts::CELL_SIZE + consts::CELL_BORDER_SIZE)) * zoom_level,
            y: (y * (consts::CELL_SIZE + consts::CELL_BORDER_SIZE)) * zoom_level,
        };
        */

        let top_left = Pos2 {
            x: (x * (consts::CELL_SIZE)) * zoom_level,
            y: (y * (consts::CELL_SIZE)) * zoom_level,
        };

        let bottom_right = Pos2 {
            x: (top_left.x + consts::CELL_SIZE) * zoom_level,
            y: (top_left.y + consts::CELL_SIZE) * zoom_level,
        };

        Rect::from_two_pos(top_left, bottom_right)
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
