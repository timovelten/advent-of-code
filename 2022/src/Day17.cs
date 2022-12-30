class Day17 : Solver {
    static List<Rock> rocks = new List<Rock>() {
        new Rock(new List<(int, int)>() { (0, 0),(1, 0),(2, 0),(3, 0) }),
        new Rock(new List<(int, int)>() { (1, 2),(0, 1),(1, 1),(2, 1),(1, 0) }),
        new Rock(new List<(int, int)>() { (2, 2),(2, 1),(0, 0),(1, 0),(2, 0) }),
        new Rock(new List<(int, int)>() { (0, 3),(0, 2),(0, 1),(0, 0) }),
        new Rock(new List<(int, int)>() { (0, 1),(1, 1),(0, 0),(1, 0) }),
    };

    List<(int dx, int dy)> jets = new List<(int dx, int dy)>();
    HashSet<(int x, int y)> settledRocks = new HashSet<(int x, int y)>();
    int rockIndex = 0;
    int jetIndex = 0;
    int width = 7;
    int height = 0;
    int bottomRow = 0;

    class Rock {
        public (int x, int y)[] Points;

        public Rock(IEnumerable<(int, int)> points) {
            this.Points = points.ToArray();
        }

        public void Translate(int dx, int dy) {
            for (var i = 0; i < Points.Length; i++) {
                Points[i] = (Points[i].x + dx, Points[i].y + dy);
            }
        }

        public Rock Clone() {
            // Note that the constructor copies that by itself (even without the AsEnumerable())
            return new Rock(Points.AsEnumerable());
        }
    }

    public void solve(string input) {
        jets = input.Trim().ToCharArray().Select(x => {
            return x switch {
                '>' => (1, 0),
                '<' => (-1, 0),
                _ => throw new Exception("invalid input")
            };
        }).ToList();

        // ith entry is how much height the ith fall added
        List<int> heightDeltas = new List<int>();
        Dictionary<ulong, int> stateHashes = new Dictionary<ulong, int>();

        int periodT1= -1;
        int periodT2 = -1;

        int i = 0;
        while (true) {
            if (i % 5 == 0) {
                Simplify();
                var hash = unchecked(SettledRocksHash() * 97 + (ulong) jetIndex);
                var prev = stateHashes.GetValueOrDefault(hash, -1);
                if (prev < 0) {
                    stateHashes[hash] = i;
                } else {
                    periodT1 = prev;
                    periodT2 = i;
                    break;
                }
            }
            
            heightDeltas.Insert(i, fall());
            i += 1;
        }

        var preperiodDeltas = heightDeltas.GetRange(0, periodT1);
        var period = heightDeltas.GetRange(periodT1, periodT2 - periodT1);

        var prob1 = calculateTotalHeight(2022, preperiodDeltas, period);
        Console.WriteLine($"Problem 1: {prob1}");
        var prob2 = calculateTotalHeight(1_000_000_000_000, preperiodDeltas, period);
        Console.WriteLine($"Problem 1: {prob2}");
    }

    static long calculateTotalHeight(long i, List<int> preperiodDeltas, List<int> period) {
        long ret = 0;

        int j = 0;
        while (j < i && j < preperiodDeltas.Count) {
            ret += preperiodDeltas[j];
            j += 1;
        }
        i -= j;

        // Now comes the periodic part.
        var wholePeriod = period.Sum();
        ret += wholePeriod * (i / period.Count) + period.Take((int)(i % period.Count)).Sum();
        return ret;
    }

    int fall() {
        var heightBefore = height;

        var fallingRock = rocks[rockIndex].Clone();
        rockIndex = (rockIndex + 1) % rocks.Count;

        fallingRock.Translate(2, height + 3);

        while (true) {
            var (dx, dy) = jets[jetIndex];
            jetIndex = (jetIndex + 1) % jets.Count;

            moveIfPossible(fallingRock, dx, dy);
            
            if (!moveIfPossible(fallingRock, 0, -1)) {
                // Break if we cannot fall anymore
                break;
            }
        }

        settledRocks.UnionWith(fallingRock.Points);   

        var newHeight = fallingRock.Points.Select(p => p.y).Max() + 1;
        this.height = Math.Max(height, newHeight);

        return height - heightBefore;
    }

    void Simplify() {
        // Simplify, by trying to "move the bottom up". For that, we determine the lowest point
        // any "rock fragment" could possibly reach and say that is the floor now.
        var lowestY = int.MaxValue;

        var queue = new Queue<(int x, int y)>();
        queue.Enqueue((0, height));
        var visited = new HashSet<(int x, int y)>() { (0, height) };

        while (queue.Count > 0) {
            var (x, y) = queue.Dequeue();
            // Note how we do NOT go up (because rocks also do not go up)
            var neighbors = new (int x, int y)[] {
                (x + 1, y), (x - 1, y), (x, y - 1) 
            };
            
            foreach (var p in neighbors) {
                if (!isValid(p.x, p.y) || visited.Contains(p)) continue;
                
                queue.Enqueue(p);
                visited.Add(p);
                lowestY = Math.Min(lowestY, p.y);
            }
        }

        bottomRow = lowestY;
    }

    ulong SettledRocksHash() {
        ulong ret = 0;
        for (int y = height - 1; y >= bottomRow; y--) {
            for (int x = 0; x < width; x++) {
                if (settledRocks.Contains((x, y))) {
                    ret = unchecked(ret * 13 + 1);
                } else {
                    ret = unchecked(ret * 13 + 0);
                }
                
            }
        }

        return ret;
    }

    bool isValid(int x, int y) {
        return x >= 0 && x < width && y >= bottomRow && !settledRocks.Contains((x, y));
    }

    bool moveIfPossible(Rock rock, int dx, int dy) {
        if (dx*dx + dy*dy > 1) throw new ArgumentException();

        // Check if everything is legal
        rock.Translate(dx, dy);

        bool legal = true;
        foreach (var (x, y) in rock.Points) {
            if (!isValid(x, y)) {
                legal = false;
                break;
            }
        }

        if (!legal) {
            rock.Translate(-dx, -dy);
        }

        return legal;
    }

    void Print() {
        for (int y = height; y >= bottomRow; y--) {
            Console.Write("|");
            for (int x = 0; x < width; x++) {
                if (settledRocks.Contains((x, y))) {
                    Console.Write("#");
                } else {
                    Console.Write(".");
                }
            }
            Console.Write("|");
            Console.WriteLine();
        }
        Console.WriteLine("+-------+");
    }
}