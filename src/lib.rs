use std::str::FromStr;

/// position of the tile. starts from 0, left to right then top to bottom
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
    pub start_position: usize,
    pub goal_position: usize,
    tiles: Vec<Tile>,
    values: Vec<Option<u32>>,
}
// origin is left top
impl Maze {
    pub fn flood_fill(&mut self) {
        let mut stack: Vec<usize> = vec![self.goal_position];
        self.values = vec![None; self.width * self.height];
        // flood fill from the goal position
        self.values[self.goal_position] = Some(0);
        while let Some(index) = stack.pop() {
            let value = self.values[index].unwrap();
            let neighbors = self.get_neighbors(index);
            // to avoid underflow
            let index = index as isize;
            // the index offsets in different directions
            let index_shift: [isize; 4] = [-(self.width as isize), self.width as isize, -1, 1];
            for (neighbor, shift) in neighbors.iter().zip(index_shift) {
                if let Some(neighbor) = neighbor {
                    if *neighbor == Tile::Path {
                        // to avoid underflow
                        let neighbor_index = (index + shift) as usize;
                        let neighbor_value = self.values[neighbor_index];
                        if neighbor_value.is_none() {
                            self.values[neighbor_index] = Some(value + 1);
                            stack.push(neighbor_index);
                        } else if let Some(v) = neighbor_value {
                            if v > value + 1 {
                                self.values[neighbor_index] = Some(value + 1);
                                stack.push(neighbor_index);
                            }
                        }
                    }
                }
            }
        }
    }

    /// returns None if the maze is unsolvable
    pub fn solve(&self) -> Option<Vec<usize>> {
        // TODO prefers going straight when to paths have same steps
        // enum Direction {
        //     Top = 1isize,
        //     Bottom = 2isize,
        //     Left = 3isize,
        //     Right = 4isize,
        // }
        // let mut dir = Direction::Top;
        let mut solution = vec![self.start_position];
        let mut curr_index = self.start_position;

        // if start tile is not a path tile, the maze is unsolvable
        let index_shift: [isize; 4] = [-(self.width as isize), self.width as isize, -1, 1];

        while curr_index != self.goal_position {
            let curr_value = self.values[curr_index]?;
            let mut min = curr_value;
            let mut min_index = curr_index;
            let neighbors = self.get_neighbors(curr_index);
            for (n, shift) in neighbors.iter().zip(index_shift) {
                if let Some(n) = n {
                    if *n == Tile::Path {
                        let index = (curr_index as isize + shift) as usize;
                        let value = self.values[index]?; // will always has a value
                        if value < min {
                            min = value;
                            min_index = index;
                        }
                    }
                }
            }
            solution.push(min_index);
            curr_index = min_index;
        }
        Some(solution)
    }
    // get neighboring tiles of a certain on (top, down, left, right), if there's the border of the maze on that direction, returns None
    pub fn get_neighbors(&self, index: usize) -> [Option<Tile>; 4] {
        let (x, y) = self.get_tile_coord(index);
        let mut neighbors: [Option<Tile>; 4] = [None, None, None, None];
        if x != 0 {
            neighbors[2] = Some(self.tiles[index - 1]);
        }
        if x != self.width - 1 {
            neighbors[3] = Some(self.tiles[index + 1]);
        }
        if y != 0 {
            let top_index = self.get_tile_index((x, y - 1));
            neighbors[0] = Some(self.tiles[top_index]);
        }
        if y != self.height - 1 {
            let bottom_index = self.get_tile_index((x, y + 1));
            neighbors[1] = Some(self.tiles[bottom_index]);
        }
        neighbors
    }
    pub fn get_tile_coord(&self, index: usize) -> Coordinate {
        (index % self.width, index / self.width)
    }
    pub fn get_tile_index(&self, coord: Coordinate) -> usize {
        coord.1 * self.width + coord.0
    }
    /// print the maze in terminal
    pub fn visualize(&self) {
        let mut output = String::new();
        println!();
        for (i, tile) in self.tiles.iter().enumerate() {
            if i == self.goal_position {
                output.push_str("G ");
            } else if i == self.start_position {
                output.push_str("S ");
            } else {
                let c = match tile {
                    Tile::Wall => "[7m  [0m",
                    Tile::Path => "  ",
                };
                output.push_str(c);
            }

            if (i + 1) % self.width == 0 {
                output.push('\n');
            }
        }

        println!("{output}");
        println!();
    }
    pub fn visulize_values(&self) {
        // let mut output = String::new();
        // for (i, v) in self.values.iter().enumerate() {
        //     if i == self.start_position {
        //         output.push('S');
        //     } else {
        //         let c = match *v {
        //             Some(v) =>
        //         };
        //         output.push(c);
        //     }

        //     if (i + 1) % self.width == 0 {
        //         output.push('\n');
        //     }
        // }
        // println!("{output}");
    }
}

