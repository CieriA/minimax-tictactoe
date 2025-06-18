use crate::board::{p_colored, Board};
use std::{io::{self, Write}, error::Error};
use std::cmp::{max, min, Ordering};
use crate::geomath::Point;

fn pass_to_play(board: &mut Board, turn: bool) -> Result<Option<String>, Box<dyn Error>> {
    println!("{}", board);
    println!("It's `{}`'s turn", p_colored(turn));
    
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    let input = Point::try_from(input.parse::<usize>()? - 1)?;
    
    if board[input].is_some() {
        return Err("Cell is not empty".into());
    }
    
    board[input] = Some(turn);
    
    if board.check_win(input) {
        return Ok(Some(format!("`{}`'s player won", p_colored(turn))));
    }
    if board.is_full() {
        return Ok(Some("It's a tie".to_string()));
    }
    
    Ok(None)
}

fn with_bot(board: &mut Board, turn: bool) -> Result<Option<String>, Box<dyn Error>> {
    let mov = if turn {
        println!("{}", board);
        println!("It's `{}`'s turn", p_colored(turn));


        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        let input = Point::try_from(input.parse::<usize>()? - 1)?;

        if board[input].is_some() {
            return Err("Cell is not empty".into());
        }

        input
    } else {
        board
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, square)| (Point::new(x, y), square)))
            .filter(|(p, square)| square.is_none())
            .map(|(p, square)| {
                (p, minimax(&board, p, turn))
            })
            .reduce(|acc, e| {
                match e.1.cmp(&acc.1) {
                    Ordering::Greater if !turn => e,
                    Ordering::Less if turn => e,
                    _ => acc,
                }
            }).unwrap().0
    };
    
    board[mov] = Some(turn);

    if board.check_win(mov) {
        return Ok(Some(format!("`{}`'s player won", p_colored(turn))));
    }
    if board.is_full() {
        return Ok(Some("It's a tie".to_string()));
    }
    
    Ok(None)
}

fn minimax(board: &Board, placement: Point, minimize: bool) -> i32 {
    let mut board = board.clone();
    
    board[placement] = Some(minimize);
    
    if board.check_win(placement) {
        return if minimize { -10 } else { 10 }; // bcuz the bot is `false`
    }
    if board.is_full() {
        return 0;
    }
    
    let initial = if minimize { i32::MIN } else { i32::MAX };
    
    board
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, square)| (Point::new(x, y), square)))
        .filter(|(_, square)| square.is_none())
        .map(|(p, _)| p)
        .fold(initial, |acc, e| {
            let mm = minimax(&board, e, !minimize);
            if minimize { max(acc, mm) } else { min(acc, mm) }
        })
}

pub(crate) fn run() {
    print!("Wanna play against a bot? (y/n): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    println!("To play, just write the number you see on the square where you want to place your piece.");

    let mut turn = true;
    let mut board = Board::default();
    
    match input.to_ascii_lowercase().trim() {
        "y" => {
            println!("Playing with a bot");
            loop {
                let Ok(ended) = with_bot(&mut board, turn) else {
                    println!("Invalid input.");
                    continue;
                };

                if let Some(msg) = ended {
                    println!("{}", board);
                    println!("{}", msg);
                    break;
                }

                turn = !turn;
            }
        }
        _ => {
            println!("Playing pass-to-play.");
            loop {
                let Ok(ended) = pass_to_play(&mut board, turn) else {
                    println!("Invalid input.");
                    continue;
                };

                if let Some(msg) = ended {
                    println!("{}", board);
                    println!("{}", msg);
                    break;
                }

                turn = !turn;
            }
        }
    }
}
