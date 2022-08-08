use std::cell::Cell;

const MAZE_WIDTH: usize = 3;
const MAZE_HEIGHT: usize = 2;

const BIT_UPPER: u8 = 0b0000_0001;
const BIT_RIGHT: u8 = 0b0000_0010;
const BIT_LOWER: u8 = 0b0000_0100;
const BIT_LEFT: u8 = 0b0000_1000;

/// position of the tile. starts from 0, left to right then top to bottom
type Position = usize;

pub struct Maze {
    pub start_position: Position,
    pub goal_position: Position,
    pub tiles: [Tile; MAZE_HEIGHT * MAZE_WIDTH],
}
// origin is left top
impl Maze {
    pub fn flood_fill(&self) {
        let mut stack = vec![];
        let start = &self.tiles[self.goal_position];
        start.value.set(0);
        stack.push(start);
        while let Some(tile) = stack.pop() {
            let pos = tile.position;
            let pos_x = pos % MAZE_WIDTH;
            let pos_y = pos / MAZE_WIDTH;
            if pos_x < MAZE_WIDTH - 1 && tile.right {
                let right = &self.tiles[pos + 1];
                if right.value.get() > tile.value.get() + 1 {
                    right.value.set(tile.value.get() + 1);
                    stack.push(right);
                }
            };
            if pos_x > 0 && tile.left {
                let left = &self.tiles[pos - 1];
                if left.value.get() > tile.value.get() + 1 {
                    left.value.set(tile.value.get() + 1);
                    stack.push(left);
                }
            };
            if pos_y < MAZE_HEIGHT - 1 && tile.lower {
                let lower = &self.tiles[pos + MAZE_WIDTH];
                if lower.value.get() > tile.value.get() + 1 {
                    lower.value.set(tile.value.get() + 1);
                    stack.push(lower);
                }
            };
            if pos_y > 0 && tile.upper {
                let upper = &self.tiles[pos - MAZE_WIDTH];
                if upper.value.get() > tile.value.get() + 1 {
                    upper.value.set(tile.value.get() + 1);
                    stack.push(upper);
                }
            };
        }
    }

    pub fn solve(&self) -> Vec<Position> {
        // todo!();
        let mut solution = Vec::new();
        solution.push(self.start_position);
        let mut tile = &self.tiles[self.start_position];
        while tile.position != self.goal_position {
            let pos = tile.position;
            let pos_x = pos % MAZE_WIDTH;
            let pos_y = pos / MAZE_WIDTH;
            if pos_x < MAZE_WIDTH - 1 && tile.right {
                let right = &self.tiles[pos + 1];
                if right.value.get() == tile.value.get() - 1 {
                    tile = right;
                    solution.push(tile.position);
                    continue;
                }
            };
            if pos_x > 0 && tile.left {
                let left = &self.tiles[pos - 1];
                if left.value.get() == tile.value.get() - 1 {
                    tile = left;
                    solution.push(tile.position);
                    continue;
                }
            };
            if pos_y < MAZE_HEIGHT - 1 && tile.lower {
                let lower = &self.tiles[pos + MAZE_WIDTH];
                if lower.value.get() == tile.value.get() - 1 {
                    tile = lower;
                    solution.push(tile.position);
                    continue;
                }
            };
            if pos_y > 0 && tile.upper {
                let upper = &self.tiles[pos - MAZE_WIDTH];
                if upper.value.get() == tile.value.get() - 1 {
                    tile = upper;
                    solution.push(tile.position);
                    continue;
                }
            };
        };
        solution
        
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub position: usize,
    /// if upper is open
    pub upper: bool,
    pub lower: bool,
    pub left: bool,
    pub right: bool,
    /// cell value in flood fill algorithm
    pub value: Cell<usize>,
}

impl Tile {
    /// create a new tile
    /// pos: position of the tile. starts from 0, left to right then top to bottom
    /// opens: openings of the tile. bitwise OR of the four walls. 8 means left is open, 1 means upper is open, 2 for right and 4 for bottom;
    pub fn new(pos: usize, openings: u8) -> Self {
        Self {
            position: pos,
            upper: openings & BIT_UPPER > 0,
            lower: openings & BIT_LOWER > 0,
            left: openings & BIT_LEFT > 0,
            right: openings & BIT_RIGHT > 0,
            value: Cell::new(MAZE_HEIGHT * MAZE_WIDTH),
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{Maze, Tile};

    #[test]
    fn basic() {
        let tiles = [
            Tile::new(0, 6),
            Tile::new(1, 12),
            Tile::new(2, 4), //goal
            Tile::new(3, 1), //start
            Tile::new(4, 3),
            Tile::new(5, 9),
        ];
        let maze = Maze {
            start_position: 3,
            goal_position: 2,
            tiles,
        };
        maze.flood_fill();
        assert_eq!(maze.solve(),vec![3,0,1,4,5,2]);
    }
}
