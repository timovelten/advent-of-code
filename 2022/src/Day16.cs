using Sprache;

class Day16 : Solver {
    List<string> vertices = new List<string>();
    Dictionary<string, int> vertexToIdx = new Dictionary<string, int>();

    Dictionary<int, List<int>> adj = new Dictionary<int, List<int>>();
    Dictionary<int, int> rate = new Dictionary<int, int>();

    Dictionary<(int, int), int> dist = new Dictionary<(int, int), int>();

    List<int> openableGates = new List<int>();

    long maxFlowPerMinute = 0;

    public void solve(string input) {
        var parsedSortedValves = input.Trim().Split("\n")
            .Select(line => ParseLine.Parse(line))
            .ToList();
        parsedSortedValves.Sort((x, y) => y.Rate.CompareTo(x.Rate));

        vertices = parsedSortedValves.Select(x => x.Name).ToList();

        for (var i = 0; i < parsedSortedValves.Count; i++) {
            vertexToIdx[vertices[i]] = i;
        }

        // Build the various dumb data structures we will need. Note that the
        // vertices in the vertices list are sorted in descending order by flow rate.
        var start = 0;

        foreach (var v in parsedSortedValves) {
            var idx = vertexToIdx[v.Name];
            adj[idx] = v.LeadsTo.Select(x => vertexToIdx[x]).ToList();
            rate[idx] = v.Rate;

            if (v.Rate > 0) {
                openableGates.Add(idx);
            }

            if (v.Name == "AA") start = idx;

            maxFlowPerMinute += v.Rate;
        }

        // Pre-compute distances between the vertices
        foreach (var from in adj.Keys) {
            // Calculate distance between the node "from" and every other node.
            var queue = new Queue<int>();
            var visited = new HashSet<int>();

            queue.Enqueue(from);
            visited.Add(from);
            dist[(from, from)] = 0;

            while (queue.Count > 0) {
                var v = queue.Dequeue();

                foreach (var to in adj[v]) {
                    if (visited.Contains(to)) continue;

                    queue.Enqueue(to);
                    visited.Add(to);
                    dist[(from, to)] = dist[(from, v)] + 1;
                }
            }
        }

        var part1 = calc2(new int[] { start }, 0, new int[] { 30 }, new int[] { 0 }, 0);
        Console.WriteLine($"Problem 1: {part1}");

        var part2 = calc2(new int[] { start, start }, 0, new int[] { 26, 26 }, new int[] { 0, 0 }, 0);
       Console.WriteLine($"Problem 2: {part2}");
    }

    long calc2(int[] pos, long opened, int[] rem, int[] fpm, int i) {
        long best = rem.Zip(fpm).Select(x => x.First * x.Second).Sum();

        foreach (var next in openableGates) {
            if (IsBitSet(opened, next)) continue;

            var d = dist[(pos[i], next)] + 1;

            if (rem[i] > d) {
                var newPos = (int[])pos.Clone();
                var newRem = (int[])rem.Clone();
                var newFpm = (int[])fpm.Clone();

                newRem[i] -= d;
                newFpm[i] += rate[next];
                newPos[i] = next;

                var val = d * fpm[i] + calc2(newPos, SetBit(opened, next), newRem, newFpm, (i + 1) % pos.Length);

                if (val > best) {
                    best = val;
                }
            }
        }

        return best;
    }

    class Valve {
        public string Name { get; init; }

        public int Rate { get; init; } = 0;

        public List<string> LeadsTo = new List<string>();

        public Valve(string name) {
            this.Name = name;
        }
    }

    static bool IsBitSet(long l, int i) {
        return ((l >> i) & 1) == 1;
    }

    static long SetBit(long l, int i) {
        return l | (1L << i);
    }


    static readonly Parser<String> ParseValve =
        from ws1 in Parse.WhiteSpace.Many()
        from valve in Parse.Repeat(Parse.Upper, 2).Text()
        from ws2 in Parse.WhiteSpace.Many()
        select valve;

    static readonly Parser<IEnumerable<string>> ParseValveList = (
            from w in ParseValve
            from comma in Parse.Char(',').Optional()
            select w
        ).Many();

    static readonly Parser<Valve> ParseLine =
        from _t1 in Parse.String("Valve ")
        from v in ParseValve
        from _t2 in Parse.String("has flow rate=")
        from rate in Parse.Digit.Many().Text()
        from _t3 in Parse.String("; tunnels lead to valves ")
            .Or(Parse.String("; tunnel leads to valve"))
        from valves in ParseValveList
        select new Valve(v) {
            Rate = int.Parse(rate),
            LeadsTo = valves.ToList()
        };
}