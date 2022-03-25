// use crate::board::{Board, ChessMove};
// use crate::echo_client;
//
// use std::sync::{Mutex, Arc};
// use std::thread;
// use std::time::Duration;
// use std::sync::atomic::{AtomicUsize, Ordering};
//
// #[derive(Debug, Clone)]
// pub struct Analysis {
//     count: Arc<AtomicUsize>,
//     stop: Arc<Mutex<bool>>,
// }
//
// impl Analysis {
//     pub fn new() -> Analysis {
//         Analysis {
//             count: Arc::new(AtomicUsize::new(0)),
//             stop: Arc::new(Mutex::new(false)),
//         }
//     }
//
//     pub fn inc(&mut self, n: usize) {
//         self.count.fetch_add(n, Ordering::Relaxed);
//     }
//
//     pub fn count(&self) -> usize {
//         self.count.load(Ordering::Relaxed)
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
// fn find_best_rec(analysis: &mut Analysis, board: Board, coef: f32, depth: u32) -> Option<(ChessMove, f32)> {
//     let moves: Vec<ChessMove> = board.get_legal_moves();
//
//     if moves.len() == 0 {
//         // in a checkmate position
//         return None
//     }
//
//     let mut res: Vec<(ChessMove, f32)> = moves.into_iter()
//         .map(|cm| {
//             // for each move, do all possible opponents moves as well
//             let n_board = board.clone().apply_move(&cm.clone());
//             let opp_moves = n_board.get_legal_moves();
//
//             let res = if opp_moves.len() == 0 {
//                 // has been checkmated. Score of 1000
//                 (cm, coef * 1000_f32) // mul by coeff to get white-centric score
//             } else {
//                 let mut opp_res: Vec<(ChessMove, f32)> = opp_moves.into_iter()
//                     .map(|opp_cm| {
//                         let nn_board = n_board.clone().apply_move(&opp_cm);
//
//                         if depth == 0 {
//                             let score = coef * -1_f32 * nn_board.score();
//                             (cm.clone(), score)
//                         } else {
//                             let res = find_best_rec(analysis, nn_board, coef, depth - 1);
//
//                             // if return is None, it implies that we got checkmated
//                             match res {
//                                 None => (cm.clone(), coef * -1000_f32),
//                                 Some((_, s)) => (cm.clone(), coef * -1_f32 * s)
//                             }
//                         }
//
//                     })
//                     .collect();
//
//                 analysis.inc(opp_res.len());
//                 opp_res.sort_by(|(_b1, s1), (_b2, s2)| s2.partial_cmp(s1).unwrap());
//                 opp_res.remove(0)
//             };
//
//             res
//         })
//         .collect();
//
//     analysis.inc(res.len());
//     res.sort_by(|(_b1, s1), (_b2, s2)| (coef * s2).partial_cmp(&(coef * s1)).unwrap());
//
//     Some(res.remove(0))
// }
//
//
// pub(crate) fn find_best_move(board: &Board) -> std::result::Result<ChessMove, String> {
//     let coef = if board.is_whites_move() {
//         1_f32
//     } else {
//         -1_f32
//     };
//
//     let mut analysis = Analysis::new();
//
//     let clone_analysis = analysis.clone();
//     thread::spawn(move || {
//         monitor_thread(clone_analysis)
//     });
//
//     let res = match find_best_rec(&mut analysis, board.clone(), coef, 2) {
//         Some((cm, _score)) => Ok(cm),
//         None => Err(String::from("no move found"))
//     };
//
//     analysis.stop();
//     echo_client::log(format!("Monitor :: Analyzed {} moves total", analysis.count()).as_str());
//
//     return res
// }
//
// fn monitor_thread(analysis: Analysis) {
//     let mut prev: usize = analysis.count();
//     loop {
//         thread::sleep(Duration::from_secs(1));
//         let count = analysis.count();
//
//         if analysis.is_done() {
//             return
//         }
//
//         echo_client::log(format!("Monitor :: Analyzed {} moves, {} total", count - prev, count).as_str());
//         prev = count;
//     }
// }
//
//
