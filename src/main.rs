#![allow(non_snake_case)]
#[warn(unused_assignments)]

use rand::Rng;
use std::string::ToString;
use std::fmt;
use std::collections::HashSet;

const WIDTH: i32 = 15;
const HEIGHT: i32 = 15;
const MINES: i32 = 15;
pub struct GameObject {
    _x: i32,
    _y: i32,
    _v: i32,
    _revealed: bool
}

pub struct GameBoard {
    _board: Vec<Vec<GameObject>>,
    _w: i32,
    _h: i32,
    _n: i32,
    _revealed: Vec<(i32, i32)>
}


impl ToString for GameObject {
    fn to_string(&self) -> String {
        return format!("(X: {}, Y: {}, Value: {}, Revealed?: {})", self._x, self._y, self._v, self._revealed);
    }
}


impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for i in 0..self._w {
            output = output + "  " + &i.to_string() + " ";
        }
        output = output + "\n ";
        for (i, row) in self._board.iter().enumerate() {
            if i == 0 {
                output = output + &"+---".repeat(self._w as usize) + "+" + "\n"; 
            }

            output = output + " |";

            for (j, col) in row.iter().enumerate() {
                let val = &col.get_v().to_string();
                if col._revealed {
                    if col.get_v() < 9 {
                        output = output + " " + val + " ";
                    } else {
                        output = output + " " + "X" + " ";
                    }
                } else {
                    output = output + " " + "-" + " ";
                }

                if j as i32 != self._w - 1 {
                    output = output + "|";
                }
            }

            output = output + "| " + &i.to_string() + "\n " + &"+---".repeat((self._w) as usize);
            output = output + "+\n"
            
        }
        write!(f, " {} ", output)
    }
}


pub fn reveal_all(_board: &mut GameBoard) {
    for y in 0.._board._h {
        for x in 0.._board._w {
            _board.mut_cell_at(x, y).reveal();
        }
    }
}


impl GameBoard {
    pub fn new(_w: i32, _h: i32, _n: i32) -> Self {
        let mut _board: Vec<Vec<GameObject>> = (0.._h).map(|y| (0.._w).map(|x| (GameObject::new(x, y, 0))).collect()).collect();
        GameBoard::generate_mines(&mut _board, _n);

        return Self { _board , _w , _h , _n , _revealed: Vec::new() };
    }
    
    pub fn get_cell_at(&self, _x: i32, _y: i32) -> &GameObject {
        return &self._board[_y as usize][_x as usize];
    }

    pub fn mut_cell_at(&mut self, _x: i32, _y: i32) -> &mut GameObject {
        return &mut self._board[_y as usize][_x as usize];
    }

    pub fn mut_board(&mut self) -> &mut Vec<Vec<GameObject>> {
        return &mut self._board;
    }
    
    pub fn cell_exist(&self, _x: i32, _y: i32) -> bool {
        return _x >= 0 && _x < self._w && _y >= 0 && _y < self._h
    }

    pub fn get_revealed(&self) -> &Vec<(i32, i32)> {
        return &self._revealed;
    }

    pub fn generate_mines(board: &mut Vec<Vec<GameObject>>, _n: i32) {
        let mut mines = HashSet::new();
        let mut rng = rand::thread_rng();
        
        while mines.len() < _n as usize {
            let rand_x: i32 = rng.gen_range(0..WIDTH);
            let rand_y = rng.gen_range(0..HEIGHT);
            let c = (rand_x, rand_y);
            mines.insert(c);
        }
    
        for mine in mines.iter() {
            GameObject::set_mine(board, mine.0, mine.1);
        }
    }

    pub fn reveal(&mut self, _x: i32, _y: i32) {
        if !self._revealed.iter().any(|&pos| pos == (_x, _y)) && self.cell_exist(_x, _y) {
            self.mut_cell_at(_x, _y).reveal();
            self._revealed.push((_x, _y));
        
            if self.get_cell_at(_x, _y).get_v() == 0 {
                self.reveal(_x , _y - 1);
                self.reveal(_x - 1, _y);
                self.reveal(_x + 1, _y);
                self.reveal( _x, _y + 1);
            }
        }
    }
}


impl GameObject {
    pub fn new(_x: i32, _y: i32, _v: i32) -> Self {
        return Self { _x, _y, _v, _revealed: false}
    }

