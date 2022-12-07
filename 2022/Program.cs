var problems = new Dictionary<String, Solver>
{
    {"day1", new Day1()},
    {"day2", new Day2()},
    {"day3", new Day3()},
    {"day4", new Day4()},
    {"day5", new Day5()},
    {"day6", new Day6()},
    {"day7", new Day7()},
};

foreach (var day in problems) {
    var solver = day.Value;

    var input = File.ReadAllText($"input/{day.Key}", System.Text.Encoding.UTF8);
    var watch = new System.Diagnostics.Stopwatch();
    watch.Start();
    solver.solve(input);
    watch.Stop();

    Console.WriteLine($"// {day.Key}, Took {watch.Elapsed.TotalMilliseconds} ms");
}

interface Solver {
    void solve(String input);
}