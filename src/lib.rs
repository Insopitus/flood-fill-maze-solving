use std::{str::FromStr, fmt::Display};

/// position of the tile. starts from 0, left to right then top to bottom
type Index = usize;
type Coordinate = (usize, usize);


/// can be represented as a string like bellow
/// 
/// w5h5s10g0#0000011101000011010100100
/// 
/// which is a maze whose width is 5, height is 5, start from index 10, goal is at index 0. 
/// tiles are after `#` where 1s means walls and 0s means pathes
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub start_position: Index,
    pub goal_position: Index,
    tiles: Vec<Tile>,
    values:Vec<u32>
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
                    right.value.set(Some(tile.value.get().unwrap() + 1));
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
                    left.value.set(Some(tile.value.get().unwrap() + 1));
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
                    lower.value.set(Some(tile.value.get().unwrap() + 1));
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
                    upper.value.set(Some(tile.value.get().unwrap() + 1));
                    stack.push(upper);
                }
            };
        }
    }

    pub fn solve(&self) -> Vec<Index> {
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
        }
        solution
    }
    // get neighboring tiles of a certain on (top, down, left, right), if there's the border of the maze on that direction, returns None
    pub fn get_neighbors(
        tile: Index,
    ) -> (
        Option<(Index, Tile)>,
        Option<(Index, Tile)>,
        Option<(Index, Tile)>,
        Option<(Index, Tile)>,
    ) {
        todo!()
    }
    pub fn get_tile_position(&self, index: Index) -> Coordinate {
        (index % self.width, index / self.width)
    }
    pub fn get_tile_index(&self, coord: Coordinate) -> Index {
        coord.1 * self.width + coord.0
    }
}


#[derive(Debug)]
enum MazeParseError{
    InvalidToken(char),
    InvalidSize
}
impl FromStr for Maze{
    type Err = MazeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
    }
}
enum Tile {
    Wall,
    Path,
}

#[cfg(test)]
mod test {

    use crate::{Maze, Tile};

    #[test]
    fn basic() {}
}
