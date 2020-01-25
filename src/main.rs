//use std::io;
use std::time::{Duration,Instant};
use std::cmp::max;
use std::i32::{MIN,MAX};
fn main() {
    let mut root = Node::new2d(3);
    println!("Building tree...");
    let start = Instant::now();
    root.negamax(MIN+1,MAX-1,10);
    let duration = start.elapsed();
    println!("That took {:?}", duration);
    println!("Counting Children...");
    let start = Instant::now();
    let count = root.count_tree();
    let duration = start.elapsed();
    println!("That took {:?}", duration);
    println!("There are {} nodes saved in this tree.", count);
}

#[derive(Debug)]
struct Node<T>{
    state:T,
    children: Option<Vec<Node<T>>>

}

impl Node<Board2d>{
    fn negamax(&mut self, mut alpha:i32, beta:i32, depth:u8)->i32{
        // Base Cases for recursion
        if let Some(win) = self.state.winner {
            match win{
                Space::X | Space::O => return 1000,
                _=> return 0
            }
        }

        if depth == 0{return 0}

        match &mut self.children{
            // Test if we need to make Children
            None=>{
                self.make_children();
                // Call negamax on self after making children
                return self.negamax(alpha, beta, depth);
            },
            // Main code for negamax algorithm
            Some(children)=>{
                let mut value = MIN;
                for child in children{
                    value = max(value, -child.negamax(-beta, -alpha, depth-1));
                    alpha = max(alpha, value);
                    if alpha >= beta {break};
                }
                value
            }
        }
    }
    fn count_tree(&self)->i32{
        match &self.children{
            None=>1,
            Some(children)=>{
                let mut sum = 1;
                for child in children{
                    sum += child.count_tree();
                }
                sum
            }
        }
    }

    fn new2d(size: usize) -> Node<Board2d>{
        Node{
            state: Board2d::new(size),
            children: None
        }
    }
    // Makes all of the Children of a node
    fn make_children(&mut self){
        if let None = self.children {
            let mut children = vec![];
             for x in 0..self.state.size{
                for y in 0..self.state.size{
                    if let Space::Blank = self.state.board[x][y]{
                        children.push(
                            Node{
                                state: self.state.new_child((x,y)),
                                children: None
                            }
                        )
                    }
                }
            }
            // Sets a draw if there are no children
            if children.is_empty() {self.state.winner = Some(Space::Blank)}
            // Otherwise, put the vector in self.children
            else {self.children = Some(children)};
        }
    }
}
impl Node<Board3d>{
    fn new3d(size: usize)-> Node<Board3d>{
        Node{
            state: Board3d::new(size),
            children: None
        }
    }
}


#[derive(Debug)]
struct Board2d{
    board:[[Space; 4];4],
    last:Option<(usize,usize)>,
    size: usize,
    winner: Option<Space>
}
impl Board2d{
    fn new(size: usize) -> Board2d{
        Board2d{
            board:[[Space::Blank; 4];4],
            last:None,
            size,
            winner:None
        }
    }
    fn new_child(&self, pos: (usize,usize))->Board2d{
        let mark = match self.last{
            None => Space::X, // If no pervious turn, choose x
            Some(last) => match self.board[last.0][last.1]{
                // Choose the oposite of the last turn
                Space::X => Space::O,
                Space::O => Space::X,
                _=> panic!("Last turn is an invalid Space!")
            }
        };
        let mut result = Board2d{
            last:Some(pos),
            board: self.board,
            size: self.size,
            winner:None
        };
        result.board[pos.0][pos.1] = mark;
        result.check_win();
        result
    }
    fn check_win(&mut self){

        if let Some((x,y)) = self.last{
            if
            {    // Check horizontal
                let mut result = true;
                for n in 0..self.size{
                    if self.board[n][y] != self.board[x][y]{
                        result = false;
                        break
                    }
                }
                result
            }||
            {   // Check vertical
                let mut result = true;
                for n in 0..self.size{
                    if self.board[x][n] != self.board[x][y]{
                        result = false;
                        break
                    }
                }
                result
            }||
            {   // Check 1st diag
                if x != y{false}
                else{
                    let mut result = true;
                    for n in 0..self.size{
                        if self.board[n][n] != self.board[1][1]{
                            result = false;
                            break
                        }
                    }
                    result
                }
            }||
            {   // Check 2nd diagonal
                if x != self.size -1 -y{false}
                else{
                    let mut result = true;
                    for n in 0..self.size{
                        if self.board[n][self.size -1 -n] != self.board[x][y]{
                            result = false;
                            break
                        }
                    }
                    result
                }
            }
            {
                self.winner = Some(self.board[x][y]);
            }
        }
    }
}


#[derive(Debug)]
struct Board3d{
    board:[[[Space; 4];4];4],
    last:Option<(usize,usize,usize)>,
    size:usize,
    winner: Option<Space>
}
impl Board3d{
    fn new(size: usize) -> Board3d{
        Board3d{
            board:[[[Space::Blank; 4];4];4],
            last:None,
            size,
            winner:None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Space{
    Blank,
    X,
    O
}
