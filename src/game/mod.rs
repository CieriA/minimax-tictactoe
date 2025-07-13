use crate::board::{Board, p_colored};
use crate::geomath::Point;
use std::cmp::{Ordering, max, min};
use std::{
    error::Error,
    io::{self, Write},
};

fn pass_to_play(board: &mut Board, turn: bool) -> Result<Option<String>, Box<dyn Error>> {
    println!("{board}");
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
        println!("{board}");
        println!("It's your turn");

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
            .possible_moves()
            .into_iter()
            .map(|p| (p, minimax(board, p, turn)))
            .reduce(|acc, e| match e.1.cmp(&acc.1) {
                Ordering::Greater if !turn => e,
                Ordering::Less if turn => e,
                _ => acc,
            })
            .unwrap()
            .0
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
        return if minimize { -10 } else { 10 }; // because the bot is `false`
    }
    if board.is_full() {
        return 0;
    }

    let initial = if minimize { i32::MIN } else { i32::MAX };

    board.possible_moves().into_iter().fold(initial, |acc, e| {
        let mm = minimax(&board, e, !minimize);
        if minimize { max(acc, mm) } else { min(acc, mm) }
    })
}

pub(crate) fn run() {
    print!("Wanna play against a bot? (y/n): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!(
        "To play, just write the number you see on the square where you want to place your piece."
    );

    let mut turn = true; // This states which player starts (`O`, false for bot)
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
                    println!("{board}");
                    println!("{msg}");
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
                    println!("{board}");
                    println!("{msg}");
                    break;
                }

                turn = !turn;
            }
        }
    }
}
