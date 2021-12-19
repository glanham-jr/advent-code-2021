use std::borrow::BorrowMut;
use std::fs::File;
use std::collections::hash_map::HashMap;
use std::collections::hash_map::Entry;
use std::io::{BufReader, BufRead};

// correct answer is 39902

#[derive(Copy, Clone)]
struct BingoPosition {
    value: u16,
    row: usize,
    column: usize,
    checked: bool,
}

#[derive(Clone)]
struct BingoGame {
    board: HashMap<u16, BingoPosition>,
    row_counts: [u16; 5],
    column_counts: [u16; 5],
    won: bool,
}
impl BingoGame {
    fn new() -> Self {
        return BingoGame {
            board: HashMap::with_capacity(25),
            row_counts: [0, 0, 0, 0, 0],
            column_counts: [0, 0, 0, 0, 0],
            won: false,
        };
    }
}

// fn pretty_print(game: &BingoGame) {
//     // Not super efficient but for debugging
//     let mut string: String = String::new();
//     let mut values = game.board
//                          .values()
//                          .cloned()
//                          .into_iter()
//                          .collect::<Vec<BingoPosition>>();
//     values.sort_by(|a, b| a.column.cmp(&b.column));
//     values.sort_by(|a, b| a.row.cmp(&b.row));

//     for row in 0..5 {
//         string += r#"["#;
//         for column in 0..5 {
//             let p = values[row * 5 + column];
//             // let str_checked = if p.checked { "X" } else { " " };
//             let str_checked = if p.checked { "X" } else { " " };
//             string += &format!("({:#02}, {})", p.value, str_checked).to_string();
//         }
//         string += r#"]"#;
//         string += "\n";
//     }
//     println!("----------\n{}----------\n", string);
// }

struct Bingo {
    moves: Vec<u16>,
    games: Vec<BingoGame>,
}

fn read_file() -> BufReader<File> {
    let file_path = "data/day4_bingo.txt";
    let file =  match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file: {:?}", e),
    };
    return BufReader::new(file);
}

fn str_to_u16(string: &String) -> Vec<u16> {
    return string
        .split(",")
        .into_iter()
        .map(|v| {
            return v.trim_end().parse::<u16>().unwrap();
        })
        .collect();
}

fn str_to_u16_bingo(game: &mut BingoGame, row: usize, string: &String) {
    for (i, s) in string.split(" ").filter(|s| *s != "\n" && !(*s).is_empty()).enumerate() {
        let value = s.trim_end().parse::<u16>().unwrap();
        game.board.insert(value, BingoPosition {
            row,
            value,
            column: i,
            checked: false,
        });
    }
}

fn read_bingo() -> Bingo {
    let mut reader = read_file();
    let mut buf = String::new();
    let mut bytes_read: usize = reader.read_line(&mut buf).unwrap();
    let moves: Vec<u16> = str_to_u16(&buf);
    buf.clear();

    // ignore empty lines
    reader.read_line(&mut buf).unwrap();
    buf.clear();

    let mut games: Vec<BingoGame> = Vec::new();
    while bytes_read != 0 {
        let mut game = BingoGame::new();

        reader.read_line(&mut buf).unwrap();
        str_to_u16_bingo(&mut game, 0, &buf);
        buf.clear();

        reader.read_line(&mut buf).unwrap();
        str_to_u16_bingo(&mut game, 1, &buf);
        buf.clear();

        reader.read_line(&mut buf).unwrap();
        str_to_u16_bingo(&mut game, 2, &buf);
        buf.clear();

        reader.read_line(&mut buf).unwrap();
        str_to_u16_bingo(&mut game, 3, &buf);
        buf.clear();

        reader.read_line(&mut buf).unwrap();
        str_to_u16_bingo(&mut game, 4, &buf);
        buf.clear();

        games.push(game);

        // ignore empty lines
        bytes_read = reader.read_line(&mut buf).unwrap();
        buf.clear();

    }

    return Bingo {
        moves,
        games,
    };
}

fn _bingo(bingo: &mut Bingo, win_break: usize) -> (BingoGame, u16) {
    println!("win_break: {}", win_break);
    let mut win_counter: usize = 0;
    for mv in bingo.moves.iter() {
        for g in bingo.games.iter_mut().filter(|g| !g.won) {
            match &mut g.board.borrow_mut().entry(*mv) {
                Entry::Occupied(po) => {
                    let p = po.get_mut();
                    (*p).checked = true;
                    // po.insert(*p);
                    g.row_counts[p.row] += 1;
                    g.column_counts[p.column] += 1;
                    if g.row_counts[p.row] == 5 {
                        win_counter += 1;
                        (*g).won = true;
                        println!("wincnt: {}", win_counter);
                        if win_counter >= win_break {
                            return (Clone::clone(g), *mv);
                        }
                    }
                    else if g.column_counts[p.column]  == 5 {
                        win_counter += 1;
                        (*g).won = true;
                        println!("wincnt: {}", win_counter);
                        if win_counter >= win_break {
                            return (Clone::clone(g), *mv);
                        }
                    }

                }
                _ => (),
            }
        }
    }
    panic!("No one won bingo :(");
}

pub fn p1_bingo() -> u16 {
    let bingo = &mut read_bingo();
    let (result, mv) = _bingo(bingo, 1); // only need to one 1 game
    let sum = result.board
                    .values()
                    .filter(|x| !x.checked)
                    .map(|x| x.value)
                    .sum::<u16>();

    return sum * mv;
}

pub fn p2_bingo() -> u16 {

    let bingo = &mut read_bingo();
    let (result, mv) = _bingo(bingo, bingo.games.len()); // must win all games
    let sum = result.board
                    .values()
                    .filter(|x| !x.checked)
                    .map(|x| x.value)
                    .sum::<u16>();

    return sum * mv;
}
