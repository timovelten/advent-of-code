class Day9 : Solver {
    public void solve(string input) {
        var lines = input.Trim().Split("\n");
        Console.WriteLine($"Problem 1: {doIt(lines, 1)}");
        Console.WriteLine($"Problem 2: {doIt(lines, 9)}");
    }

    int doIt(IEnumerable<string> lines, int n) {
        var visited = new HashSet<(int, int)>();

        var (hx, hy) = (0, 0);
        var knots = new (int, int)[n];

        foreach (var line in lines) {
            var split = line.Split(" ");
            var dir = split[0];
            var num = int.Parse(split[1]);

            for (var i = 0; i < num; i++) {
                if (dir == "U") {
                    hy -= 1;
                }
                else if (dir == "D") {
                    hy += 1;
                }
                else if (dir == "L") {
                    hx -= 1;
                }
                else {
                    hx += 1;
                }

                // Now each knot must follow. Initially, we follow the head.
                var (fx, fy) = (hx, hy);
                for (int j = 0; j < n; j++) {
                    var dx = fx - knots[j].Item1;
                    var dy = fy - knots[j].Item2;
                    var sx = Math.Sign(dx);
                    var sy = Math.Sign(dy);

                    if (dx == 0 && sy * dy >= 2) {
                        knots[j].Item2 += sy;
                    }
                    else if (dy == 0 && sx * dx >= 2) {
                        knots[j].Item1 += sx;
                    }
                    else if (sy * dy >= 2 || sx * dx >= 2) {
                        knots[j].Item1 += sx;
                        knots[j].Item2 += sy;
                    }

                    (fx, fy) = knots[j];
                }

                visited.Add(knots[knots.Length - 1]);
            }
        }

        return visited.Count;
    }
}