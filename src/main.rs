use std::{
    fmt::{self, Display},
    thread,
    time::Duration,
};

fn main() {
    let mut board = Board::new(10, 10);
    board.start();
    let mut block = Block::new();
    loop {
        //board.add_block(block.y, block.x);
        board.down();
        board.draw();
        block.down();
        if block.y == 5 {
            break;
        }
        thread::sleep(Duration::from_millis(1000));
    }
    board.end();
}

struct Block {
    x: usize,
    y: usize,
}

impl Block {
    fn new() -> Block {
        Block { x: 4, y: 1 }
    }
    fn down(&mut self) {
        self.y += 1;
    }
}

struct Board {
    cols: usize,
    rows: usize,
    board: Vec<i32>,
    block: (usize, usize),
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = self
            .board
            .iter()
            .enumerate()
            .fold("".to_string(), |acc, (pos, val)| {
                let text = if val == &0 {
                    " ".to_string()
                } else {
                    val.to_string()
                };
                if pos % self.cols == 0 {
                    acc + "\n" + &text
                } else {
                    acc + " " + &text
                }
            });
        write!(f, "{}", output)
    }
}

impl Board {
    fn add_block(&mut self, row: usize, col: usize) {
        self.board[self.cols * row + col] = 1;
    }

    fn down(&mut self) {
        self.board[self.cols * self.block.0 + self.block.1] = 0;
        self.block.0 += 1;
        self.board[self.cols * self.block.0 + self.block.1] = 1;
    }

    fn new(cols: usize, rows: usize) -> Board {
        Board {
            cols,
            rows,
            board: vec![0; cols * rows],
            block: (0, 0),
        }
    }

    fn start(&mut self) {
        hide_cursor();
        clear_screen();
        self.block = (0, 4);
        println!("{}", self);
    }

    fn end(&self) {
        show_cursor();
    }

    fn draw(&self) {
        clear_screen();
        println!("{}", self);
    }
}

fn hide_cursor() {
    print!("\x1B[?25l");
}

fn show_cursor() {
    print!("\x1B[?25h");
}

fn clear_screen() {
    print!("\x1Bc");
}

fn move_cursor(row: usize, col: usize) {
    print!("\x1B[{0};{1}H", row, col);
}
