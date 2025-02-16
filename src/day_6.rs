use std::collections::HashSet;


#[derive(Copy, Clone, Debug)]
#[derive(Eq, Hash, PartialEq)]
enum Face {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}


#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct BoardPos{
    row: usize,
    col: usize,
}

#[derive(Copy, Clone, Debug)]
#[derive(Eq, Hash, PartialEq)]
enum PathLoc{
    Map(BoardPos),
    OffGrid,
}

#[derive(Copy, Clone, Debug)]
#[derive(Eq, Hash, PartialEq)]
struct BoardMeta {
    curr_pos: PathLoc,
    curr_char : Face,
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in board.iter() {
        for col in row.iter() {
            print!("  {}", col);
        }
        println!("");
    }
}

fn get_next_loc(board: &Vec<Vec<char>>, board_meta: &mut BoardMeta) {
    if let PathLoc::Map(board_pos) = board_meta.curr_pos {
        let x = board_pos.col;
        let y = board_pos.row;
        let direction = board_meta.curr_char;

        let (nx, ny, mut ndir) = match direction {
            Face::NORTH => (x as i32, y as i32 - 1, Face::NORTH),
            Face::SOUTH => (x as i32, y as i32 + 1, Face::SOUTH),
            Face::EAST =>  (x as i32 + 1, y as i32, Face::EAST),
            Face::WEST =>  (x as i32 - 1, y as i32, Face::WEST),
        };

        if ny < 0 || ny >= board.len() as i32 || nx < 0 || nx >= board.get(0).unwrap().len() as i32 {
            *board_meta = BoardMeta {
                curr_pos: PathLoc::OffGrid,
                curr_char: ndir,
            };
            return;
        }
        let mut nx = nx as usize;
        let mut ny = ny as usize;
        if board[ny][nx] == '#' {
            nx = x;
            ny = y;
            ndir = match direction {
                Face::NORTH => Face::EAST,
                Face::SOUTH => Face::WEST,
                Face::EAST => Face::SOUTH,
                Face::WEST => Face::NORTH,
            }
        }

        *board_meta = BoardMeta {
            curr_pos: PathLoc::Map(BoardPos { row: ny, col: nx }),
            curr_char: ndir,
        };
    } 
}

fn part_1(mut board: Vec<Vec<char>>, start_pos: PathLoc) {
    let mut visited_locations: HashSet<BoardPos> = HashSet::new();
    if let PathLoc::Map(board_pos) = start_pos {
        visited_locations.insert(board_pos);
    }

    let mut board_meta = BoardMeta {
        curr_pos: start_pos,
        curr_char: Face::NORTH,
    };

    loop {
        let curr_pos = board_meta.curr_pos.clone();
        get_next_loc(&board, &mut board_meta);
        if let PathLoc::Map(board_pos) = board_meta.curr_pos {
            let x = board_pos.col;
            let y = board_pos.row;
            let arrow = match board_meta.curr_char {
                Face::NORTH => '^',
                Face::SOUTH => 'V',
                Face::EAST => '>',
                Face::WEST => '<',
            };

            board[y][x] = arrow;
            if let PathLoc::Map(BoardPos { row, col }) = curr_pos {
                board[row][col] = 'X';
            }

            visited_locations.insert(board_pos);
        } else {
            let result_1 = visited_locations.len();
            println!("Result 1: {result_1}");
            break;
        }
    }
}

fn part_2(board: Vec<Vec<char>>, start_pos: PathLoc) {
    let mut result_2 = 0;

    for row_idx in 0..board.len() {
        for col_idx in 0..board.get(0).unwrap().len() {
            if let PathLoc::Map(BoardPos { row, col }) = start_pos {
                if row == row_idx && col == col_idx {
                    continue;
                }
            }
            let mut visited_locations: HashSet<BoardMeta> = HashSet::new();
            let mut scratch_board = board.clone();
            let mut board_meta = BoardMeta {
                curr_pos: start_pos,
                curr_char: Face::NORTH,
            };

            scratch_board[row_idx][col_idx] = '#';
            loop {
                let curr_pos = board_meta.curr_pos.clone();
                get_next_loc(&scratch_board, &mut board_meta);
                if let PathLoc::Map(board_pos) = board_meta.curr_pos {
                    let x = board_pos.col;
                    let y = board_pos.row;
                    let arrow = match board_meta.curr_char {
                        Face::NORTH => '^',
                        Face::SOUTH => 'V',
                        Face::EAST => '>',
                        Face::WEST => '<',
                    };
        
                    scratch_board[y][x] = arrow;
                    if let PathLoc::Map(BoardPos { row, col }) = curr_pos {
                        scratch_board[row][col] = 'X';
                    }

                    if visited_locations.contains(&board_meta) {
                        result_2 += 1;
                        break;
                    }
        
                    visited_locations.insert(board_meta.clone());
                } else {
                    break;
                }
            }
        }
    }

    println!("Result 2: {result_2}");

}

pub fn day_6(){
    let file: &str = include_str!("inputs/day_6.txt");
    let mut board = Vec::new();
    let mut start_pos: PathLoc = PathLoc::OffGrid;
    for (line_idx, line) in file.lines().enumerate() {
        let mut row = Vec::new();
        for (char_idx, char) in line.chars().enumerate() {
            if char == '^' {
                start_pos = PathLoc::Map(BoardPos{row: line_idx, col: char_idx});
            }
            row.push(char);
        }
        board.push(row);
    }
    let is_debug = false;
    if is_debug {
        print_board(&board);
    }
    part_1(board.clone(), start_pos);
    part_2(board.clone(), start_pos);
}
