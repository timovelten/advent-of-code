use std::collections::HashMap;

const INPUT: &str = include_str!("../input/day4");
const INF: usize = usize::MAX;

fn determine_winning_move_count(
    board: &Vec<u32>,
    moves: &Vec<u32>,
    moves_map: &HashMap<u32, usize>,
    width: usize,
) -> Option<(usize, u32)> {
    let marked_after = board
        .iter()
        .map(|x| moves_map.get(x).copied().unwrap_or(INF))
        .collect::<Vec<_>>();

    let mut can_win_after = INF;

    // Check horizontal lines
    for y in 0..width {
        let tmp = (0..width)
            .map(|x| marked_after[x + y * width])
            .max()
            .unwrap();

        can_win_after = std::cmp::min(can_win_after, tmp);
    }

    // Check vertical lines
    for x in 0..width {
        let tmp = (0..width)
            .map(|y| marked_after[x + y * width])
            .max()
            .unwrap();

        can_win_after = std::cmp::min(can_win_after, tmp);
    }

    if can_win_after < INF {
        // Determine the score
        let unmarked_sum: u32 = marked_after
            .iter()
            .zip(board)
            .filter(|(&v, _)| v > can_win_after)
            .map(|(_, &x)| x)
            .sum();

        Some((can_win_after, unmarked_sum * moves[can_win_after]))
    } else {
        None
    }
}

pub fn solve() -> crate::Result<()> {
    const BOARD_ENTRIES: usize = 5 * 5;

    let mut lines = INPUT.lines().chain(std::iter::once("".into()));

    let moves = lines
        .next()
        .ok_or(crate::Error::NoInput)?
        .split(",")
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let moves_map = moves
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect::<HashMap<u32, usize>>();

    // Skip following whitespace
    let _ = lines.next();

    let mut boards: Vec<Vec<u32>> = Vec::new();
    let mut current_board: Vec<u32> = Vec::with_capacity(BOARD_ENTRIES);

    for line in lines {
        if line.trim().is_empty() {
            let old = std::mem::replace(&mut current_board, Vec::with_capacity(BOARD_ENTRIES));
            boards.push(old);
        } else {
            for x in line.split_whitespace() {
                let x = x.parse::<u32>()?;
                current_board.push(x);
            }
        }
    }

    let mut final_score_1 = 0;
    let mut winning_move_count_1 = INF;

    let mut final_score_2 = 0;
    let mut winning_move_count_2 = 0;

    for board in boards {
        if let Some((count, score)) = determine_winning_move_count(&board, &moves, &moves_map, 5) {
            if count < winning_move_count_1 {
                final_score_1 = score;
                winning_move_count_1 = count;
            }

            if count > winning_move_count_2 || winning_move_count_2 == 0 {
                final_score_2 = score;
                winning_move_count_2 = count;
            }
        }
    }

    println!("Problem 1: {}", final_score_1);
    println!("Problem 2: {}", final_score_2);

    Ok(())
}
