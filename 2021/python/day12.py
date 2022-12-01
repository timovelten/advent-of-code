def solve():
    with open("input/day12") as f:
        lines = f.readlines()

    adjacent = {}
    for line in lines:
        v, w = line.strip().split("-")
        
        if v in adjacent and w not in adjacent[v]:
            adjacent[v].append(w)
        else:
            adjacent[v] = [w]

        if w in adjacent and v not in adjacent[w]:
            adjacent[w].append(v)
        else:
            adjacent[w] = [v]

    small_caves = {v for v in adjacent if v.lower() == v and v != 'start' and v != 'end'}

    visit_once = small_caves | {'start', 'end'}

    # f(start, end) counts the paths from start to end
    # if visit_twice is set, it only counts those paths that visit the vertex passed in visit_twice
    # *exactly* twice
    def f(start, end, path, visit_twice):
        if start == end:
            if not visit_twice:
                return 1
            else:
                return 0

        ret = 0

        for v in adjacent[start]:
            new_visit_twice = visit_twice   

            if v in visit_once and v in path:
                if v == visit_twice:
                    # This vertex is now visited for the second time, but we allow it
                    new_visit_twice = None
                else:
                    continue
            
            ret += f(v, end, path | {v}, new_visit_twice)

        return ret

    prob1 = f('start', 'end', {'start'}, None)

    prob2 = prob1
    for v in small_caves:
        prob2 += f('start', 'end', {'start'}, v)

    print("Problem 1:", prob1)
    print("Problem 2:", prob2)
        
        
if __name__ == '__main__':
    solve()