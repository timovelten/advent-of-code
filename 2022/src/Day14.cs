class Day14 : Solver {
    enum Tile {
        Rock,
        Sand
    }

    public void solve(string input) {
        var grid = new Dictionary<(int x, int y), Tile>();
        var lines = input.Trim().Split("\n");

        foreach (var line in lines) {
            var path = line.Split(" -> ").Select(x => x.Split(","))
                .Select(split => (int.Parse(split[0]), int.Parse(split[1])));

            var (x, y) = (0, 0);
            var first = true;

            foreach (var (nx, ny) in path) {
                if (first) {
                    (x, y) = (nx, ny);
                    grid[(x, y)] = Tile.Rock;
                    first = false;
                    continue;
                }

                if (nx != x && ny == y) {
                    var d = Math.Sign(nx - x);
                    while (true) {
                        grid[(x, y)] = Tile.Rock;
                        if (nx == x) break;

                        x += d;
                    }
                }
                else if (nx == x && ny != y) {
                    var d = Math.Sign(ny - y);
                    while (true) {
                        grid[(x, y)] = Tile.Rock;
                        if (ny == y) break;

                        y += d;
                    }
                }
                else if (nx == x && ny == y) {
                    // Ok, do nothing...
                }
                else {
                    throw new Exception("no diagonal movement");
                }
            }
        }

        var floor = grid.Select(entry => entry.Key.y).Max() + 2;

        Console.WriteLine($"floor at {floor}");

        // Copy the grid...
        var grid1 = grid.ToDictionary(item => item.Key, item => item.Value);
        var prob1 = 0;
        while (step(grid1, floor, false) != null) {
            prob1 += 1;
        }
        Console.WriteLine($"Problem 1: {prob1}");

        var grid2 = grid;
        var prob2 = 0;
        while (step(grid2, floor, true) != (500, 0)) {
            prob2 += 1;
        }
        Console.WriteLine($"Problem 2: {prob2 + 1}");
    }

    // Returns the location where the new piece of sand comes to rest. If it falls forver, returns null
    (int x, int y)? step(Dictionary<(int, int), Tile> grid, int floor, bool landOnFloor) {
        if (grid.ContainsKey((500, 0))) {
            throw new Exception("entrance blocked");
        }

        var (x, y) = (500, 0);
        while (true) {
            // Invariant: (x, y) is not contained in grid.
            if (y > floor) {
                return null;
            }

            var onFloor = landOnFloor && y + 1 == floor;

            if (!grid.ContainsKey((x, y + 1)) && !onFloor) {
                // Move down
                y += 1;
                continue;
            }
            else if (!grid.ContainsKey((x - 1, y + 1)) && !onFloor) {
                // Move down left
                y += 1;
                x -= 1;
                continue;
            }
            else if (!grid.ContainsKey((x + 1, y + 1)) && !onFloor) {
                // Move down right
                y += 1;
                x += 1;
                continue;
            }

            // We could not move (possibly because we hit the floor). Return that.
            grid[(x, y)] = Tile.Sand;
            return (x, y);
        }
    }
}