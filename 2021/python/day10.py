def solve():
    with open("input/day10") as f:
        lines = f.readlines()

    prob1 = 0
    autocomplete_scores = []

    braces = {
        '(':')',
        '[':']',
        '{':'}',
        '<':'>'
    }

    for line in lines:
        opened = []

        first_illegal_character = None

        for c in line.strip():
            if c in braces:
                opened.append(c)
            elif len(opened) > 0:
                expected = braces[opened.pop()]
                if c != expected:
                    first_illegal_character = c
                    break

        if first_illegal_character:
            # Corrupted line
            if first_illegal_character == ')':
                prob1 += 3
            elif first_illegal_character == ']':
                prob1 += 57
            elif first_illegal_character == '}':
                prob1 += 1197
            elif first_illegal_character == '>':
                prob1 += 25137
        else:
            # Incomplete line
            score = 0

            for c in reversed(opened):
                score *= 5
                if c == '(':
                    score += 1
                elif c == '[':
                    score += 2
                elif c == '{':
                    score += 3
                elif c == '<':
                    score += 4
            
            autocomplete_scores.append(score)

    print("Problem 1:", prob1)

    autocomplete_scores.sort()
    print("Problem 2:", autocomplete_scores[len(autocomplete_scores) // 2])

if __name__ == '__main__':
    solve()