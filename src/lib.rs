use std::cell::{Cell, RefCell, Ref};

const MAZE_WIDTH: usize = 25;
const MAZE_HEIGHT: usize = 25;

pub struct Maze {
    pub start_point: Position,
    pub goal: Position,
    pub tiles: Cell<[Tile; MAZE_HEIGHT * MAZE_WIDTH]>,
}
// origin is left top
impl Maze {
    pub fn flood_fill(&self) {
        let mut stack: Vec<Tile> = vec![];
        let start = self.get_tile_at(&self.goal);
        stack.push(start);
        while let Some(tile) = stack.pop() {
            let pos = &tile.position;
            if pos.x < MAZE_WIDTH && tile.right {
                let mut right = self.get_tile_at(&Position::new(pos.x + 1, pos.y));
                if right.value > tile.value + 1 {
                    right.value = tile.value + 1;
                    stack.push(right);
                }
            };
            if pos.x > 1 && tile.left {
                let mut left = self.get_tile_at(&Position::new(pos.x - 1, pos.y));
                if left.value > tile.value + 1 {
                    left.value = tile.value + 1;
                    stack.push(left);
                }
            };
            if pos.y < MAZE_HEIGHT && tile.lower {
                let mut lower = self.get_tile_at(&Position::new(pos.x, pos.y + 1));
                if lower.value > tile.value + 1 {
                    lower.value = tile.value + 1;
                    stack.push(lower);
                }
            };
            if pos.y > 1 && tile.upper {
                let mut upper = self.get_tile_at(&Position::new(pos.x, pos.y - 1));
                if upper.value > tile.value + 1 {
                    upper.value = tile.value + 1;
                    stack.push(upper);
                }
            };
        }
    }

    pub fn solve(&self)->Vec<Position>{
        todo!()
    }

    fn get_tile_at(&self, pos: &Position) -> Tile {
        self.tiles.get_mut()[pos.x-1+(pos.y-1)*MAZE_WIDTH]
    }

    
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}


pub struct Tile {
    pub position: Position,
    /// if upper is open
    pub upper: bool,
    pub lower: bool,
    pub left: bool,
    pub right: bool,
    /// cell value in flood fill algorithm
    pub value: u32,
}
