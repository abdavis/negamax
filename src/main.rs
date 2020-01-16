//use std::io;
use std::time::{Duration,Instant};

fn main() {
    let mut root = Node::new2d(3);
    println!("Building tree...");
    let start = Instant::now();
    root.build_tree();
    let duration = start.elapsed();
    println!("That took {:?}", duration);
    println!("Counting Children...");
    let start = Instant::now();
    let count = root.count_tree();
    let duration = start.elapsed();
    println!("That took {:?}", duration);
    println!("There are {} leaf nodes (possible games) in this tree.", count);
}

#[derive(Debug)]
struct Node<T>{
    state:T,
    children: Vec<Node<T>>

}
impl Node<Board2d>{
//This method is ineffecient, meant for testing purposes
    fn build_tree(&mut self){
        match self.state.winner{
            Some(_v)=> return,
            None =>{
                self.make_children();
                for n in 0..self.children.len(){
                    self.children[n].build_tree();
                }
            }
        }
    }

    fn count_tree(&self)->i32{
        if self.children.len() == 0{
            return 1
        }
        let mut sum = 0;
        for n in 0..self.children.len(){
            sum += self.children[n].count_tree();
        }
        sum
    }

    fn new2d(size: usize) -> Node<Board2d>{
        Node{
            state: Board2d::new(size),
            children: vec![]
        }
    }
    fn make_children(&mut self){
         for x in 0..self.state.size{
            for y in 0..self.state.size{
                if let Space::Blank = self.state.board[x][y]{
                    self.children.push(
                        Node{
                            state: self.state.new_child((x,y)),
                            children: vec![]
                        }
                    )
                }
            }
        }

        if self.children.len() == 0{ //if there are no children, then this node is a tie.
            self.state.winner = Some(Space::Blank);
        }
    }
}
impl Node<Board3d>{
    fn new3d(size: usize)-> Node<Board3d>{
        Node{
            state: Board3d::new(size),
            children: vec![]
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
            None => Space::X, //if no pervious turn, choose x
            Some(last) => match self.board[last.0][last.1]{
                //choose the oposite of the last turn
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
            {    //check horizontal
                let mut result = true;
                for n in 0..self.size{
                    if self.board[n][y] != self.board[x][y]{
                        result = false;
                        break
                    }
                }
                result
            }||
            {   //check vertical
                let mut result = true;
                for n in 0..self.size{
                    if self.board[x][n] != self.board[x][y]{
                        result = false;
                        break
                    }
                }
                result
            }||
            {   //check 1st diag
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
            {   //check 2nd diagonal
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
