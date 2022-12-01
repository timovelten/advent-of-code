def sign(x):
    if x > 0:
        return 1
    elif x < 0:
        return -1
    else:
        return 0

def solve():
    with open("input/day5") as f:
        lines = f.readlines()
        
    hit_count_1 = {}
    hit_count_2 = {}

    for line in lines:
        from_, to = line.split(" -> ")
        x1, y1 = (int(x) for x in from_.split(","))
        x2, y2 = (int(x) for x in to.split(","))

        dx = sign(x2 - x1)
        dy = sign(y2 - y1)

        straight = dx == 0 or dy == 0

        x, y = x1, y1
        while True:
            if straight:
                if (x, y) in hit_count_1:
                    hit_count_1[(x, y)] += 1
                else:
                    hit_count_1[(x, y)] = 1

            if (x, y) in hit_count_2:
                hit_count_2[(x, y)] += 1
            else:
                hit_count_2[(x, y)] = 1

            if x == x2 and y == y2:
                break

            x += dx
            y += dy
    
    prob1 = sum(1 for v in hit_count_1.values() if v >= 2)
    prob2 = sum(1 for v in hit_count_2.values() if v >= 2)

    print("Problem 1:", prob1)
    print("Problem 2:", prob2)

if __name__ == '__main__':
    solve()