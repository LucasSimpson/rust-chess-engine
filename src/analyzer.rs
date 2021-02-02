// use crate::board::{Board, ChessMove};
//
// use std::sync::{Mutex, Arc};
// use std::thread;
// use std::time::Duration;
//
// #[derive(Debug, Clone)]
// pub struct Analysis {
//     best_moves: Vec<Arc<Mutex<Option<(ChessMove, f32)>>>>,
//     stop: Arc<Mutex<bool>>,
// }
//
// impl Analysis {
//     pub fn new(n: usize) -> Analysis {
//         let mut bm = Vec::with_capacity(n);
//
//         for _i in 0..n {
//             bm.push(Arc::new(Mutex::new(None)))
//         }
//
//         Analysis {
//             best_moves: bm,
//             stop: Arc::new(Mutex::new(false)),
//         }
//     }
//
//     pub fn update_best_move(&self, i: usize, best_move: ChessMove, score: f32) {
//         let mut cm = self.best_moves[i].lock().unwrap();
//
//         *cm = {
//             if cm.is_none() {
//                 Some((best_move, score))
//             } else {
//                 cm.as_ref().map(|(cur_move, cur_score)| {
//                     if score >= *cur_score {
//                         (best_move, score)
//                     } else {
//                         (cur_move.clone(), *cur_score)
//                     }
//                 })
//             }
//         };
//     }
//
//     pub fn get_best_move(&self, i: usize) -> Option<(ChessMove, f32)> {
//         let cm = self.best_moves[i].lock().unwrap();
//         cm.clone()
//     }
//
//     pub fn stop(&self) {
//         let mut s = self.stop.lock().unwrap();
//         *s = true;
//     }
//
//     pub fn is_done(&self) -> bool {
//         *self.stop.lock().unwrap()
//     }
// }
//
// #[derive(Debug, Clone)]
// pub struct Node {
//     board: Board,
//     root_move: ChessMove,
//     coef: f32,
// }
//
// impl Node {
//     pub fn from_root(board: Board, cm: ChessMove, coef: f32) -> Node {
//         Node {
//             board,
//             root_move: cm,
//             coef
//         }
//     }
//
//
// }
//
// // fn thread_explore(id: String, board: Board, analysis: Analysis) {
// //
// //     if analysis.is_done() {
// //         echo_client::log(format!("T#{}: stop is set, quitting", id).as_str());
// //         return
// //     }
// //
// //     let coef = if board.is_whites_move() {
// //         1_f32
// //     } else {
// //         -1_f32
// //     };
// //
// //     // get all legal moves
// //     board.get_legal_moves()
// //         .into_iter()
// //
// //         // apply the move to a copied board and calc the score
// //         .map(|cm| (cm.clone(), board.clone().apply_move(&cm).score() * coef))
// //
// //         // some debug logging
// //         .map(|(cm, score)| {
// //             echo_client::log(format!("T#{}: \t{} => {}", id, cm.to_long_algebraic_notation(), score).as_str());
// //
// //             (cm, score)
// //         })
// //
// //         // update the analysis TODO calculate the best thread-local, then only set the best through the mutex
// //         .for_each(|(cm, score)| analysis.update_best_move(cm, score));
// //
// //
// //     let mut possible_moves = board.get_legal_moves();
// //
// //     // debug
// //     echo_client::log("Legal moves:");
// //     possible_moves.iter().for_each(|cm| echo_client::log(format!("\t{}", cm.to_long_algebraic_notation()).as_str()));
// //
// //     possible_moves.shuffle(&mut rand::thread_rng());
// //     possible_moves.pop()
// //         .map(|x| Ok((board, x)))
// //         .unwrap_or(Err(String::from("no possible moves")))
// //
// //     let score = board.apply_move(cm)
// //
// //     cm.map(|(_board, cm)| analysis.update_best_move(cm, score)).unwrap();
// // }
//
// pub(crate) fn find_best_move(board: &Board) -> std::result::Result<ChessMove, String> {
//
//
//     let coef = if board.is_whites_move() {
//         1_f32
//     } else {
//         -1_f32
//     };
//
//     // get the seed root moves
//     let root_moves: Vec<(ChessMove, f32)> = board.get_legal_moves()
//         .into_iter()
//
//         // apply the move to a copied board and calc the score
//         .map(|cm| (cm.clone(), board.clone().apply_move(&cm).score() * coef))
//         .collect();
//
//     // let analysis = Analysis::new(root_moves.len());
//     Err(String::from("unimplemented"))
//
//         // // some debug logging
//         // .map(|(cm, score)| {
//         //     echo_client::log(format!("T#{}: \t{} => {}", id, cm.to_long_algebraic_notation(), score).as_str());
//         //
//         //     (cm, score)
//         // })
//
//         // // update the analysis TODO calculate the best thread-local, then only set the best through the mutex
//         // .for_each(|(cm, score)| analysis.update_best_move(cm, score));
//
//
//
//
//     // // start some (1 for now) thread
//     // let board_clone = board.clone();
//     // let analysis_clone = analysis.clone();
//     // let handle = thread::spawn(move || {
//     //     thread_explore(String::from("1"), board_clone, analysis_clone)
//     // });
//     //
//     // // wait for a second or two or w/e
//     // thread::sleep(Duration::from_millis(200));
//     //
//     // // tell the thread to stop, grab the best move
//     // analysis.stop();
//     //
//     // // join threads
//     // handle.join().unwrap();
//
//     // analysis.get_best_move(0)
//     //     .map(|(cm, _score)| Ok(cm))
//     //     .unwrap_or(Err(String::from("No best move found")))
// }
//
//
//
//
//
//
//
// // // get a random chess move; super temporary
// // fn get_random_move(board: Board) -> std::result::Result<(Board, ChessMove), String> {
// //     let mut possible_moves = board.get_legal_moves();
// //
// //     // debug
// //     echo_client::log("Legal moves:");
// //     possible_moves.iter().for_each(|cm| echo_client::log(format!("\t{}", cm.to_long_algebraic_notation()).as_str()));
// //
// //     possible_moves.shuffle(&mut rand::thread_rng());
// //     possible_moves.pop()
// //         .map(|x| Ok((board, x)))
// //         .unwrap_or(Err(String::from("no possible moves")))
// // }