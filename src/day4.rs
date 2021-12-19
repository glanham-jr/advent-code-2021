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
}
impl BingoGame {
    fn new() -> Self {
        return BingoGame {
            board: HashMap::with_capacity(25),
            row_counts: [0, 0, 0, 0, 0],
            column_counts: [0, 0, 0, 0, 0],
        };
    }
}

fn pretty_print(game: &BingoGame) {
    // Not super efficient but for debugging
    let mut string: String = String::new();
    let mut values = game.board
                         .values()
                         .cloned()
                         .into_iter()
                         .collect::<Vec<BingoPosition>>();
    values.sort_by(|a, b| a.column.cmp(&b.column));
    values.sort_by(|a, b| a.row.cmp(&b.row));

    for row in 0..5 {
        string += r#"["#;
        for column in 0..5 {
            let p = values[row * 5 + column];
            // let str_checked = if p.checked { "X" } else { " " };
            let str_checked = if p.checked { "X" } else { " " };
            string += &format!("({:#02}, {})", p.value, str_checked).to_string();
        }
        string += r#"]"#;
        string += "\n";
    }
    println!("----------\n{}----------\n", string);
}

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
    println!("games {:?}", games.len());

    return Bingo {
        moves,
        games,
    };
}

fn _p1_bingo(bingo: &mut Bingo) -> (BingoGame, u16) {
    for mv in bingo.moves.iter() {
        println!("mv {:?}", mv);
        for g in bingo.games.iter_mut() {
            match &mut g.board.borrow_mut().entry(*mv) {
                Entry::Occupied(po) => {
                    let p = po.get_mut();
                    (*p).checked = true;
                    // po.insert(*p);
                    // println!("mv {}, val {}, row {}, col {}", *mv, p.value, p.row, p.column);
                    g.row_counts[p.row] += 1;
                    g.column_counts[p.column] += 1;
                    if g.row_counts[p.row] == 5 {
                        // pretty_print(g);
                        return (Clone::clone(g), *mv);
                    }
                    if g.column_counts[p.column]  == 5 {
                        // println!("WINNER");
                        pretty_print(g);
                        return (Clone::clone(g), *mv);
                    }

                }
                _ => (),
            }
        }
    }
    panic!("No one won bingo :(");
}

pub fn p1_bingo() -> u16 {
    // 562 is too low
    let bingo = &mut read_bingo();
    let (result, mv) = _p1_bingo(bingo);
    let sum = result.board
                    .values()
                    .filter(|x| !x.checked)
                    .map(|x| x.value)
                    .sum::<u16>();

    return sum * mv;
}

pub fn p2_bingo() -> u32 {
    let bingo = read_bingo();
    return bingo.games.len() as u32;
}
