use std::{collections::VecDeque, fmt::Display};

use ahash::AHashMap;

use crate::{
    util::{Edge, Graph},
};

const INPUT: &str = include_str!("../input/day23");
const INF: u64 = 1 << 42;

// We encode A as 1, B as 2, C as 3 and D as 4. Further, we index the rooms by the same
// number, i.e. room 1 is actually indexed by 1 (and not by zero!)

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Position {
    Hallway(u8),
    Room(u8, u8),
}

struct StateGraph;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Board<const N: usize> {
    hallway: [u8; 11],
    rooms: [[u8; N]; 4],
}

impl<const N: usize> Display for Board<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn letter(x: u8) -> char {
            match x {
                0 => '.',
                1 => 'A',
                2 => 'B',
                3 => 'C',
                4 => 'D',
                _ => panic!(),
            }
        }

        write!(f, "#############\n")?;

        write!(f, "#")?;
        for i in 0..11 {
            write!(f, "{}", letter(self.get(Position::Hallway(i))))?;
        }
        write!(f, "#\n")?;

        for j in 0..N as u8 {
            if j == 0 {
                write!(f, "###")?;
            } else {
                write!(f, "  #")?;
            }

            for i in 1..=4 {
                write!(f, "{}", letter(self.get(Position::Room(i, j))))?;
                write!(f, "#")?;
            }

            if j == 0 {
                write!(f, "##\n")?;
            } else {
                write!(f, "\n")?;
            }
        }

        write!(f, "  #########")
    }
}

impl<const N: usize> Graph<Board<N>> for StateGraph {
    fn neighbors(&self, v: &Board<N>) -> Vec<crate::util::Edge<Board<N>>> {
        fn room_state<const N: usize>(board: &Board<N>, i: u8) -> (bool, Option<usize>) {
            let mut good = true;
            let mut empty_spot = None;

            for (j, &x) in board.rooms[(i - 1) as usize].iter().enumerate() {
                if x == 0 {
                    empty_spot = Some(j)
                } else if x != i {
                    good = false;
                }
            }

            (good, empty_spot)
        }

        fn cost(x: u8) -> u64 {
            10u64.pow(x as u32 - 1)
        }

        fn next_states_from_hallway<const N: usize>(
            board: &Board<N>,
            out: &mut Vec<Edge<Board<N>>>,
        ) {
            for (from, &x) in board.hallway.iter().enumerate() {
                if x == 0 {
                    continue;
                }

                if let (room_good, Some(spot)) = room_state(board, x) {
                    if !room_good {
                        continue;
                    }

                    let from = Position::Hallway(from as u8);
                    let to = Position::Room(x, spot as u8);

                    if let Some(dist) = board.distance(from, to) {
                        out.push(Edge {
                            vertex: board.make_move_unchecked(from, to),
                            cost: dist * cost(x),
                        })
                    }
                }
            }
        }

        fn next_states_from_room<const N: usize>(
            board: &Board<N>,
            i: u8,
            out: &mut Vec<Edge<Board<N>>>,
        ) {
            let (room_good, _) = room_state(board, i);
            if room_good {
                return;
            }

            // We could move to destinations within our current room,
            // within our destination room or the hallway. But moves
            // within our current room are not useful in any case, and we can
            // always stop in the hallway "on the way", so we only allow
            // moves into the hallway from here
            for from in 0..N {
                let from = Position::Room(i, from as u8);
                let x = board.get(from);

                if board.get(from) == 0 {
                    continue;
                }

                let distances = board.distances(from);

                for to in [0, 1, 3, 5, 7, 9, 10] {
                    let to = Position::Hallway(to);
                    if let Some(dist) = distances.get(&to) {
                        let new_board = board.make_move_unchecked(from, to);

                        // Make sure the "interior" of the hallway stays sorted,
                        // otherwise two amphipods will just block each other there forever
                        // (since they have to get past each other to move into their rooms)
                        let (a, b, c) = (new_board.hallway[3], new_board.hallway[5], new_board.hallway[7]);
                        if (a == 0 || b == 0 || a < b) && (b == 0 || c == 0 || b < c) {
                            out.push(Edge {
                                vertex: new_board,
                                cost: dist * cost(x),
                            })
                        }
                    }
                }
            }
        }

        let mut out = Vec::new();

        next_states_from_hallway(v, &mut out);

        next_states_from_room(v, 1, &mut out);
        next_states_from_room(v, 2, &mut out);
        next_states_from_room(v, 3, &mut out);
        next_states_from_room(v, 4, &mut out);

        out
    }
}

