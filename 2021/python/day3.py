import time

def most_common_bit(bits):
    res = 0
    for bit in bits:
        if bit:
            res += 1
        else:
            res -= 1

    return int(res >= 0)

def bits_to_number(bits):
    res = 0
    for bit in bits:
        res *= 2
        res += bit

    return res

def solve():
    with open("input/day3") as f:
        lines = f.readlines()

    nrs = [[int(digit) for digit in line.strip()] for line in lines]
    solve1(nrs)
    solve2(nrs)

def solve1(xs):
    n = len(xs[0])

    most_common_bits = [most_common_bit(x[i] for x in xs) for i in range(n)]

    gamma = bits_to_number(most_common_bits)
    epsilon = 2**n - 1 - gamma

    print("Problem 1:", gamma * epsilon)

def select(xs, select_most_common=True):
    n = len(xs[0])

    # Current set of numbers
    selected = [True] * len(xs)
    selected_count = len(xs)
    
    for i in range(n):
        most_common = most_common_bit(x[i] for j, x in enumerate(xs) if selected[j])
        v = most_common if select_most_common else 1 - most_common

        for j, x in enumerate(xs):
            if selected[j] and x[i] != v:
                selected[j] = False
                selected_count -= 1

            if selected_count <= 1:
                for i, s in enumerate(selected):
                    if s:
                        return xs[i]



def solve2(xs):
    oxygen_generator_rating = bits_to_number(select(xs, select_most_common=True))
    co2_scrubber_rating = bits_to_number(select(xs, select_most_common=False))      

    print("Problem 2:", oxygen_generator_rating * co2_scrubber_rating)

if __name__ == '__main__':
    solve()