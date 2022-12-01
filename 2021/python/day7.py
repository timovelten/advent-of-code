def solve():
    pass

def solve():
    with open("input/day7") as f:
        line = f.readline()

    crabs = [int(x) for x in line.split(",")]
    max_ = max(crabs)

    def cost2(x, i):
        n = abs(x - i)
        return n * (n + 1)

    print("Problem 1:", minimize(crabs, max_, lambda x, i: abs(x - i)))
    print("Problem 2:", minimize(crabs, max_, cost2) // 2)

def minimize(crabs, max_, cost):
    best = 2**64 - 1

    for i in range(0, max_ + 1):
        current = 0

        for x in crabs:
            current += cost(x, i)

            if current > best:
                break

        if current < best:
            best = current

    return best

if __name__ == '__main__':
    solve()