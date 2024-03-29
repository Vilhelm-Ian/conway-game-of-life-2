use bevy::prelude::*;

#[derive(Component)]
pub struct Game {
    pub board: Vec<Vec<i32>>,
    pub live_cells: Vec<Cordinates>,
}

impl Game {
    pub fn new(height: usize, width: usize, live_cells: Vec<Cordinates>) -> Self {
        let board = fill_board(height, width, &live_cells);
        Self { board, live_cells }
    }

    fn print_board(&self) -> String {
        let mut result = String::from("");
        for row in self.board.iter() {
            for element in row {
                if element == &0 {
                    result += " ";
                } else {
                    result += "#";
                }
            }
            result += "\n"
        }
        result
    }

    fn generate_from_string(input: &str) -> Self {
        let splited_input: Vec<Vec<char>> =
            input.split('\n').map(|x| x.chars().collect()).collect();
        let mut board = vec![vec![]];
        let mut live_cells = vec![];
        for (y, row) in splited_input.iter().enumerate() {
            for (x, element) in row.iter().enumerate() {
                let board_length = board.len() - 1;
                match element {
                    ' ' => board[board_length].push(0),
                    '#' => {
                        board[board_length].push(1);
                        live_cells.push(Cordinates::new(y as i32, x as i32))
                    }
                    _ => panic!("invalid character at y:{:?}, x:{:?}", x, y),
                }
            }
            board.push(vec![]);
        }
        Self { board, live_cells }
    }

    pub fn update_board(&mut self) {
        let mut new_live_cells = vec![];
        let mut checked_cells = vec![];
        for cell in &self.live_cells {
            for y in cell.y - 1..cell.y + 2 {
                for x in cell.x - 1..cell.x + 2 {
                    if checked_cells.contains(&Cordinates::new(y, x))
                        || !is_valid_index(y, x, &self.board)
                    {
                        continue;
                    }
                    checked_cells.push(Cordinates::new(y, x));
                    let mut new_cell = if self.live_cells.contains(&Cordinates::new(y, x)) {
                        Cell::new(y, x, State::Alive)
                    } else {
                        Cell::new(y, x, State::Dead)
                    };
                    new_cell.find_neighbours(&self.board);
                    new_cell.update_cell();
                    match new_cell.state {
                        State::Alive => new_live_cells.push(new_cell.cordinates),
                        State::Dead => (),
                    }
                }
            }
        }
        self.board = fill_board(self.board.len(), self.board[0].len(), &new_live_cells);
        self.live_cells = new_live_cells;
    }

    pub fn set_alive(&mut self, y: usize, x: usize) {
        self.live_cells.push(Cordinates::new(y as i32, x as i32));
        self.board[y][x] = 1;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Component)]
pub struct Cell {
    cordinates: Cordinates,
    neighbours: i32,
    state: State,
}

impl Cell {
    fn new(y: i32, x: i32, state: State) -> Self {
        Self {
            cordinates: Cordinates::new(y, x),
            state,
            neighbours: 0,
        }
    }
    fn update_cell(&mut self) {
        match self.state {
            State::Alive => {
                if self.neighbours != 2 && self.neighbours != 3 {
                    self.state = State::Dead
                }
            }
            State::Dead => {
                if self.neighbours == 3 {
                    self.state = State::Alive
                }
            }
        }
    }

    fn find_neighbours(&mut self, board: &Vec<Vec<i32>>) {
        self.neighbours = 0;
        for y in self.cordinates.y - 1..self.cordinates.y + 2 {
            for x in self.cordinates.x - 1..self.cordinates.x + 2 {
                if (x == self.cordinates.x && y == self.cordinates.y)
                    || !is_valid_index(y, x, board)
                {
                    continue;
                }
                self.neighbours += board[y as usize][x as usize]
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Alive,
    Dead,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cordinates {
    x: i32,
    y: i32,
}

impl Cordinates {
    fn new(y: i32, x: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let mut game = Game::new(4, 4, vec![]);
    game.set_alive(1, 1);
    game.set_alive(2, 2);
    game.set_alive(3, 3);

    println!("--------");
    println!("{}", game.print_board());
    game.update_board();
    println!("--------");
    println!("{}", game.print_board());
    println!("Hello, world!");
}

fn fill_board(height: usize, width: usize, live_cells: &Vec<Cordinates>) -> Vec<Vec<i32>> {
    let mut board = vec![vec![0; width]; height];
    for cell in live_cells {
        board[cell.y as usize][cell.x as usize] = 1;
    }
    board
}

fn is_valid_index(y: i32, x: i32, board: &Vec<Vec<i32>>) -> bool {
    if y < 0 || y >= board.len() as i32 {
        return false;
    }
    if x < 0 || x >= board[0].len() as i32 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_to_vertical() {
        let mut game = Game::new(4, 4, vec![]);
        game.set_alive(1, 1);
        game.set_alive(1, 2);
        game.set_alive(1, 3);
        println!("{}", game.print_board());
        game.update_board();
        println!("{}", game.print_board());
        assert_eq!(1, game.board[0][2]);
        assert_eq!(1, game.board[1][2]);
        assert_eq!(1, game.board[2][2]);
    }
}