impl<const N: usize> Board<N> {
    const SOLVED: Self = Board {
        hallway: [0; 11],
        rooms: [[1; N], [2; N], [3; N], [4; N]],
    };

    fn get(&self, pos: Position) -> u8 {
        match pos {
            Position::Hallway(i) => self.hallway[i as usize],
            Position::Room(i, j) => self.rooms[(i - 1) as usize][j as usize],
        }
    }

    fn get_mut(&mut self, pos: Position) -> &mut u8 {
        match pos {
            Position::Hallway(i) => &mut self.hallway[i as usize],
            Position::Room(i, j) => &mut self.rooms[(i - 1) as usize][j as usize],
        }
    }

    fn make_move_unchecked(&self, from: Position, to: Position) -> Self {
        let mut new = self.clone();
        *new.get_mut(to) = new.get(from);
        *new.get_mut(from) = 0;

        new
    }

    fn adjacent(&self, pos: Position, out: &mut Vec<Position>) {
        match pos {
            Position::Hallway(i) => {
                if i + 1 < 11 {
                    out.push(Position::Hallway(i + 1))
                }

                if i > 0 {
                    out.push(Position::Hallway(i - 1))
                }

                // Move into room
                if i == 2 {
                    out.push(Position::Room(1, 0));
                } else if i == 4 {
                    out.push(Position::Room(2, 0));
                } else if i == 6 {
                    out.push(Position::Room(3, 0));
                } else if i == 8 {
                    out.push(Position::Room(4, 0));
                }
            }
            Position::Room(i, j) => {
                if j + 1 < N as u8 {
                    out.push(Position::Room(i, j + 1))
                }

                if j > 0 {
                    out.push(Position::Room(i, j - 1))
                }

                if j == 0 {
                    out.push(Position::Hallway(2 * i))
                }
            }
        }
    }

    fn distances(&self, start: Position) -> AHashMap<Position, u64> {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut visited: AHashMap<_, _> = Default::default();
        visited.insert(start, 0);

        let mut adj = Vec::new();

        while let Some(v) = queue.pop_front() {
            adj.clear();
            self.adjacent(v, &mut adj);

            for &w in &adj {
                if visited.contains_key(&w) {
                    continue;
                }

                if self.get(w) > 0 {
                    // Can't go to occupied field
                    continue;
                }

                queue.push_back(w);
                visited.insert(w, visited[&v] + 1);
            }
        }

        visited
    }

    fn distance(&self, start: Position, end: Position) -> Option<u64> {
        self.distances(start).get(&end).copied()
    }
}

fn is_solved<const N: usize>(b: &Board<N>) -> bool {
    b == &Board::SOLVED
}

fn find_solution<const N: usize>(b: Board<N>) -> u64 {
    let (dist, _) = StateGraph.dijsktra(b, Some(Board::SOLVED), false);
    dist[&Board::SOLVED]
}

pub fn solve() -> crate::Result<()> {
    let mut rooms = [[0, 0], [0, 0], [0, 0], [0, 0]];

    for (j, line) in INPUT.lines().skip(2).take(2).enumerate() {
        for (i, x) in line
            .trim()
            .split('#')
            .filter(|x| !x.trim().is_empty())
            .enumerate()
        {
            let x = match x {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                "D" => 4,
                _ => panic!("unknown: {}", x),
            };

            rooms[i][j] = x;
        }
    }

    let board1 = Board {
        hallway: [0; 11],
        rooms,
    };

    println!("Problem 1: {}", find_solution(board1));

    let board2 = Board {
        hallway: [0; 11],
        rooms: [
            [board1.rooms[0][0], 4, 4, board1.rooms[0][1]],
            [board1.rooms[1][0], 3, 2, board1.rooms[1][1]],
            [board1.rooms[2][0], 2, 1, board1.rooms[2][1]],
            [board1.rooms[3][0], 1, 3, board1.rooms[3][1]],
        ],
    };

    println!("Problem 2: {}", find_solution(board2));

    Ok(())
}
