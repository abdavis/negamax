//use std::io
use std::cmp::{max};
use std::i32::{MAX, MIN};
use std::time::{Instant};
use std::collections::HashMap;
const MAP_CUTOFF:usize = 4;
fn main() {
    let start = Instant::now();
    let mut root = Node::new2d(4);
    while match root.state.winner{ WinState::None => true, _ => false}{
        let start = Instant::now();
        root.calc_scores(21);
        let time = start.elapsed();
        root.print_scores();
        println!("{:?}", time);
        println!("States in Hash Map: {}", root.map.len());
        root = root.get_child();
        root.state.print();
    }
    println!("{}", match root.state.winner{
        WinState::X => "X Won!",
        WinState::O => "O Won!",
        WinState::Draw => "Cat!",
        WinState::None => "whoops, I don't know who won!"
});
    let end = start.elapsed();
    println!("Total Time: {:?}", end);
}



struct Node<T,S> {
    state: T,
    children: Vec<(T, i32)>,
    map: HashMap<S, i32>
}

impl Node<Board2d, [[Space; 5]; 5]> {
    // This function consumes self and returns a new child
    fn get_child(mut self) -> Self {
        if let Some(child) = self.children.pop() {
            let mut new_node = Node {
                state: child.0,
                children: vec![],
                map: HashMap::new()
            };
            new_node.make_children();
            new_node
        } else {
            panic!()
        }
    }
    fn calc_scores(&mut self, depth: u8) {
        for mut child in &mut self.children{
            child.1 = -child.0.negamax(MIN + 1, MAX - 1, depth, &mut self.map);
        }
        self.children.sort_unstable_by_key(|a| a.1);
    }

    fn print_scores(&self) {
        let mut out = String::new();
        for child in &self.children {
            out.push_str(&child.1.to_string());
            out.push_str(", ");
        }
        println!("{}", out);
    }

    fn new2d(size: usize) -> Node<Board2d, [[Space; 5]; 5]> {
        let mut result = Node {
            state: Board2d::new(size),
            children: vec![],
            map: HashMap::new()
        };
        result.make_children();
        result
    }
    // Makes all of the Children of a node
    fn make_children(&mut self) {
        // But only if the board is not in an ending state
        if let WinState::None = self.state.winner {
            let mut children = vec![];
            for y in 0..self.state.size {
                for x in 0..self.state.size {
                    if let Space::Blank = self.state.board[x][y] {
                        children.push((self.state.new_child((x, y)), 0));
                    }
                }
            }
            // Sets a draw if there are no children
            if children.is_empty() {
                self.state.winner = WinState::Draw
            }
            // Otherwise, put the vector in self.children
            else {
                self.children = children
            };
        }
    }
}
impl Node<Board3d, [[[Space; 4]; 4]; 4]> {
    fn new3d(size: usize) -> Node<Board3d, [[[Space; 4]; 4]; 4]> {
        Node {
            state: Board3d::new(size),
            children: vec![],
            map: HashMap::new()
        }
    }
}

#[derive(Copy, Clone)]
struct Board2d {
    board: [[Space; 5]; 5],
    last: Option<(usize, usize)>,
    size: usize,
    winner: WinState,
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
    fn negamax(&self, mut alpha: i32, beta: i32, depth: u8, map:&mut HashMap<[[Space; 5]; 5], i32>)
    -> i32 {
        // Check if we have already done the work for this node
        match map.get(&self.board) {
            Some(score) => return *score,
            None => ()
        }
        match self.winner{
            // Base cases for recursion
            WinState::X | WinState::O => -100 + i32::from(depth),
            WinState::Draw => 0,
            WinState::None =>{
                if depth == 0 { 0 }
                // Main part of negamax function
                else {
                    let mut value = MIN;
                    'outer: for y in 0..self.size {
                        for x in 0..self.size {
                            if self.board[x][y] == Space::Blank {
                                value = max(
                                    value,
                                    -self.new_child((x, y)).negamax(-beta, -alpha, depth - 1, map)
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
                        value=0;
                    }
                    map.insert(self.board, value);
                    value
                }
            }
        }
    }

    fn new(size: usize) -> Board2d {
        Board2d {
            board: [[Space::Blank; 5]; 5],
            last: None,
            size,
            winner: WinState::None,
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
            winner: WinState::None,
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
                self.winner = match self.board[x][y]{
                    Space::X => WinState::X,
                    Space::O => WinState::O,
                    _ => panic!("Invalid Board state")
                };
            }
        }
    }
}


struct Board3d {
    board: [[[Space; 4]; 4]; 4],
    last: Option<(usize, usize, usize)>,
    size: usize,
    winner: WinState
}
impl Board3d {
    fn new(size: usize) -> Board3d {
        Board3d {
            board: [[[Space::Blank; 4]; 4]; 4],
            last: None,
            size,
            winner: WinState::None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Space {
    Blank,
    X,
    O,
}
#[derive(Copy, Clone)]
enum WinState{
    None,
    X,
    O,
    Draw
}
