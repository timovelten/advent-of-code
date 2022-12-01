def solve():
    with open("input/day6") as f:
        line = f.readline()

    # This is dynamic programming but recursive because I dont like writing dp programs
    # The rust version of this will be a proper dp version
    cache = {}   
    def f(x, n):
        """f(x, n) is the number of fish the fish turns into in n days"""
        if (x, n) in cache:
            return cache[(x, n)]

        if n == 0:
            ret = 1
        elif x == 0:
            ret = f(6, n - 1) + f(8, n - 1)
        else:
            ret = f(x - 1, n - 1)

        cache[(x, n)] = ret
        return ret

    fish = [int(x) for x in line.split(",")]
    
    print("Problem 1:", sum(f(x, 80) for x in fish))
    print("Problem 2:", sum(f(x, 256) for x in fish))

if __name__ == '__main__':
    solve()