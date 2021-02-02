
use std::io::{self, Error, ErrorKind};
use crate::board::{Board, ChessMove};

mod echo_server;
mod echo_client;
mod board;
mod analyzer;
mod brute;


// #[derive(Debug)]
// pub struct CommandError {
//     details: String
// }
//
// impl std::result::Error for CommandError {
//     fn description(&self) -> &str {
//         self.details.as_str()
//     }
// }
//
// impl fmt::Display for CommandError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.description.fmt(f)
//     }
// }


// pub type CommandResult<T> = std::result::Result<T, CommandError>;

fn respond(message: &str) {
    echo_client::log(format!("Responding: {}", message).as_str());
    println!("{}", message)
}

fn io_error(message: &str) -> io::Result<Board> {
    io::Result::Err(Error::new(ErrorKind::Other, message))
}


fn start_view_server(board: Board) -> io::Result<Board> {
    echo_server::start();
    io::Result::Ok(board)
}

fn handle_command_exit() -> io::Result<Board> {
    echo_client::log("exiting");
    io_error("exiting")
}

fn handle_command_unknown(board: Board, _command: &str) -> io::Result<Board> {
    echo_client::log("command unknown");
    io::Result::Ok(board)
}

fn handle_command_position(board: Board, parts: &Vec<&str>) -> io::Result<Board> {
    if parts.len() < 2 {
        io_error("expected more args")
    } else {
        let (res, index) = if parts[1] == "startpos" {
            let res = board.reset_from_fen(["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", "w", "KQkq", "-", "0", "1"]);
            (res, 2)
        } else if parts[1] == "fen" {
            let mut arg: [&str; 6] = Default::default();
            arg.copy_from_slice(&parts[2..8]);
            let res = board.reset_from_fen(arg);
            (res, 8)
        } else {
            echo_client::log(format!("Unsupported position type: {}", parts[1]).as_str());
            return io_error("Unsupported position type")
        };

        res
            .and_then(|board| {
                if parts.len() < index + 2 {
                    return Ok(board)
                }

                let moves = &parts[index + 1..];
                let mut r_board = Ok(board);

                for str_move in moves {
                    r_board = r_board.and_then(move |b| {
                        let chess_move = ChessMove::from_long_algebraic_notation(str_move);
                        Ok(b.apply_move_into(chess_move))
                    });
                }

                r_board
            })
            .map(|board| io::Result::Ok(board))
            .map_err(|err| io_error(err.as_str()))
            .unwrap()
    }
}

fn handle_command_go(board: &Board, _parts: &Vec<&str>) -> io::Result<()> {
    // TODO parse all the args
    // let args = parts &parts[1..];

    // for now just ask the board for best move, this gonna have to be heavily refactored
    brute::find_best_move(board)
        .map(|chosen_move| {
            respond(format!("bestmove {}", chosen_move.to_long_algebraic_notation()).as_str());
            io::Result::Ok(())
        })
        .map_err(|err| io_error(err.as_str()))
        .unwrap()

    // optm
    //     .map(|chosen_move| {
    //         respond(format!("bestmove {}", chosen_move.to_long_algebraic_notation()).as_str());
    //         io::Result::Ok(board)
    //     })
    //     .map_err(|err| io_error(err))
    //     .unwrap()
}

fn handle_command_playground(_board: Board) -> io::Result<Board> {
    let mut board = Board::new();

    board = board
        .reset_from_fen(["rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR", "w", "KQkq", "-", "0", "1"])
        .unwrap();

    println!("{}", board.as_debug_string().as_str());

    io::Result::Ok(board)
}

fn handle_command_uci(board: Board) -> io::Result<Board> {
    respond("id name Cesac 0.1 ALPHA");
    respond("id author Lucas Simpson");

    // TODO sent options supported

    respond("uciok");
    io::Result::Ok(board)
}

fn handle_command_is_ready(board: Board) -> io::Result<Board> {
    respond("readyok");
    io::Result::Ok(board)
}

fn handle_command(board: Board, command: &str) -> io::Result<Board> {
    echo_client::log(format!("\n\n\nReceived command: {}", command).as_str());

    // split by spaces
    let parts: Vec<&str> = command.split(" ").collect();

    // if this fails its weird, just assume unknown
    if parts.len() == 0 {
        handle_command_unknown(board, command)
    } else if parts[0] == "exit" {
        handle_command_exit()
    } else if parts[0] == "server" {
        start_view_server(board)
    } else if parts[0] == "uci" {
        handle_command_uci(board)
    } else if parts[0] == "ucinewgame" {
        io::Result::Ok(Board::new())
    } else if parts[0] == "isready" {
        handle_command_is_ready(board)
    } else if parts[0] == "position" {
        handle_command_position(board, &parts)
    } else if parts[0] == "go" {
        handle_command_go(&board, &parts).map(|_| board)
    } else if parts[0] == "stop" {
        handle_command_exit()
    } else if parts[0] == "t" {
        handle_command_playground(board)
    } else {
        handle_command_unknown(board, command)
    }
}

fn read_input(board: Board) -> io::Result<()> {
    echo_client::log(board.as_debug_string().as_str());

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .and_then(|_| handle_command(board, buffer.trim()))
        .and_then(|board| read_input(board))
}

fn main() {
    let board = Board::new();

    read_input(board).unwrap()

    // // final move e1g1 is the king castling, board should show rook moved as well
    // handle_command(board, "position startpos moves e2e4 a7a6 d2d4 a6a5 a2a4 a8a7 c1e3 a7a8 d4d5 a8a6 f1a6 b8a6 b1c3 a6b8 g1f3 b7b6 f3e5 b8a6 d1h5 g7g6 h5h3 d7d6 e5c6 c8h3 g2h3 d8d7 e1c1")
    //     // .and_then(|board| handle_command(board, "go"))
    //     .and_then(|board| read_input(board))
    //     .unwrap();


    // handle_command(board, "position fen 7k/5ppp/8/8/8/8/8/RK6 b KQkq - 0 1")
    //     .and_then(|board| handle_command(board, "go"))
    //     .and_then(|board| read_input(board))
    //     .unwrap();
}

