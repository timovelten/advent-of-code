class Day1 : Solver
{
    public void solve(string input)
    {
        var amounts = input.Split("\n\n").Select(block =>
        {
            return block
                .Split("\n").Where(x => !String.IsNullOrWhiteSpace(x))
                .Select(x => long.Parse(x)).Sum();
        }).ToList();

        Console.WriteLine($"Problem 1: {amounts.Max()}");
        Console.WriteLine($"Problem 2: {amounts.TakeLast(3).Sum()}");
    }
}