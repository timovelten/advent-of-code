def solve():
    with open("input/day2") as f:
        lines = f.readlines()

    pos1 = 0
    depth1 = 0

    aim2 = 0
    pos2 = 0
    depth2 = 0

    for line in lines:
        instruction, x = line.split()
        x = int(x)

        if instruction == 'forward':
            pos1 += x

            pos2 += x
            depth2 += aim2 * x
        elif instruction == 'down':
            depth1 += x

            aim2 += x
        elif instruction == 'up':
            depth1 -= x

            aim2 -= x

    print("Problem 1:", pos1 * depth1)
    print("Problem 2:", pos2 * depth2)

if __name__ == '__main__':
    solve()