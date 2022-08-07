use std::cell::{Cell, Ref, RefCell};

const MAZE_WIDTH: usize = 3;
const MAZE_HEIGHT: usize = 2;

const BIT_UPPER: u8 = 0b0000_0001;
const BIT_RIGHT: u8 = 0b0000_0010;
const BIT_LOWER: u8 = 0b0000_0100;
const BIT_LEFT: u8 = 0b0000_1000;

pub struct Maze {
    pub start_point: Position,
    pub goal: Position,
    pub tiles: RefCell<[Tile; MAZE_HEIGHT * MAZE_WIDTH]>,
}
// origin is left top
impl Maze {
    pub fn flood_fill(&self) {
        let mut stack: Vec<Tile> = vec![];
        let mut start = self.get_tile_at(&self.goal);
        start.value = 0;
        stack.push(start);
        while let Some(tile) = stack.pop() {
            let pos = &tile.position;
            dbg!(pos);
            if pos.x < MAZE_WIDTH - 1 && tile.right {
                let mut right = self.get_tile_at(&Position::new(pos.x + 1, pos.y));
                if right.value > tile.value + 1 {
                    right.value = tile.value + 1;
                    stack.push(right);
                }
            };
            if pos.x > 0 && tile.left {
                let mut left = self.get_tile_at(&Position::new(pos.x - 1, pos.y));
                if left.value > tile.value + 1 {
                    left.value = tile.value + 1;
                    stack.push(left);
                }
            };
            if pos.y < MAZE_HEIGHT - 1 && tile.lower {
                let mut lower = self.get_tile_at(&Position::new(pos.x, pos.y + 1));
                if lower.value > tile.value + 1 {
                    lower.value = tile.value + 1;
                    stack.push(lower);
                }
            };
            if pos.y > 0 && tile.upper {
                let mut upper = self.get_tile_at(&Position::new(pos.x, pos.y - 1));
                if upper.value > tile.value + 1 {
                    upper.value = tile.value + 1;
                    stack.push(upper);
                }
            };
        }
    }

    pub fn solve(&self) -> Vec<Position> {
        todo!()
    }

    fn get_tile_at(&self, pos: &Position) -> Tile {
        self.tiles.borrow_mut()[pos.x + pos.y * MAZE_WIDTH]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub position: Position,
    /// if upper is open
    pub upper: bool,
    pub lower: bool,
    pub left: bool,
    pub right: bool,
    /// cell value in flood fill algorithm
    pub value: usize,
}

impl Tile {
    pub fn new(pos: usize, walls: u8) -> Self {
        let x = pos % MAZE_WIDTH;
        let y = pos / MAZE_WIDTH;
        Self {
            position: Position::new(x, y),
            upper: walls & BIT_UPPER > 0,
            lower: walls & BIT_LOWER > 0,
            left: walls & BIT_LEFT > 0,
            right: walls & BIT_RIGHT > 0,
            value: MAZE_HEIGHT * MAZE_WIDTH,
        }
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use crate::{Position, Tile, Maze};

    #[test]
    fn basic() {
        let tiles = [
            Tile::new(0,4),
            Tile::new(1,14),
            Tile::new(2,8), //goal
            Tile::new(3,2),
            Tile::new(4,11),
            Tile::new(5,8),
        ];
        let maze = Maze{
            start_point:Position { x: 0, y: 1 },
            goal:Position { x: 2, y: 0 },
            tiles:RefCell::new(tiles)
        };
        maze.flood_fill();
        dbg!(tiles);
    }
}