#[derive(Debug)]
pub enum MazeParseError {
    InvalidToken,
    InvalidSize,
    UnexpectedEnding,
}
impl FromStr for Maze {
    type Err = MazeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<_> = s.chars().collect();
        let mut start: usize = 0;
        let mut tokens: Vec<&[char]> = vec![];
        for (i, c) in chars.iter().enumerate() {
            // TODO returns for unexpected ending error here too
            if (chars[start].is_alphabetic() && c.is_numeric())
                || (chars[start].is_numeric() && c.is_alphabetic())
            {
                tokens.push(&chars[start..i]);
                start = i;
            }
            if *c == '#' {
                tokens.push(&chars[start..i]);
                if chars.get(i + 1).is_none() {
                    return Err(MazeParseError::UnexpectedEnding);
                }
                tokens.push(&chars[i + 1..]);
                break;
            }
        }
        let mut tokens = tokens.iter();
        let w_char: String = tokens
            .next()
            .ok_or(MazeParseError::InvalidToken)?
            .iter()
            .collect();
        if w_char != "w" {
            return Err(MazeParseError::InvalidToken);
        }
        let w_num: usize = tokens
            .next()
            .ok_or(MazeParseError::InvalidToken)?
            .iter()
            .collect::<String>()
            .parse()
            .map_err(|_| MazeParseError::InvalidToken)?;

        // dbg!(&w_char,&w_num);

        let h_char: String = tokens
            .next()
            .ok_or(MazeParseError::InvalidToken)?
            .iter()
            .collect();
        if h_char != "h" {
            return Err(MazeParseError::InvalidToken);
        }
        let h_num: usize = tokens
            .next()
            .ok_or(MazeParseError::InvalidToken)?
            .iter()
            .collect::<String>()
            .parse()
            .map_err(|_| MazeParseError::InvalidToken)?;

        // dbg!(&h_char,&h_num);

        let s_char: String = tokens
            .next()
            .ok_or(MazeParseError::InvalidToken)?
            .iter()
            .collect();
        if s_char != "s" {
            return Err(MazeParseError::InvalidToken);
        }
        let s_num: usize = tokens
            .next()
            .ok_or(MazeParseError::InvalidToken)?
            .iter()
            .collect::<String>()
            .parse()
            .map_err(|_| MazeParseError::InvalidToken)?;

        // dbg!(&s_char,&s_num);

        let g_char: String = tokens
            .next()
            .ok_or(MazeParseError::InvalidToken)?
            .iter()
            .collect();
        if g_char != "g" {
            return Err(MazeParseError::InvalidToken);
        }
        let g_num: usize = tokens
            .next()
            .ok_or(MazeParseError::InvalidToken)?
            .iter()
            .collect::<String>()
            .parse()
            .map_err(|_| MazeParseError::InvalidToken)?;

        let tiles_res: Vec<_> = tokens
            .next()
            .ok_or(MazeParseError::InvalidToken)?
            .iter()
            .map(|c| Tile::parse(*c))
            .collect();
        let mut tiles = vec![];
        for t in tiles_res {
            tiles.push(t?);
        }

        Ok(Self {
            width: w_num,
            height: h_num,
            start_position: s_num,
            goal_position: g_num,
            values: vec![],
            tiles,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Wall,
    Path,
}
impl Tile {
    fn parse(c: char) -> Result<Self, MazeParseError> {
        match c {
            '0' => Ok(Self::Path),
            '1' => Ok(Self::Wall),
            _ => Err(MazeParseError::InvalidToken),
        }
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

    use crate::{Maze, Tile};
    const MAZE_STR: &str = "w7h7s7g41#1111111000010111101011000101101110110000001111111";
    #[test]
    fn basic() {
        let mut maze = Maze::from_str(MAZE_STR).unwrap();
        maze.visualize();
        maze.flood_fill();
        assert_eq!(
            &maze.solve().unwrap(),
            &[7, 8, 9, 10, 17, 24, 23, 22, 29, 36, 37, 38, 39, 40, 41,]
        );
    }
    #[test]
    fn parse() {
        let maze = Maze::from_str(MAZE_STR).unwrap();
        assert_eq!(maze.width, 7);
        assert_eq!(maze.height, 7);
        assert_eq!(maze.start_position, 7);
        assert_eq!(maze.goal_position, 41);
        assert_eq!(maze.tiles[0], Tile::Wall);
        assert_eq!(maze.tiles[7], Tile::Path);
    }
}
