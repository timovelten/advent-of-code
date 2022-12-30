class Day18 : Solver {
    static IEnumerable<(int x, int y, int z)> allNeighbors((int x, int y, int z) p) {
        for (var dx = -1; dx <= 1; dx++) {
            for (var dy = -1; dy <= 1; dy++) {
                for (var dz = -1; dz <= 1; dz++) {
                    if (dx == 0 && dy == 0 && dz == 0) continue;
                    yield return (p.x + dx, p.y + dy, p.z + dz);
                }
            }
        }
    }

    static IEnumerable<(int x, int y, int z)> adjacentPoints((int x, int y, int z) p) {
        yield return (p.x + 1, p.y, p.z);
        yield return (p.x - 1, p.y, p.z);
        yield return (p.x, p.y + 1, p.z);
        yield return (p.x, p.y - 1, p.z);
        yield return (p.x, p.y, p.z + 1);
        yield return (p.x, p.y, p.z - 1);
    }

    public void solve(string input) {
        HashSet<(int x, int y, int z)> allPoints = input.Trim().Split('\n').Select(line => {
            var split = line.Split(',').Select(x => int.Parse(x)).ToList();
            return (split[0], split[1], split[2]);
        }).ToHashSet();

        var prob1 = allPoints.Select(p => {
            var exposedSides = 6 - adjacentPoints(p).Where(q => allPoints.Contains(q)).Count();
            return exposedSides;
        }).Sum();
        Console.WriteLine($"Problem 1: {prob1}");

        // Strategy for part 2: bfs the outside of the shape. For that, enlarge the bouding box
        // slightly then start in a corner of that bounding box and go go go.
        var (xMin, xMax) = (int.MaxValue, int.MinValue);
        var (yMin, yMax) = (int.MaxValue, int.MinValue);
        var (zMin, zMax) = (int.MaxValue, int.MinValue);

        foreach (var p in allPoints) {
            (xMin, xMax) = (Math.Min(p.x, xMin), Math.Max(p.x, xMax));
            (yMin, yMax) = (Math.Min(p.y, yMin), Math.Max(p.y, yMax));
            (zMin, zMax) = (Math.Min(p.z, zMin), Math.Max(p.z, zMax));
        }
        (xMin, xMax) = (xMin - 1, xMax + 1);
        (yMin, yMax) = (yMin - 1, yMax + 1);
        (zMin, zMax) = (zMin - 1, zMax + 1);

        var start = (xMax, yMax, zMax);

        var queue = new Queue<(int x, int y, int z)>();
        queue.Enqueue(start);

        // Visited will contain all outside points within in the "bounding cube"
        var visited = new HashSet<(int, int, int)>() { start };

        while (queue.Count > 0) {
            var p = queue.Dequeue();

            foreach (var q in adjacentPoints(p)) {
                if (q.x > xMax || q.x < xMin || q.y > yMax || q.y < yMin || q.z > zMax || q.z < zMin)
                    continue;
                if (allPoints.Contains(q) || visited.Contains(q)) continue;

                queue.Enqueue(q);
                visited.Add(q);
            }
        }

        var prob2 = allPoints.Select(p =>
            adjacentPoints(p).Where(q => visited.Contains(q)).Count()
        ).Sum();
        Console.WriteLine($"Problem 2: {prob2}");
    }
}