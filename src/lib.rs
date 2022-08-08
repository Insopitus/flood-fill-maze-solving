use std::cell::Cell;



const BIT_UPPER: u8 = 0b0000_0001;
const BIT_RIGHT: u8 = 0b0000_0010;
const BIT_LOWER: u8 = 0b0000_0100;
const BIT_LEFT: u8 = 0b0000_1000;

/// position of the tile. starts from 0, left to right then top to bottom
type Position = usize;

pub struct Maze {
    pub width:usize,
    pub height:usize,
    pub start_position: Position,
    pub goal_position: Position,
    pub tiles: Vec<Tile>,
}
// origin is left top
impl Maze {
    pub fn flood_fill(&self) {
        let mut stack = vec![];
        let start = &self.tiles[self.goal_position];
        start.value.set(Some(0));
        stack.push(start);
        while let Some(tile) = stack.pop() {
            let pos = tile.position;
            let pos_x = pos % self.width;
            let pos_y = pos / self.width;
            if pos_x < self.width - 1 && tile.right {
                let right = &self.tiles[pos + 1];
                if let Some(value) = right.value.get() {
                    if value + 1 < tile.value.get().unwrap() {
                        tile.value.set(Some(value + 1));
                        stack.push(tile);
                    }
                } else {
                    right.value.set(Some(tile.value.get().unwrap()+1));
                    stack.push(right);
                }
            };
            if pos_x > 0 && tile.left {
                let left = &self.tiles[pos - 1];
                if let Some(value) = left.value.get() {
                    if value + 1 < tile.value.get().unwrap() {
                        tile.value.set(Some(value + 1));
                        stack.push(tile);
                    }
                } else {
                    left.value.set(Some(tile.value.get().unwrap()+1));
                    stack.push(left);
                }
            };
            if pos_y < self.width - 1 && tile.lower {
                let lower = &self.tiles[pos + self.width];
                if let Some(value) = lower.value.get() {
                    if value + 1 < tile.value.get().unwrap() {
                        tile.value.set(Some(value + 1));
                        stack.push(tile);
                    }
                } else {
                    lower.value.set(Some(tile.value.get().unwrap()+1));
                    stack.push(lower);
                }
            };
            if pos_y > 0 && tile.upper {
                let upper = &self.tiles[pos - self.width];
                if let Some(value) = upper.value.get() {
                    if value + 1 < tile.value.get().unwrap() {
                        tile.value.set(Some(value + 1));
                        stack.push(tile);
                    }
                } else {
                    upper.value.set(Some(tile.value.get().unwrap()+1));
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
            let pos_x = pos % self.width;
            let pos_y = pos / self.width;
            if pos_x < self.width - 1 && tile.right {
                let right = &self.tiles[pos + 1];
                if let Some(value) = right.value.get() {
                    if value + 1 == tile.value.get().unwrap() {
                        solution.push(pos + 1);
                        tile = right;
                        continue;
                    }
                }
               
            };
            if pos_x > 0 && tile.left {
                let left = &self.tiles[pos - 1];
                if let Some(value) = left.value.get() {
                    if value + 1 == tile.value.get().unwrap() {
                        solution.push(pos - 1);
                        tile = left;
                        continue;
                    }
                }
            };
            if pos_y < self.width - 1 && tile.lower {
                let lower = &self.tiles[pos + self.width];
                if let Some(value) = lower.value.get() {
                    if value + 1 == tile.value.get().unwrap() {
                        solution.push(pos + self.width);
                        tile = lower;
                        continue;
                    }
                }
            };
            if pos_y > 0 && tile.upper {
                let upper = &self.tiles[pos - self.width];
                if let Some(value) = upper.value.get() {
                    if value + 1 == tile.value.get().unwrap() {
                        solution.push(pos - self.width);
                        tile = upper;
                        continue;
                    }
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
    pub value: Cell<Option<usize>>,
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
            value: Cell::new(None),
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
            width:3,
            height:2,
            start_position: 3,
            goal_position: 2,
            tiles:tiles.into(),
        };
        maze.flood_fill();
        assert_eq!(maze.solve(),vec![3,0,1,4,5,2]);
    }
}
