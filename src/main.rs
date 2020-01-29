//use std::io
use std::cmp::max;
use std::i32::{MAX, MIN};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
fn main() {
    let start = Instant::now();
    let mut root = Node::new2d(4);
    while match &root.children {
        Some(_v) => true,
        None => false,
    } {
        let start = Instant::now();
        root.calc_scores(20);
        let time = start.elapsed();
        root.print_scores();
        println!("{:?}", time);
        root = root.get_child();
        root.make_children();
        root.state.print();
    }
    println!("{:?}", root.state.winner);
    let end = start.elapsed();
    println!("Total Time: {:?}", end);
}

#[derive(Debug)]
struct Node<T> {
    state: T,
    children: Option<Vec<(T, i32)>>,
}

impl Node<Board2d> {
    // This function consumes self and returns a new child
    fn get_child(self) -> Self {
        if let Some(mut children) = self.children {
            if let Some(child) = children.pop() {
                Node {
                    state: child.0,
                    children: None,
                }
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
    fn calc_scores(&mut self, depth: u8) {
        if let Some(children) = &mut self.children {
            // Create a channel and mutex here so that threads can communicate
            let (tx, rx) = mpsc::channel();
            let mutex = Arc::new(Mutex::new(children.clone()));
            for _n in 0..5 {
                let tx = tx.clone();
                let mutex = mutex.clone();
                thread::spawn(move || loop {
                    let mut children = mutex.lock().unwrap();
                    let child = children.pop();
                    // Drop children here to let other threads lock on mutex
                    drop(children);
                    match child {
                        None => break,
                        Some(mut child) => {
                            child.1 = -child.0.negamax(MIN + 1, MAX - 1, depth);
                            tx.send(child).unwrap();
                        }
                    }
                });
            }
            drop(tx);
            let mut new_children = vec![];
            for message in rx {
                new_children.push(message);
            }
            *children = new_children;
            children.sort_unstable_by_key(|a| a.1);
        }
    }

    fn print_scores(&self) {
        if let Some(children) = &self.children {
            let mut out = String::new();
            for child in children {
                out.push_str(&child.1.to_string());
                out.push_str(", ");
            }
            println!("{}", out);
        }
    }

    fn new2d(size: usize) -> Node<Board2d> {
        let mut result = Node {
            state: Board2d::new(size),
            children: None,
        };
        result.make_children();
        result
    }
    // Makes all of the Children of a node
    fn make_children(&mut self) {
        // But only if the board is not in an ending state
        if let None = self.state.winner {
            if let None = self.children {
                let mut children = vec![];
                for y in 0..self.state.size {
                    for x in 0..self.state.size {
                        if let Space::Blank = self.state.board[x][y] {
                            children.push((self.state.new_child((x, y)), 0))
                        }
                    }
                }
                // Sets a draw if there are no children
                if children.is_empty() {
                    self.state.winner = Some(Space::Blank)
                }
                // Otherwise, put the vector in self.children
                else {
                    self.children = Some(children)
                };
            }
        }
    }
}
impl Node<Board3d> {
    fn new3d(size: usize) -> Node<Board3d> {
        Node {
            state: Board3d::new(size),
            children: None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Board2d {
    board: [[Space; 4]; 4],
    last: Option<(usize, usize)>,
    size: usize,
    winner: Option<Space>,
}
impl Board2d {
    fn print(&self) {
        let mut out = String::new();
        out.push('+');
        for _n in 0..self.size {
            out.push('-');
        }
        out.push_str("+\n");
        for y in 0..self.size {
            out.push('|');
            for x in 0..self.size {
                match self.board[x][y] {
                    Space::X => out.push('X'),
                    Space::O => out.push('O'),
                    _ => out.push(' '),
                }
            }
            out.push_str("|\n");
        }
        out.push('+');
        for _n in 0..self.size {
            out.push('-');
        }
        out.push_str("+\n");
        print!("{}", out);
    }
    fn negamax(&self, mut alpha: i32, beta: i32, depth: u8) -> i32 {
        // Base cases for recursion
        if let Some(winner) = self.winner {
            match winner {
                Space::X | Space::O => -100,
                _ => 0,
            }
        } else if depth == 0 {
            0
        }
        // Main part of negamax function
        else {
            let mut value = MIN;
            'outer: for y in 0..self.size {
                for x in 0..self.size {
                    if self.board[x][y] == Space::Blank {
                        value = max(
                            value,
                            -self.new_child((x, y)).negamax(-beta, -alpha, depth - 1),
                        );
                        alpha = max(alpha, value);
                        if alpha >= beta {
                            break 'outer;
                        };
                    }
                }
            }
            // Returns 0 if there are no child nodes and there is no winner
            if value == MIN {
                0
            } else {
                value
            }
        }
    }

    fn new(size: usize) -> Board2d {
        Board2d {
            board: [[Space::Blank; 4]; 4],
            last: None,
            size,
            winner: None,
        }
    }
    fn new_child(&self, pos: (usize, usize)) -> Board2d {
        let mark = match self.last {
            None => Space::X, // If no pervious turn, choose x
            Some(last) => match self.board[last.0][last.1] {
                // Choose the oposite of the last turn
                Space::X => Space::O,
                Space::O => Space::X,
                _ => panic!("Last turn is an invalid Space!"),
            },
        };
        let mut result = Board2d {
            last: Some(pos),
            board: self.board,
            size: self.size,
            winner: None,
        };
        result.board[pos.0][pos.1] = mark;
        result.check_win();
        result
    }
    fn check_win(&mut self) {
        if let Some((x, y)) = self.last {
            if {
                // Check horizontal
                let mut result = true;
                for n in 0..self.size {
                    if self.board[n][y] != self.board[x][y] {
                        result = false;
                        break;
                    }
                }
                result
            } || {
                // Check vertical
                let mut result = true;
                for n in 0..self.size {
                    if self.board[x][n] != self.board[x][y] {
                        result = false;
                        break;
                    }
                }
                result
            } || {
                // Check 1st diag
                if x != y {
                    false
                } else {
                    let mut result = true;
                    for n in 0..self.size {
                        if self.board[n][n] != self.board[x][y] {
                            result = false;
                            break;
                        }
                    }
                    result
                }
            } || {
                // Check 2nd diagonal
                if x != self.size - 1 - y {
                    false
                } else {
                    let mut result = true;
                    for n in 0..self.size {
                        if self.board[n][self.size - 1 - n] != self.board[x][y] {
                            result = false;
                            break;
                        }
                    }
                    result
                }
            }
            // End of if conditions, expression follows
            {
                self.winner = Some(self.board[x][y]);
            }
        }
    }
}

#[derive(Debug)]
struct Board3d {
    board: [[[Space; 4]; 4]; 4],
    last: Option<(usize, usize, usize)>,
    size: usize,
    winner: Option<Space>,
}
impl Board3d {
    fn new(size: usize) -> Board3d {
        Board3d {
            board: [[[Space::Blank; 4]; 4]; 4],
            last: None,
            size,
            winner: None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Space {
    Blank,
    X,
    O,
}
