DIRECTIONS = [(-1, 0), (1, 0), (0, -1), (0, 1)]

def adjacent_locations(x, y, width, height):
    for (dx, dy) in DIRECTIONS:
        (nx, ny) = (x + dx, y + dy)

        if nx >= 0 and ny >= 0 and nx < width and ny < height:
            yield (nx, ny)

def solve():
    with open("input/day9") as f:
        lines = f.readlines()

    grid = [[int(x) for x in line.strip()] for line in lines if not len(line.strip()) == 0]
    width = len(grid[0])
    height = len(grid)

    prob1 = 0
    basin_sizes = []

    for x in range(width):
        for y in range(height):
            lowpoint = True
            val = grid[y][x]

            for nx, ny in adjacent_locations(x, y, width, height):
                if grid[ny][nx] <= val:
                    lowpoint = False
                    break

            if not lowpoint:
                continue

            prob1 += val + 1

            basin = set()
            to_be_checked = [(x, y)]
            while len(to_be_checked) > 0:
                cx, cy = to_be_checked.pop()
                basin.add((cx, cy))

                val = grid[cy][cx]
                for nx, ny in adjacent_locations(cx, cy, width, height):
                    nval = grid[ny][nx]

                    if nval < 9 and nval > val and (nx, ny) not in basin:
                        to_be_checked.append((nx, ny))

            basin_sizes.append(len(basin))

    
    print("Problem 1:", prob1)
    
    basin_sizes.sort(reverse=True)
    prob2 = basin_sizes[0] * basin_sizes[1] * basin_sizes[2]

    print("Problem 2:", prob2)

if __name__ == '__main__':
    solve()