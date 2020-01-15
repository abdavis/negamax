//use std::io;

fn main() {
    let root = Node::new2d(3);
    println!("{:?}", root);
}

#[derive(Debug)]
struct Node<T>{
    state:T,
    children: Vec<Node<T>>

}
impl Node<Board2d>{
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

                        }
                    )
                }
            }
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
    fn next(&self, pos: (usize,usize))->Board2d{
        let mark = match self.last{
            None => Space::X, //if no pervious turn, choose x
            Some(last) => match self.board[last.0][last.1]{//choose the oposite of the last turn
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
        result
    }
    fn check_win(&mut self){
        //check horizontal
        let mut winner = true;
        if(
            for n in 0..self.size{
                if self.board[self.last.0][self.last.1] != self.board[n][self.last.1]{winner = false; break;}
            }
        ){

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

#[derive(Copy, Clone, Debug)]
enum Space{
    Blank,
    X,
    O
}
