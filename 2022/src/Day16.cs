using Sprache;

class Day16 : Solver {
    List<string> vertices = new List<string>();
    Dictionary<string, int> vertexToIdx = new Dictionary<string, int>();


    List<List<int>> adj = new List<List<int>>();
    List<int> rate = new List<int>();
    int[,] dist = new int[0,0];
    int openableValvesCount;

    public void solve(string input) {
        var valves = input.Trim().Split("\n")
            .Select(line => ParseLine.Parse(line))
            .ToList();

        // Sort the valves with positive rate to the beginning so that we can iterate over them better
        valves.Sort((x, y) => y.rate.CompareTo(x.rate));
        openableValvesCount = valves.Where(x => x.rate > 0).Count();

        if (openableValvesCount > 32) throw new Exception("too many valves with non-zero flow rate");

        // Build the various dumb data structures we will need. Note that the
        // vertices in the vertices list are sorted in descending order by flow rate.
        vertices = valves.Select(x => x.name).ToList();
        for (var i = 0; i < valves.Count; i++) {
            vertexToIdx[vertices[i]] = i;
        }

        var start = 0;
        for (var i = 0; i < vertices.Count; i++) {
            var v = valves[i];
            rate.Insert(i, v.rate);
            adj.Insert(i, v.leadsTo.Select(x => vertexToIdx[x]).ToList());

            if (v.name == "AA") start = i;
        }

        // Pre-compute distances between the vertices using bfs for every vertex
        // (Some notes on floyd warshall are at the bottom)
        dist = new int[vertices.Count,vertices.Count];
        for (var from = 0; from < vertices.Count; from++) {
            // Calculate distance between the node "from" and every other node.
            var queue = new Queue<int>(new int[] { from });
            var visited = new HashSet<int>() { from };
            dist[from, from] = 0;

            while (queue.Count > 0) {
                var v = queue.Dequeue();
                foreach (var to in adj[v]) {
                    if (visited.Contains(to)) continue;

                    queue.Enqueue(to);
                    visited.Add(to);
                    dist[from, to] = dist[from, v] + 1;
                }
            }
        }

        var part1 = f(start, 0, 30);
        Console.WriteLine($"Problem 1: {part1}");

        var part2 = g(start, start, 0, 26, 26);
        Console.WriteLine($"Problem 2: {part2}");
    }

    long f(int pos, uint opened, int rem) {
        if (rem <= 1) return 0;

        long best = 0;
        for (var next = 0; next < openableValvesCount; next++) {
            if (((opened >> next) & 1) == 1) continue;
            var newRem = rem - (dist[pos, next] + 1);
            if (newRem > 0) {
                var val = newRem * rate[next] + f(next, opened | (1u << next), newRem);
                if (val > best) best = val;
            }
        }

        return best;
    }

    long g(int pos1, int pos2, uint opened, int rem1, int rem2) {
        if (rem1 <= 1) return f(pos2, opened, rem2);

        long best = 0;
        for (var next = 0; next < openableValvesCount; next++) {
            if (((opened >> next) & 1) == 1) continue;
            var newRem = rem1 - (dist[pos1, next] + 1);
            if (newRem > 0) {
                // We could implement this with two functions (g1 and g2), but we do the "fancy" way
                // where we switch around the parameters here (thereby switching the roles of p1 and
                // p2)
                var val = newRem * rate[next] + g(pos2, next, opened | (1u << next), rem2, newRem);
                if (val > best) best = val;
            }
        }

        return best;
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

    static readonly Parser<(string name, int rate, List<string> leadsTo)> ParseLine =
        from _t1 in Parse.String("Valve ")
        from v in ParseValve
        from _t2 in Parse.String("has flow rate=")
        from rate in Parse.Digit.Many().Text()
        from _t3 in Parse.String("; tunnels lead to valves ")
            .Or(Parse.String("; tunnel leads to valve"))
        from valves in ParseValveList
        select (v, int.Parse(rate), valves.ToList());
}

// The algorithm of Floyd-Warshall is
// 1. Initialize dist[i, i] = 0, dist[i, j] = edge weight for edges (i, j) and dist[i, j] = infty otherwise
// 2. Do DP using the subproblems 
//      compute the shortest path between (i, j) using only vertices {0,...,k - 1}
//      and possibly i and j themselves
// where distances are updated using dist[i, j] = min(dist[i, j], dist[i, k] + dist[k, j])
// (ask the question: can we improve the path using the new vertex k?). When implementing this,
// take care of the integer overflow...
// This algorithm is not really faster than using bfs on every vertex tho