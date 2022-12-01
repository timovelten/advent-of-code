def adjacent_locations(x, y, width, height):
    for dx in range(-1, 2):
        for dy in range(-1, 2):
            if dx == 0 and dy == 0:
                continue

            (nx, ny) = (x + dx, y + dy)

            if nx >= 0 and ny >= 0 and nx < width and ny < height:
                yield (nx, ny)

def solve():
    with open("input/day11") as f:
        lines = f.readlines()

    grid = [[int(x) for x in line.strip()] for line in lines if line.strip()]

    width = len(grid[0])
    height = len(grid)

    def increase(x, y):
        grid[y][x] += 1

        # Only flash once
        if grid[y][x] == 10:
            ret = 1

            for nx, ny in adjacent_locations(x, y, width, height):
                ret += increase(nx, ny)

            return ret
        else:
            return 0

    prob1 = 0
    prob2 = None

    for step in range(2**32):
        flashed_this_step = 0

        for x in range(width):
            for y in range(height):
                flashed_this_step += increase(x, y)

        for x in range(width):
            for y in range(height):
                if grid[y][x] > 9:
                    grid[y][x] = 0

        if step < 100:
            prob1 += flashed_this_step

        if not prob2 and flashed_this_step == width * height:
            prob2 = step + 1

        if step >= 100 and prob2:
            break

    print("Problem 1:", prob1)
    print("Problem 2:", prob2)


if __name__ == '__main__':
    solve()