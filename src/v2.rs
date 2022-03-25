use std::cell::{RefCell, RefMut, Ref};
use std::collections::{HashMap, VecDeque, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::board::{Board, ChessMove};
use crate::echo_client::log;
use std::any::Any;
use std::{cmp, mem};
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
struct Node {
    board: Board,
    best_score: (Option<ChessMove>, f32),  // best known move from this node

    explored: bool,  // whether all children have been visited or not
    is_valid: bool,  // either board is illegal state OR no known valid path to get here

    parents: Vec<(ChessMove, Rc<RefCell<Node>>)>,  // (move to get from parent to here, parent)
    children: Vec<(ChessMove, Rc<RefCell<Node>>)>  // (move to get from here to child, child)
}

impl Node {
    pub fn new(board: Board) -> Node {
        Node{
            board: board,
            is_valid: false,
            best_score: (None, 0.0),
            explored: false,
            parents: vec![],
            children: vec![]
        }
    }

    pub fn update_score(&mut self, score: f32, cm: &ChessMove, depth: i32) {
        let mut update_parents = false;

        if self.board.is_whites_move() {
            // whites move, look for high score
            if self.best_score.0.is_none() || score > self.best_score.1 {
                self.best_score.0 = Some(cm.clone());
                self.best_score.1 = score;
                update_parents = true;
            } else {
                // if the new score move is the same as the current score move, then we
                // have to find the new best move, because we may no longer be the best

                // check if the move is the same as the current best
                let same_move = self.best_score.0.clone().map(|bcm| return bcm == *cm).unwrap_or(false);
                if same_move {
                    // find the new best move
                    let mut current_best: Option<(&ChessMove, f32)> = None;
                    for (cm, child) in self.children.iter() {
                        let n = match child.try_borrow() {
                            Err(_) => continue,
                            Ok(n) => n
                        };

                        current_best = current_best.map(|(best_move, best_score)| {
                            if n.best_score.1 >= best_score {
                                Some((cm, n.best_score.1))
                            } else {
                                Some((best_move, best_score))
                            }
                        }).unwrap_or(Some((cm, n.best_score.1)));
                    }
                    self.best_score = current_best.map(|(cm, s)| (Some(cm.clone()), s)).unwrap_or((None, 0 as f32));
                }
            }
        } else {
            // blacks move, look for low score
            if self.best_score.0.is_none() || score < self.best_score.1 {
                self.best_score.0 = Some(cm.clone());
                self.best_score.1 = score;
                update_parents = true;
            } else {
                // if the new score move is the same as the current score move, then we
                // have to find the new best move

                // check if the move is the same as the current best
                let same_move = self.best_score.0.clone().map(|bcm| return bcm == *cm).unwrap_or(false);
                if same_move {
                    // find the new best move
                    let mut current_best: Option<(&ChessMove, f32)> = None;
                    for (cm, child) in self.children.iter() {
                        let n = match child.try_borrow() {
                            Err(_) => continue,
                            Ok(n) => n
                        };

                        current_best = current_best.map(|(best_move, best_score)| {
                            if n.best_score.1 <= best_score {
                                Some((cm, n.best_score.1))
                            } else {
                                Some((best_move, best_score))
                            }
                        }).unwrap_or(Some((cm, n.best_score.1)));
                    }
                    self.best_score = current_best.map(|(cm, s)| (Some(cm.clone()), s)).unwrap_or((None, 0 as f32));
                }
            }
        }

        if update_parents {
            for rp in self.parents.iter_mut() {
                // if we have a loop in the graph, then the borrow will fail. This is fine since
                // we've therefore already visited the node.
                match rp.1.try_borrow_mut() {
                    Ok(mut n) => n.update_score(score, &rp.0, depth + 1),
                    Err(_) => {}
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.parents = vec![];
        let children = mem::take(&mut self.children);

        for (_cm, rnode) in children.into_iter() {
            rnode.try_borrow_mut().map(|mut child| child.clear());
        }
    }

    pub fn as_debug_string(&self) -> String {
        let mut msg = String::with_capacity(10);  // y not
        msg.push_str("\n<Node>\n");
        msg.push_str(format!("valid:         {}\n", self.is_valid).as_str());
        msg.push_str(
            format!("node score: {}, {}\n",
            self.best_score.1,
            self.best_score.0.as_ref().map_or(String::from("None"), |cm| cm.to_long_algebraic_notation())
            ).as_str()
        );
        msg.push_str(format!("parents / children: {} / {}\n", self.parents.len(), self.children.len()).as_str());
        msg.push_str(self.board.as_debug_string().as_str());
        msg.push_str("</Node>\n");
        return msg
    }
}

#[derive(Debug, Clone)]
pub struct Manager {
    boards: Rc<RefCell<HashMap<u64, Rc<RefCell<Node>>>>>, // TODO use diff hash algo?
}


impl Manager {
    pub fn new() -> Manager {
        Manager{
            boards: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    pub fn clear(&mut self) {
        let mut rhash = mem::take(&mut self.boards);

        let hash = *rhash;

        for (_key, rnode) in hash.borrow_mut().iter() {
            *rnode.borrow_mut().clear();
        }

        self.boards = Rc::new(RefCell::new(HashMap::new()));
    }

    pub fn lookup_highest_move(&self, key: u64) -> std::result::Result<ChessMove, String> {
        log(format!("HM size: {}", self.boards.borrow().len()).as_str());

        match self.boards.borrow().get(&key) {
            None => Err(String::from("no board found in boards map")),
            Some(n) => {
                // println!("current board: {}", n.borrow().as_debug_string());
                // println!("TOTAL CHILDREN: {}", n.borrow().children.len());
                // for (cm, p) in n.borrow().children.iter() {
                //     println!("\n{}{}", cm.to_long_algebraic_notation(), p.borrow().as_debug_string());
                // }
                println!("Rc<RefCell<Node>>={}, usize={}, Node={}, Board={}, Vec<(ChessMove, Rc<RefCell<Node>>)>={}",
                         mem::size_of_val(&Rc::new(RefCell::new(Node::new(Board::new())))),
                         mem::size_of::<usize>(),
                         mem::size_of::<Node>(),
                         mem::size_of::<Board>(),
                         mem::size_of::<Vec<(ChessMove, Rc<RefCell<Node>>)>>(),
                );

                match &n.borrow().best_score.0 {
                    None => {
                        Err(String::from("no move found for board"))
                    },
                    Some(cm) => Ok(cm.clone())
                }
            }
        }
    }

    pub fn find_best_move(&mut self, board: Board, iters: usize) -> std::result::Result<ChessMove, String> {
        let key = board.id();
        let queue = {
            let mut bm = self.boards.borrow_mut();
            match bm.get(&key) {
                Some(rnode) => {
                    println!("NODE EXISTS WOAH, explored: {}, #child: {}", rnode.borrow().explored, rnode.borrow().children.len());
                    // TODO can free all nodes that are non-children of the requested one
                    // TODO grab node, grab it's children, find node with no children to actually explore

                    if !rnode.borrow().explored {
                        // node exists but we haven't explored it yet, can simply add it to the queue
                        ChessQueue::new(key)
                    } else {
                        // add all children nodes that aren't explored
                        let queue = ChessQueue::empty();
                        let entries = Manager::unexplored_child_nodes(rnode.borrow(), 1, &mut HashSet::new());
                        queue.append(entries);
                        queue
                    }
                },
                None => {
                    let mut node = Node::new(board);
                    node.is_valid = true;  // current board is assumed to be valid??
                    let rnode = Rc::new(RefCell::new(node));
                    bm.insert(key, rnode);

                    ChessQueue::new(key)
                }
            }
        };

        println!("Starting queue size: {}", queue.len());
        self.clone().work(key, queue.clone(), iters);

        // query current best move
        self.lookup_highest_move(key)
    }

    pub fn work(self, baseline_key: u64, mut queue: ChessQueue, iters: usize) -> () {
        let mut count = 0;

        // resolve baseline score so that we can do some basic filtering
        let base_node = self.boards.borrow().get(&baseline_key).unwrap().clone();
        let baseline_score = base_node.borrow().board.score();
        let whites_turn = base_node.borrow().board.is_whites_move();
        let f = |score: f32| -> bool {
            if whites_turn {
                score >= baseline_score
            } else {
                score <= baseline_score
            }
        };

        for _ in 0..iters {

            let items = queue.grab(100);
            println!("grabbed {} items, queue size = {}", items.len(), queue.len());

            for (depth, key) in items.into_iter() {
                count += 1;
                if count % 500 == 0 {
                    log(format!("ITER={}, depth={}", count, depth).as_str());
                }

                // get the node from the hashmap
                let rnode_base = match self.boards.borrow_mut().get(&key) {
                    None => {
                        log("board not found in hashmap!");
                        return;
                    },
                    Some(rnode) => rnode.clone(),
                };

                // filter it :D
                let s = rnode_base.borrow().board.score();
                if !f(s) {
                    continue
                }

                // list of final nodes to update scores
                let mut final_nodes: Vec<Rc<RefCell<Node>>> = Vec::with_capacity(1500);

                // rip all legal moves
                rnode_base.borrow_mut().explored = true;
                let moves = rnode_base.borrow_mut().board.get_legal_moves();
                for cm in moves.iter() {
                    let nboard = rnode_base.borrow().board.apply_move(cm);
                    let rnode_one = Manager::grow(&mut self.boards.borrow_mut(), &rnode_base, cm, nboard);

                    // rip all opponent legal moves as well
                    rnode_one.borrow_mut().explored = true;
                    let opp_moves = rnode_one.borrow_mut().board.get_legal_moves();
                    for opp_cm in opp_moves.iter() {
                        let nnboard = rnode_one.borrow().board.apply_move(opp_cm);
                        let rnode_two = Manager::grow(&mut self.boards.borrow_mut(), &rnode_one, opp_cm, nnboard);

                        final_nodes.push(rnode_two);
                    }
                }

                // backprop scores and add new nodes to the queue
                queue.append(
                    final_nodes.into_iter()
                        .map(|rnode| {
                            let node = rnode.borrow();

                            // propagate scores through the parents
                            for (cm, parent) in node.parents.iter() {
                                parent.try_borrow_mut().map(|mut n| n.update_score(node.best_score.1, cm, 0));
                            }

                            (depth + 2, node.board.id())
                        })
                        .collect()
                    );
            }
        }
    }

    fn unexplored_child_nodes(n: Ref<Node>, depth: u32, visited: &mut HashSet<u64>) -> Vec<(u32, u64)> {
        if visited.contains(&n.board.id()) {
            return Vec::new();
        } else {
            visited.insert(n.board.id());
        }

        let mut res: Vec<(u32, u64)> = Vec::new();
        for (_, r_child) in n.children[0..cmp::min(40, n.children.len())].to_vec().iter() {
            r_child.try_borrow().map(|child| {
                // println!("{}, {}", depth, child.board.id());
                if child.explored {
                    res.extend(Manager::unexplored_child_nodes(child, depth + 1, visited));
                } else {
                    res.push((depth, child.board.id()));
                }
            });
        }
        res
    }

    fn grow(hm: &mut RefMut<HashMap<u64, Rc<RefCell<Node>>>>, from: &Rc<RefCell<Node>>, cm: &ChessMove, board: Board) -> Rc<RefCell<Node>> {
        match hm.get(&board.id()) {
            None => {
                // create node
                let bs = board.score();
                let key = board.id();
                let mut node = Node::new(board);
                node.is_valid = true; // technically for now we're only considering legal moves
                node.best_score.1 = bs;
                let rnode = Rc::new(RefCell::new(node));

                // insert into hashmap
                hm.insert(key, rnode.clone());

                // update the parents & children
                rnode.borrow_mut().parents.push((cm.clone(), from.clone()));
                from.borrow_mut().children.push((cm.clone(), rnode.clone()));

                return rnode;
            },
            Some(rnode) => {
                // update the parents & children
                rnode.borrow_mut().parents.push((cm.clone(), from.clone()));
                from.borrow_mut().children.push((cm.clone(), rnode.clone()));

                return rnode.clone();
            }
        }

        // purposely _don't_ propogate score; caller should do that when necessary
    }
}

pub fn find_best_move(board: Board, manager: &mut Manager) -> std::result::Result<ChessMove, String> {
    manager.clear();
    manager.find_best_move(board, 50)
}

#[derive(Clone, Debug)]
pub struct ChessQueue {
    queue: Arc<Mutex<VecDeque<(u32, u64)>>>  // (depth, board_id)
}

impl ChessQueue {
    pub fn new(initial_board: u64) -> ChessQueue {
        let mut queue: VecDeque<(u32, u64)> = VecDeque::new();
        queue.push_back((0, initial_board));
        ChessQueue{queue: Arc::new(Mutex::new(queue))}
    }

    pub fn empty() -> ChessQueue {
        ChessQueue{queue: Arc::new(Mutex::new(VecDeque::new()))}
    }

    fn append(&self, boards: Vec<(u32, u64)>) {
        let mut queue = self.queue.lock().unwrap();

        // let the queue know we're gonna add a bunch of elements
        queue.reserve(boards.len());

        // add in the elements
        for elem in boards.into_iter() {
            queue.push_back(elem);
        };
    }

    fn grab(&mut self, size: u32) -> Vec<(u32, u64)> {
        let mut queue = self.queue.lock().unwrap();

        let amount = {
            if size >= queue.len() as u32 {
                queue.len()
            } else {
                size as usize
            }
        };

        let mut res: Vec<(u32, u64)> = Vec::with_capacity(amount);
        for elem in queue.drain(0..amount).into_iter() {
           res.push(elem);
        }

        res
    }

    fn len(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
}