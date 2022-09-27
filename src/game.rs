use egui::*;

use crate::cell;

pub const BOARD_SIZE: usize = 200;
pub type Board = [[cell::Cell; BOARD_SIZE]; BOARD_SIZE];

#[allow(dead_code)]
pub struct Game {
    board: Board,
    tps: usize,
    tick: u128,
    zoom_level: f32,
    paused: bool,
}

impl Game {
    pub fn game_tick(&mut self) {
        self.tick += 1;

        let mut new_board = Game::empty_board();

        //Optimisation, replace with for loop i=0
        for x in 1..BOARD_SIZE - 1 {
            for y in 1..BOARD_SIZE - 1 {
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

        let rect = painter.clip_rect();
        let to_screen = emath::RectTransform::from_to(
            Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom_level),
            rect,
        );

        let test_rect = Shape::rect_filled(
            Rect::from_two_pos(Pos2::ZERO, Pos2 { x: 100., y: 100. }),
            Rounding::same(30.),
            Color32::LIGHT_GRAY,
        );
        shapes.push(test_rect);

        for x in 1..BOARD_SIZE - 1 {
            for y in 1..BOARD_SIZE - 1 {
                shapes.push(self.board[x][y].to_shape(x, y, 10))
            }
        }

        //shapes.push(Shape::line_segment(line, (width, color)));

        /*
        let mut paint_line = |points: [Pos2; 2], color: Color32, width: f32| {
            let line = [to_screen * points[0], to_screen * points[1]];

            // culling
            if rect.intersects(Rect::from_two_pos(line[0], line[1])) {
                shapes.push(Shape::line_segment(line, (width, color)));
            }
        };

        let mut paint_rect = |points: [Pos2; 2], color: Color32| {
            let line = [to_screen * points[0], to_screen * points[1]];

            // culling
            if rect.intersects(Rect::from_two_pos(line[0], line[1])) {
                shapes.push(Shape::line_segment(line, (width, color)));
            }
        };

        paint_line(
            [Pos2 { x: 0., y: 0. }, Pos2 { x: 100., y: 100. }],
            Color32::BLACK,
            10.,
        );
        */

        painter.extend(shapes);
    }

    pub fn empty_board() -> Board {
        [[cell::Cell::default(); BOARD_SIZE]; BOARD_SIZE]
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
