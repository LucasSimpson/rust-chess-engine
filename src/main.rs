use std::io::{self, Error, ErrorKind};

use crate::board::{Board, ChessMove};
use crate::v2::Manager;
use crate::echo_client::log;

mod echo_server;
mod echo_client;
mod board;
mod analyzer;
mod brute;
mod v2;

#[derive(Debug, Clone)]
pub struct State {
    board: Board,
    manager: Manager,
}

impl State {
    pub fn new() -> State {
        State {
            board: Board::new(),
            manager: Manager::new(),
        }
    }

    pub fn new_board(mut self) -> State {
        self.board = Board::new();
        return self
    }

    pub fn set_board(mut self, board: Board) -> State {
        self.board = board;
        return self
    }
}


// pub type CommandResult<T> = std::result::Result<T, CommandError>;

fn respond(message: &str) {
    echo_client::log(format!("Responding: {}", message).as_str());
    println!("{}", message)
}

fn io_error(message: &str) -> io::Result<State> {
    io::Result::Err(Error::new(ErrorKind::Other, message))
}


fn start_view_server(state: State) -> io::Result<State> {
    echo_server::start();
    io::Result::Ok(state)
}

fn handle_command_exit() -> io::Result<State> {
    echo_client::log("exiting");
    io_error("exiting")
}

fn handle_command_unknown(state: State, _command: &str) -> io::Result<State> {
    echo_client::log("command unknown");
    io::Result::Ok(state)
}

fn handle_command_position(mut state: State, parts: &Vec<&str>) -> io::Result<State> {
    if parts.len() < 2 {
        io_error("expected more args")
    } else {
        let (res, index) = if parts[1] == "startpos" {
            let res = Board::from_fen(["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", "w", "KQkq", "-", "0", "1"]);
            // let res = state.board.clone().reset_from_fen(["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", "w", "KQkq", "-", "0", "1"]);
            (res, 2)
        } else if parts[1] == "fen" {
            let mut arg: [&str; 6] = Default::default();
            arg.copy_from_slice(&parts[2..8]);
            let res = Board::from_fen(arg);
            // let res = state.board.clone().reset_from_fen(arg);
            (res, 8)
        } else {
            echo_client::log(format!("Unsupported position type: {}", parts[1]).as_str());
            return io_error("Unsupported position type")
        };

        res
            .and_then(|board| {
                state.board = board;

                if parts.len() < index + 2 {
                    return Ok(state)
                }

                let moves = &parts[index + 1..];

                let mut r_state = Ok(state);

                for str_move in moves {
                    r_state = r_state.and_then(move |mut state| {
                        let chess_move = ChessMove::from_long_algebraic_notation(str_move);
                        state.board = state.board.apply_move(&chess_move);
                        Ok(state)
                    });
                }
                r_state
            })
            .map(|board| io::Result::Ok(board))
            .map_err(|err| io_error(err.as_str()))
            .unwrap()
    }
}

fn handle_command_go(state: &mut State, _parts: &Vec<&str>) -> io::Result<()> {
    // TODO parse all the args
    // let args = parts &parts[1..];

    // for now just ask the board for best move, this gonna have to be heavily refactored
    // brute::find_best_move(&state.board)
    v2::find_best_move(state.board.clone(), &mut state.manager)
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

fn handle_command_playground(mut state: State) -> io::Result<State> {
    state.board = Board::from_fen(["rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR", "w", "KQkq", "-", "0", "1"]).unwrap();

    println!("{}", state.board.as_debug_string().as_str());

    io::Result::Ok(state)
}

fn handle_command_uci(state: State) -> io::Result<State> {
    respond("id name Cesac 0.1 ALPHA");
    respond("id author Lucas Simpson");

    // TODO sent options supported

    respond("uciok");
    io::Result::Ok(state)
}

fn handle_command_is_ready(state: State) -> io::Result<State> {
    respond("readyok");
    io::Result::Ok(state)
}

fn handle_command(mut state: State, command: &str) -> io::Result<State> {
    echo_client::log(format!("\n\n\nReceived command: {}", command).as_str());

    // split by spaces
    let parts: Vec<&str> = command.split(" ").collect();

    // if this fails its weird, just assume unknown
    if parts.len() == 0 {
        handle_command_unknown(state, command)
    } else if parts[0] == "exit" {
        handle_command_exit()
    } else if parts[0] == "server" {
        start_view_server(state)
    } else if parts[0] == "uci" {
        handle_command_uci(state)
    } else if parts[0] == "ucinewgame" {
        io::Result::Ok(state.new_board())
    } else if parts[0] == "isready" {
        handle_command_is_ready(state)
    } else if parts[0] == "position" {
        handle_command_position(state, &parts)
    } else if parts[0] == "go" {
        handle_command_go(&mut state, &parts).map(|_| state)
    } else if parts[0] == "stop" {
        handle_command_exit()
    } else if parts[0] == "t" {
        handle_command_playground(state)
    } else {
        handle_command_unknown(state, command)
    }
}

fn read_input(state: State) -> io::Result<()> {
    echo_client::log(state.board.as_debug_string().as_str());

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .and_then(|_| handle_command(state, buffer.trim()))
        .and_then(|state| read_input(state))
}

fn test(pos: &str) {
    handle_command(State::new(), pos)
        .and_then(|state| handle_command(state, "go"))
        .and_then(|state| read_input(state))
        .unwrap();
}

fn log_unwrap(res: io::Result<()>) {
    match res {
        Ok(_) => log("exited OK"),
        Err(err) => log(err.to_string().as_str())
    }
}

fn main() {
    // test("position startpos moves g1f3 a7a5 e2e4 h7h6 b1c3 c7c6 d2d4 h6h5 f3d2 h5h4 h2h3 h8h6 d2c4 h6e6 c1g5 g7g6 g5h4");
    // test("position startpos moves g1f3 a7a5 a2a4 a8a7 b1c3 a7a8 c3b5 c7c6 b5d4 c6c5 d4b3 e7e6 d2d4 c5d4 f3d4 e6e5 d4b5 f8b4 c1d2");
    // test("position startpos moves b1c3 a7a5 a2a4 a8a7 d2d4 a7a8 e2e4 a8a7 c3b5 a7a6 c1f4");


    let state = State::new();

    // actual command to play legit
    log_unwrap(read_input(state))

    // handle_command(state, "position startpos")
    //     .and_then(|state| handle_command(state, "go"))
    //     .and_then(|state| read_input(state))
    //     .unwrap();



    // // should be a draw with final move a4a5
    // handle_command(state, "position startpos moves e2e4 a7a6 d2d4 a6a5 b1c3 a5a4 f1d3 a4a3 b2b4 b8c6 d4d5 c6b4 d3c4 a8a7 c4b5 a7a5 h2h4 b7b6 g1e2 c8b7 c1d2 b7a6 d2c1 a6b5 e1f1 b5c4 c1e3 a5a6 d1d2 a6a7 h1h3 a7a8 d2d1 a8b8 d1b1 b4a6 g2g4 b8c8 e3d2 a6b8 d2f4 b8a6 c3b5 c4e2 f1e1 e2b5 h3e3 b5c4 e3g3 a6c5 b1c1 c5e4 a1b1 c4a2 g3h3 a2b1 h3d3 a3a2 c1b2 b6b5 b2d4 b1c2 d4e5 c2d3 e5d4 c7c6 d4a7 c8a8 a7e3 d8a5 e1d1 a5a4 d1e1 a4a5 e1d1 a5a4 d1e1 a4a5")
    //     .and_then(|state| handle_command(state, "go"))
    //     .and_then(|state| read_input(state))
    //     .unwrap();
}

