class Day12 : Solver
{
    public void solve(string input)
    {
        var parsed = input.Trim().Split("\n")
            .Select(s => s.ToCharArray())
            .ToArray();

        var height = parsed.Length;
        var width = parsed[0].Length;

        // Flatten the grid to make it easier on us (notationally, mostly)
        var grid = new char[width * height];

        // Find start and end
        int start = 0;
        int end = 0;

        foreach (var (x, y) in gridPoints(width, height))
        {
            var c = parsed[y][x];
            var ind = x + y * width;

            if (c == 'S')
            {
                c = 'a';
                start = ind;
            }
            if (c == 'E')
            {
                c = 'z';
                end = ind;
            }

            grid[ind] = c;
        }

        // Solve the problem using BFS, in light of the second part of the problem
        // we start from end, go to start and reverse the edges.
        int[] dist = Enumerable.Repeat(int.MaxValue, width * height).ToArray();

        var visited = new HashSet<int>();
        var queue = new Queue<int>();

        dist[end] = 0;
        queue.Enqueue(end);
        visited.Add(end);

        while (queue.Count > 0)
        {
            var from = queue.Dequeue();

            var neighbors = gridNeighbors(from % width, from / width, width, height)
                // Note the "reversed" direction here
                .Where(to => heightDifference(grid[from], grid[to.x + to.y * width]) >= -1)
                .Select(to => to.x + to.y * width);

            foreach (var to in neighbors)
            {
                if (visited.Contains(to))
                    continue;

                queue.Enqueue(to);
                visited.Add(to);
                dist[to] = dist[from] + 1;
            }
        }

        var prob1 = dist[start];
        Console.WriteLine($"Problem 1: {prob1}");

        var prob2 = Enumerable.Range(0, dist.Length)
            .Where(i => grid[i] == 'a')
            .Select(i => dist[i])
            .Min();
        Console.WriteLine($"Problem 2: {prob2}");
    }

    IEnumerable<(int x, int y)> gridNeighbors(int x, int y, int width, int height)
    {
        if (x > 0)
            yield return (x - 1, y);
        if (x + 1 < width)
            yield return (x + 1, y);
        if (y > 0)
            yield return (x, y - 1);
        if (y + 1 < height)
            yield return (x, y + 1);
    }

    IEnumerable<(int, int)> gridPoints(int width, int height)
    {
        for (var x = 0; x < width; x++)
        {
            for (var y = 0; y < height; y++)
            {
                yield return (x, y);
            }
        }
    }

    char at(char[,] grid, (int, int) point)
    {
        return grid[point.Item1, point.Item2];
    }

    int heightDifference(char from, char to)
    {
        return (int)to - (int)from;
    }
}