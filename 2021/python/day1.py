def solve():
    with open("input/day1") as f:
        lines = f.readlines()

    nrs = [int(line) for line in lines]

    prob1 = 0
    for i in range(1, len(nrs)):
        if nrs[i] > nrs[i - 1]:
            prob1 += 1

    print("Problem 1:", prob1)

    prob2 = 0
    for i in range(3, len(nrs)):
        a, b, c, d = (nrs[i - 3], nrs[i - 2], nrs[i - 1], nrs[i])
        if b + c + d > a + b + c:
            prob2 += 1

    print("Problem 2:", prob2)
        
if __name__ == '__main__':
    solve()