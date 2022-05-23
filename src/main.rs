use std::{env, io, process};
mod cell;
mod board;
use crate::cell::Cell;
use crate::board::{Board, WhichRule};

fn main() {
    let args: Vec<String> = env::args().collect();
    // if args.len() == 1 {
    //     println!("provide a number N to specify the NxN grid of skyscrapers!");
    //     return;
    // }

    let arg1 = args.get(1);
    let number_n;

    match arg1 {
        Some(arg) => number_n = arg.parse().unwrap(),
        None => {
            println!("provide a number N to specify the NxN grid of skyscrapers!");
            return;
        }
    }

    let mut board = Board::new_board(number_n);

    loop {
        board.print_board();
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("Failed to read line");
        let command = input_text.trim();
        parse_command(&command, &mut board);
    }
}

fn parse_command(input_command: &str, board: &mut Board) {
    let mut command = input_command;
    let mut comm_args = "";
    if input_command.starts_with("ru") {
        command = "ru";
        comm_args = &input_command[2..];
    } else if input_command.starts_with("rd") {
        command = "rd";
        comm_args = &input_command[2..];
    } else if input_command.starts_with("rr") {
        command = "rr";
        comm_args = &input_command[2..];
    } else if input_command.starts_with("rl") {
        command = "rl";
        comm_args = &input_command[2..];
    } else if input_command.starts_with("ok") {
        command = "ok";
        comm_args = &input_command[2..];
    } else if input_command.starts_with("restore") {
        command = "restore";
        comm_args = &input_command[7..];
    }
    match command {
        "exit" => process::exit(0),
        "help" => println!("Help command TODO!"),
        "ru" => board.update_rule_x(comm_args, WhichRule::Up),
        "rd" => board.update_rule_x(comm_args, WhichRule::Down),
        "rl" => board.update_rule_x(comm_args, WhichRule::Left),
        "rr" => board.update_rule_x(comm_args, WhichRule::Right),
        "ok" => board.check_ok_cell(comm_args),
        "c" | "check" => board.check_all(),
        "restore" => board.restore_cell(comm_args),
        _ => println!(
            "\"{}\" is not a valid command, type \"help\" to get valid commands",
            input_command
        ),
    }
}


// Test file = 
//               2                       2       2            
// 5 |1234567|1234567|1234567|1234567|1234567|1234567|1234567|  
// 5 |1234567|1234567|1234567|1234567|1234567|1234567|1234567|  
// 3 |1234567|1234567|1234567|1234567|1234567|1234567|1234567|  
//   |1234567|1234567|1234567|1234567|1234567|1234567|1234567| 5
// 2 |1234567|1234567|1234567|1234567|1234567|1234567|1234567|  
// 2 |1234567|1234567|1234567|1234567|1234567|1234567|1234567| 4
//   |1234567|1234567|1234567|1234567|1234567|1234567|1234567| 3
//                               2       3       2       5    
// Commands to run:
// cargo run 7
// ru020022
// rl553022
// rr0005043
// rd0002325
//
//