class Day24 : Solver {
    record Vec2(int x, int y);

    class Ground {
        public int Up = 0;
        public int Right = 0;
        public int Down = 0;
        public int Left = 0;

        public bool IsOpen() {
            return Up == 0 && Right == 0 && Down == 0 && Left == 0;
        }
    }

    static IEnumerable<Vec2> AdjacentPositions(Vec2 p) {
        yield return new Vec2(p.x, p.y + 1);
        yield return new Vec2(p.x, p.y - 1); 
        yield return new Vec2(p.x - 1, p.y);
        yield return new Vec2(p.x + 1, p.y);
    }

    static int mod(long x, int m) {
        int r = (int)(x % m);
        return x >= 0 ? r : r + m;
    }

    public void solve(string input) {
        var lines = input.Trim().Split("\n");
        var width = lines[0].Length - 2;
        var height = lines.Length - 2;

        var grid = new Ground[width * height];
        for (var y = 0; y < height; y++) {
            for (var x = 0; x < width; x++) {
                var c = lines[1 + y][1 + x];
                if (c == '.') {
                    grid[x + y * width] = new Ground();
                } else if (c == '^') {
                    if (x == 0 || x == width - 1) throw new InvalidOperationException();
                    grid[x + y * width] = new Ground() { Up = 1 };
                } else if (c == '>') {
                    grid[x + y * width] = new Ground() { Right = 1 };
                } else if (c == 'v') {
                    if (x == 0 || x == width - 1) throw new InvalidOperationException();
                    grid[x + y * width] = new Ground() { Down = 1 }; 
                } else if (c == '<') {
                    grid[x + y * width] = new Ground() { Left = 1 }; 
                } else {
                    throw new InvalidOperationException();
                }
            }
        }

        Console.WriteLine($"Problem 1: {f(grid, width, true)}");
        Console.WriteLine($"Problem 1: {f(grid, width, false)}");
    }

    static long f(Ground[] initialGrid, int width, bool part1) {
        int height = initialGrid.Length / width;
        List<Ground[]> simulations = new List<Ground[]>() { initialGrid };

        int finalStage = part1 switch {
            true => 1,
            false => 3,
        };

        var start = new Vec2(0, -1);
        var end = new Vec2(width - 1, height);

        var queue = new Queue<(int, int, Vec2)>();
        queue.Enqueue((0, 0, start));
        var visited = new HashSet<(int, int, Vec2)>() { (0, 0, start) };

        while (queue.Count > 0) {
            var (i, stage, p) = queue.Dequeue();
            var pInBounds = 0 <= p.x && p.x < width && 0 <= p.y && p.y < height;
            EnsureEnoughSimulations(i + 1, simulations, width);
            var nextGrid = simulations[i + 1];

            var reachedNextStage = ((stage == 0 || stage == 2) && p == end) || (stage == 1 && p == start);
            if (reachedNextStage) {
                stage++;
            };

            if (stage == finalStage) {
                return i;
            }
            
            foreach (var q in AdjacentPositions(p)) {
                var qInBounds = 0 <= q.x && q.x < width && 0 <= q.y && q.y < height;
                if (!qInBounds && q != start && q != end) continue;
                if (visited.Contains((i + 1, stage, q))) continue;
                if (qInBounds && !nextGrid[q.x + q.y * width].IsOpen()) continue;

                visited.Add((i + 1, stage, q));
                queue.Enqueue((i + 1, stage, q));
            }

            var canStay = (p == start || p == end || (pInBounds && nextGrid[p.x + p.y * width].IsOpen()));
            if (canStay) {
                visited.Add((i + 1, stage, p));
                queue.Enqueue((i + 1, stage, p));
            }
        }

        return -1;
    }

    static void EnsureEnoughSimulations(int i, List<Ground[]> simulations, int width) {
        if (i >= simulations.Count) {
            var count = simulations.Count;
            var grid = simulations.Last();
            for (var j = 0; j < count; j++) {
                grid = simulate(grid, width);
                simulations.Add(grid);
            } 
        }
    }

    static Ground[] simulate(Ground[] grid, int width) {
        int height = grid.Length / width;

        // The copy is hard to avoid, because we have to avoid reading the changes we make here later
        var ret = new Ground[grid.Length];
        for (var i = 0; i < grid.Length; i++) ret[i] = new Ground();
        
        for (var y = 0; y < height; y++) {
            for (var x = 0; x < width; x++) {
                var b = grid[x + y * width];
                for (var i = 0; i < b.Up; i++) {
                    ret[x + mod(y - 1, height) * width].Up++;
                }
                for (var i = 0; i < b.Down; i++) {
                    ret[x + mod(y + 1, height) * width].Down++;
                }
                for (var i = 0; i < b.Right; i++) {
                    ret[mod(x + 1, width) + y * width].Right++;
                }
                for (var i = 0; i < b.Left; i++) {
                    ret[mod(x - 1, width) + y * width].Left++;
                }
            }
        }

        return ret;
    }

    static void print(Ground[] grid, int width, int height) {
        for (var y = 0; y < height; y++) {
            for (var x = 0; x < width; x++) {
                var g = grid[x + y * width];
                var sum = g.Up + g.Down + g.Right + g.Left;
                if (sum > 1) {
                    Console.Write(sum);
                } else if (sum == 1) {
                    if (g.Up == 1) {
                        Console.Write("^");
                    } else if (g.Down == 1) {
                        Console.Write("v");
                    } else if (g.Right == 1) {
                        Console.Write(">");
                    } else if (g.Left == 1) {
                        Console.Write("<");
                    }
                } else if (sum == 0) {
                    Console.Write(".");                }
            }
            Console.WriteLine();
        }
    }
}