    pub fn get_x(&self) -> i32 {
        self._x
    }

    pub fn get_y(&self) -> i32 {
        self._y
    }

    pub fn get_v(&self) -> i32 {
        self._v
    }

    pub fn is_revealed(&self) -> bool {
        self._revealed
    }

    pub fn reveal(&mut self) {
        self._revealed = true;
    }

    fn increment(&mut self) {
        self._v = self.get_v() + 1;
    }

    // increment all neighboring game objects' values by one
    pub fn get_neighbors(&self, board: &Vec<Vec<GameObject>>) -> Vec<GameObject> {
        let mut neighbors: Vec<GameObject> = Vec::new();

        for r in (self._y as i32 - 1)..=(self._y as i32 + 1) {
            for c in (self._x as i32 - 1)..=(self._x as i32 + 1) {
                if (c, r) != (self._x as i32, self._y as i32) && c >= 0 && c < WIDTH as i32 && r >= 0 && r < HEIGHT as i32 {
                    let neighbor: &GameObject = &board[r as usize][c as usize];
                    neighbors.push(GameObject { _x: (neighbor._x), _y: (neighbor._y), _v: (neighbor._v) , _revealed: (neighbor._revealed)})
                }
            }
        }

        return neighbors
    }

    fn set_mine(board: &mut Vec<Vec<GameObject>>, _x: i32, _y: i32) {
        if board[_y as usize][_x as usize].get_v() >= 9 {
            return;
        } else {
            board[_y as usize][_x as usize]._v = 9;
        }

        for n in board[_y as usize][_x as usize].get_neighbors(board).iter() {
            let neighbor = &mut board[n._y as usize][n._x as usize];
            if neighbor.get_v() > 8 {
                continue;
            }
            neighbor.increment();
        }
    }
}

pub fn clear_screen() {
    println!("{}[2J", 27 as char);
}

fn main() {
    let mut game_board: GameBoard = GameBoard::new(WIDTH, HEIGHT, MINES);
    
    let mut game_over = false;
    println!("{}", "-".repeat(60));
    println!("Welcome to Minesweeper!\nCreated in Rust by Blake Moody (github.com/theswiginator)");
    println!("{}", "-".repeat(60));
    println!("{game_board}");
    while !game_over {
        let mut line = String::new();
        println!("Enter a cell to be revealed: ");
        let _c = std::io::stdin().read_line(&mut line).unwrap();
        // declare a row and column variable to store integer values
        let mut row: i32 = 0;
        let mut col: i32 = 0;

        // parse the input string into two integers
        let mut iter = line.split_whitespace();
        
        match iter.next() {
            Some(s) => {
                match s.parse::<i32>() {
                    Ok(n) => col = n,
                    Err(_) => {
                        clear_screen();
                        println!("{game_board}");
                        println!("Error: Invalid input");
                        continue;
                    },
                }
            }
            None => {
                clear_screen();
                println!("{game_board}");
                println!("Error: Invalid input");
                continue;
            },
        }
        match iter.next() {
            Some(s) => {
                match s.parse::<i32>() {
                    Ok(n) => row = n,
                    Err(_) => {
                        clear_screen();
                        println!("{game_board}");
                        println!("Error: Invalid input");
                        continue;
                    },
                }
            }
            None => {
                clear_screen();
                println!("{game_board}");
                println!("Error: Invalid input");
                continue;
            },
        }

        if col < WIDTH && col >= 0 && row < HEIGHT && row >= 0 {
            // Reveal cell, as well as neighboring cells
            game_board.reveal(col, row);  // Flood-fill algorithm

            // Clear the screen
            clear_screen();

            // Checks if the player has struck a mine
            if game_board.get_cell_at(col, row).get_v() > 8 {
                // Clear the screen
                clear_screen();

                // Trigger game over
                game_over = true;
            }

            // Checks if the player has won
            if game_board.get_revealed().len() as i32 == WIDTH * HEIGHT - MINES {
                // Clear the screen
                clear_screen();

                // Trigger game over
                game_over = true;
            }
        } else {
            clear_screen();
            println!("{game_board}");
            println!("Given coordinates don't exist on this board!");
            continue;
        }

        // Print the game board
        println!("{game_board}");
    }

    println!("Game over!");
    
}
