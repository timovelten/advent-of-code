# this could just be chosen as len(moves) but a "big" number is more readable
INF = 2**32

def determine_winning_move_count(board, moves, moves_dict, size=5):
    marked_after = [moves_dict.get(x, INF) for x in board]

    # Now determine if there are any complete straight lines in marked_after
    # if so, we know when that happened
    
    can_win_after = INF

    # Horizontal lines
    for y in range(size):
        tmp = max(marked_after[x + size * y] for x in range(size))
        can_win_after = min(tmp, can_win_after)

    # Vertical lines
    for x in range(size):
        tmp = max(marked_after[x + size * y] for y in range(size))
        can_win_after = min(tmp, can_win_after)

    if can_win_after < INF:
        unmarked_sum = 0
        for v, x in zip(marked_after, board):
            if v > can_win_after:
                unmarked_sum += x

        score = unmarked_sum * moves[can_win_after]

        return (can_win_after, score)
    else:
        return None

def solve():
    with open("input/day4") as f:
        lines = f.readlines()
        lines.append("")

    moves = [int(x) for x in lines[0].split(",")]

    boards = []
    current_board = []

    for line in lines[2:]:
        if not line.strip():
            boards.append(current_board)
            current_board = []
        else:
            current_board.extend(int(x) for x in line.split())

    moves_dict = {x:i for i, x in enumerate(moves)}

    final_score_1 = 0
    winning_move_count_1 = INF

    winning_move_count_2 = -1
    final_score_2 = 0

    for board in boards:
        count, score = determine_winning_move_count(board, moves, moves_dict)

        if count < winning_move_count_1:
            final_score_1 = score
            winning_move_count_1 = count

        if count > winning_move_count_2:
            final_score_2 = score
            winning_move_count_2 = count


    print("Problem 1:", final_score_1)
    print("Problem 2:", final_score_2)

if __name__ == '__main__':
    solve()