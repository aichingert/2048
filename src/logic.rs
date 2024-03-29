// 2048 - Terminal, 12/03/2023
// Logic: All functions needed for the game a stored here
// (c) aichingert

use crate::direction::Direction;
use rand::prelude::*;

pub struct Game;

impl Game {
    pub fn start(board: &mut super::Board) -> u16 {
        Game::new(board);
        0
    }

    fn merge(board: &mut super::Board, dir: &Direction) {
        match dir {
            Direction::Up | Direction::Left => for i in 1..board.len() {
                    for j in 0..board.len() {
                        if dir == &Direction::Up && board[i-1][j] == board[i][j] {
                            board[i-1][j] *= 2;
                            board[i][j] = 0;
                        } else if dir == &Direction::Left && board[j][i-1] == board[j][i] {
                            board[j][i-1] *= 2;
                            board[j][i] = 0;
                        }
                    }},
            Direction::Down | Direction::Right => for i in 1..board.len() {
                    for j in 0..board.len() {
                        if dir == &Direction::Down && board[board.len()-i][j] == board[board.len()-i-1][j] {
                            board[board.len()-i][j] *= 2;
                            board[board.len()-i-1][j] = 0;
                        } else if dir == &Direction::Right && board[j][board.len()-i] == board[j][board.len()-i-1] {
                            board[j][board.len()-i] *= 2;
                            board[j][board.len()-i-1] = 0;
                        }
                    }},
            Direction::Invalid => (),
        }
    }

    fn push(board: &mut super::Board, dir: &Direction) {
        match dir {
            Direction::Up | Direction::Left => {
                for i in 1..board.len() {
                    for j in 0..board.len() {
                        let mut y: usize = i;
                        while y > 0 {
                            if dir == &Direction::Up {
                                if board[y-1][j] != 0 { break; }
                                board[y-1][j] = board[y][j];
                                board[y][j] = 0;
                            } else {
                                if board[j][y-1] != 0 { break; }
                                board[j][y-1] = board[j][y];
                                board[j][y] = 0;
                            }
                            y -= 1;
                        }
                    }
                }
            },
            Direction::Down | Direction::Right => {
                for i in 2..=board.len() {
                    for j in 0..board.len() {
                        let mut y: usize = board.len() - i;
                        while y < board.len()-1 {
                            if dir == &Direction::Down {
                                if board[y+1][j] != 0 { break; }
                                board[y+1][j] = board[y][j];
                                board[y][j] = 0;
                            } else {
                                if board[j][y+1] != 0 { break; }
                                board[j][y+1] = board[j][y];
                                board[j][y] = 0;
                            }
                            y += 1;
                        }
                    }
                }
            },
            Direction::Invalid => (),
        }
    }

    fn new(board: &mut super::Board) {
        let mut open: Vec<(usize,usize)> = Vec::new();
        let mut rnd = rand::thread_rng();

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == 0 { open.push((i,j)); }
            }
        }

        if open.len() == 0 {
            return;
        }

        let new: u16 = match rnd.gen_range(0..10) {
            9 => 4,
            _ => 2,
        };

        open.shuffle(&mut rnd);
        board[open[0].0][open[0].1] = new;
    }

    pub fn update(board: &mut super::Board, dir: &Direction) -> Option<u16> {
        let clone = board.clone();
        Game::push(board, dir);
        Game::merge(board, dir);
        Game::push(board, dir);

        if Game::is_lost(board) {
            return None;
        } else if Game::is_different(board, &clone) {
            Game::new(board);
        }

        Some(board
            .iter()
            .map(|sl| 
                sl
                    .iter()
                    .map(|e| e)
                .sum::<u16>())
            .sum::<u16>())
    }

    fn is_lost(board: &super::Board) -> bool {
        let mut clone = board.clone();
        let dirs: &[Direction] = &[Direction::Down, Direction::Up, Direction::Left, Direction::Right];

        for dir in dirs {
            Game::merge(&mut clone, dir);
            Game::push(&mut clone, dir);
        
            if Game::is_different(board, &clone) {
                return false;
            }
        }

        true
    }

    fn is_different(board: &super::Board, clone: &super::Board) -> bool {
        for i in 0..clone.len() {
            if clone[i] != board[i] {
                return true;
            }
        }

        false
    }
}
