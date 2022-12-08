using System.Globalization;

class Day8 : Solver
{
    public void solve(string input)
    {
        var grid = input.Trim()
            .Split("\n")
            .Select(line =>
                line.Select(c =>
                    CharUnicodeInfo.GetDecimalDigitValue(c)).ToArray()
            )
            .ToArray();

        var width = grid[0].Length;
        var height = grid.Length;

        var prob1 = 0;
        var prob2 = 0;
        for (var y = 0; y < height; y++)
        {
            for (var x = 0; x < width; x++)
            {
                if (isVisible(grid, width, height, x, y) != 0)
                {
                    prob1 += 1;
                }

                prob2 = Math.Max(scenicScore(grid, width, height, x, y), prob2);
            }
        }

        Console.WriteLine($"Problem 1: {prob1}");
        Console.WriteLine($"Problem 2: {prob2}");
    }

    static (int, int)[] Directions = { (1, 0), (-1, 0), (0, 1), (0, -1) };

    byte isVisible(int[][] grid, int width, int height, int x, int y)
    {
        byte ret = 0;

        int val = grid[y][x];
        foreach (var (dx, dy) in Directions)
        {
            bool visible = true;

            for (var k = 1; true; k++)
            {
                var xx = x + k * dx;
                var yy = y + k * dy;

                if (xx < 0 || xx >= width) break;
                if (yy < 0 || yy >= height) break;

                if (grid[y + k * dy][x + k * dx] >= val)
                {
                    visible = false;
                    break;
                }
            }

            if (visible)
            {
                ret += 1;
            }

            ret *= 2;
        }

        return ret;
    }

    int scenicScore(int[][] grid, int width, int height, int x, int y)
    {
        int ret = 1;

        int val = grid[y][x];
        foreach (var (dx, dy) in Directions)
        {
            int k = 1;
            while (true)
            {
                var xx = x + k * dx;
                var yy = y + k * dy;

                if (xx < 0 || xx >= width || yy < 0 || yy >= height)
                {
                    // This is weird, but it works
                    k -= 1;
                    break;
                }

                if (grid[y + k * dy][x + k * dx] >= val)
                {
                    break;
                }

                k += 1;
            }

            ret *= k;
        }

        return ret;
    }
}