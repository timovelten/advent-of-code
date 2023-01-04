class Day23 : Solver {
    static (int x, int y) Add((int x, int y) p, (int x, int y) q) {
        return (p.x + q.x, p.y + q.y);
    }
    public void solve(string input) {
        var lines = input.Trim().Split("\n");
        var height = lines.Length;
        var width = lines[0].Length;

        var grid = new HashSet<(int x, int y)>();
        for (var y = 0; y < height; y++) {
            for (var x = 0; x < width; x++) {
                if (lines[y][x] == '#') {
                    grid.Add((x, y));
                }
            }
        }

        int i = 0;
        while (true) {
            var ret = simulate(grid, i);
            if (i == 10) {
                var xMax = grid.Select(p => p.x).Max();
                var xMin = grid.Select(p => p.x).Min();
                var yMax = grid.Select(p => p.y).Max();
                var yMin = grid.Select(p => p.y).Min();
                
                var prob1 = (xMax - xMin + 1) * (yMax - yMin + 1) - grid.Count();
                Console.WriteLine($"Problem 1: {prob1}");
            }

            if (!ret) {
                Console.WriteLine($"Problem 2: {i + 1}");
                break;
            }

            i++;
        }
    }

    bool simulate(HashSet<(int x, int y)> grid, int round) {
        // Key is from, value is to
        var moveTo = new Dictionary<(int x, int y), (int x, int y)>();
        var proposalCount = new Dictionary<(int x, int y), int>();
        var anyoneMoved = false;

        bool[] free = new bool[4];
        (int, int)[] deltas = new [] { (0, -1), (0, 1), (-1, 0), (1, 0) };

        foreach (var p in grid) {
            var northFree = !(
                grid.Contains((p.x - 1, p.y - 1)) || grid.Contains((p.x, p.y - 1)) || grid.Contains((p.x + 1, p.y - 1))
            );
            var southFree = !(
                grid.Contains((p.x - 1, p.y + 1)) || grid.Contains((p.x, p.y + 1)) || grid.Contains((p.x + 1, p.y + 1))
            );
            var westFree = !(
                grid.Contains((p.x - 1, p.y - 1)) || grid.Contains((p.x - 1, p.y)) || grid.Contains((p.x - 1, p.y + 1))
            );
            var eastFree = !(
                grid.Contains((p.x + 1, p.y - 1)) || grid.Contains((p.x + 1, p.y)) || grid.Contains((p.x + 1, p.y + 1))
            );
            var allFree = northFree && southFree && westFree && eastFree;

            free[0] = northFree;
            free[1] = southFree;
            free[2] = westFree;
            free[3] = eastFree;
            
            var delta = (0, 0);
            if (!allFree) {
                for (var j = 0; j < 4; j++) {
                    var idx = (round + j) % 4;
                    if (free[idx]) {
                        delta = deltas[idx];
                        break;
                    }
                }
            }

            var prop = Add(p, delta);

            moveTo[p] = prop;
            proposalCount[prop] = proposalCount.GetValueOrDefault(prop, 0) + 1;
        }

        grid.Clear();

        foreach (var (from, to) in moveTo) {
            if (proposalCount[to] == 1 && from != to) {
                grid.Add(to);
                anyoneMoved = true;
            } else {
                grid.Add(from);
            }
        }

        return anyoneMoved;
    }

    static void print(HashSet<(int, int)> points, int width, int height) {
        for (var y = 0; y < height; y++) {
            for (var x = 0; x < width; x++) {
                if (points.Contains((x, y))) {
                    Console.Write("#");
                } else {
                    Console.Write(".");
                }
            }
            Console.WriteLine();
        }
    }
}