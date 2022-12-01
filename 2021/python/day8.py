CHARS = ['a', 'b', 'c', 'd', 'e', 'f', 'g']

def parse_pattern(w):
    # represent e.g. the word acf as a "binary pattern" (1,0,1,0,0,1,0)
    return tuple([int(c in w) for c in CHARS])

def permute(pi, bin_pattern):
    res = [0] * len(CHARS)
    for i, b in enumerate(bin_pattern):
        res[pi[i]] = b
    return tuple(res)

def solve():
    DIGITS = [
        'abcefg', 'cf', 'acdeg', 'acdfg', 'bcdf', 'abdfg', 'abdefg', 'acf', 'abcdefg', 'abcdfg'
    ]

    with open("input/day8") as f:
        lines = f.readlines()

    observations = [
        tuple([parse_pattern(w) for w in x.split()] for x in line.split(' | ')) 
        for line in lines
    ]

    prob1 = 0
    target_lengths = {len(DIGITS[1]),len(DIGITS[4]), len(DIGITS[7]), len(DIGITS[8])}
    for _, output in observations:
        for pattern in output:
            ones = sum(pattern)
            if  ones in target_lengths:
                prob1 += 1

    print("Problem 1:", prob1)

    ### Part 2

    # Now, our task for part 2 can be stated like this: Find a permutation pi of {0, 1,...,6} such that
    # for every binary input pattern (x0, ..., x6) the binary pattern (x)^pi = (x_pi^-1(0), x_pi^-1(1), ..., x_pi^-1(6))
    # is a valid binary pattern, i.e. a key in bin_pattern_to_digit.
    # Note that it is somewhat more natural to work with the inverse of the permutation in this instance:
    # If pi(i) = j then the jth component of the permuted tuple equals x_pi^-1{j} = x_i

    bin_pattern_to_digit = {parse_pattern(w):i for i, w in enumerate(DIGITS)}

    # We can also reduce the search space using a simple observation: 
    # Suppose that (x0, ..., x6) is an input pattern with exactly 3 one entries.
    # Then it has to represent a 7 and it holds that
    # (x)^pi = (x_pi^-1(0), ..., x_pipi^-1(6)) = (1, 0, 1, 0, 0, 1, 0)
    # If x_j = 1 then the pi(j) th component of the permuted vector is also 1, so that
    # pi(j) \in {0, 2, 5} holds.
    possible_positions = {i:set() for i in range(len(CHARS) + 1)}
    for bin_pattern in bin_pattern_to_digit:
        ones = sum(bin_pattern)
        possible_positions[ones].update(i for i, x in enumerate(bin_pattern) if x == 1)
    
    prob2 = 0
    for inp, outp in observations:
        possible_pi = {i:set(range(len(CHARS))) for i in range(len(CHARS))}

        for bin_pattern in inp:
            l = sum(bin_pattern)

            for i, b in enumerate(bin_pattern):
                if b:
                    possible_pi[i] &= possible_positions[l]

        def is_valid_pi(pi):
            return all(permute(pi, w) in bin_pattern_to_digit for w in inp)

        # We find this permutation using backtracking
        def make_pi(pi, i, used):
            if i == len(CHARS):
                if len(used) == len(CHARS) and is_valid_pi(pi):
                    return pi
            else:
                for j in possible_pi[i].difference(used):
                    used.add(j)
                    pi[i] = j
                    
                    res = make_pi(pi, i + 1, used)
                    if res:
                        return res

                    pi[i] = None
                    used.remove(j)

        pi = make_pi({i:None for i in range(len(CHARS))}, 0, set())

        displayed_number = 0
        for w in outp:
            displayed_number *= 10
            displayed_number += bin_pattern_to_digit[permute(pi, w)]

        prob2 += displayed_number

    print("Problem 2:", prob2)

if __name__ == '__main__':
    solve()