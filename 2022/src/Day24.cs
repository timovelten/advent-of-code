class Day24 : Solver {
    class Ground {
        public int Up = 0;
        public int Right = 0;
        public int Down = 0;
        public int Left = 0;

        bool IsOpen() {
            return Up == 0 && Right == 0 && Down == 0 && Left == 0;
        }
    }

    int width;
    int height;

    List<Ground[]> simulations = new List<Ground[]>();

    void EnsureEnoughSimulations(int i) {
        if (i >= simulations.Count) {
            var count = simulations.Count;
            var grid = simulations.Last();
            for (var j = 0; j < simulations.Count; j++) {
                grid = simulate(grid, width, height);
                simulations.Add(grid);
            } 
        }
    }

    static int mod(long x, int m) {
        int r = (int)(x % m);
        return x >= 0 ? r : r + m;
    }

    public void solve(string input) {
        var lines = input.Trim().Split("\n");
        var width = lines[0].Length - 2;
        var height = lines.Length - 2;

        // Represent open space with Ground and a wall with null
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

        (int x, int y) pos = (0, -1);
        


        print(grid, width, height);

        for (var i = 0; i < 1_024; i++) {
            Console.WriteLine("Starting " + i);
            grid = simulate(grid, width, height);
        }


        (int x, int y) end = (width - 1, height);


    }

    static Ground[] simulate(Ground[] grid, int width, int height) {
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