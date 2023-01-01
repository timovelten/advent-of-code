var problems = new Dictionary<String, Solver>
{
    {"day1", new Day1()},
    {"day2", new Day2()},
    {"day3", new Day3()},
    {"day4", new Day4()},
    {"day5", new Day5()},
    {"day6", new Day6()},
    {"day7", new Day7()},
    {"day8", new Day8()},
    {"day9", new Day9()},
    {"day10", new Day10()},
    {"day11", new Day11()},
    {"day12", new Day12()},
    {"day13", new Day13()},
    {"day14", new Day14()},
    {"day15", new Day15()},
    {"day16", new Day16()},
    {"day17", new Day17()},
    {"day18", new Day18()},
    {"day19", new Day19()}
};

foreach (var day in problems.Where(x => x.Value is Day19))
{
    var solver = day.Value;

    // Make this windows compatible...
    var input = File.ReadAllText($"input/{day.Key}", System.Text.Encoding.UTF8).Replace("\r\n", "\n");
    var watch = new System.Diagnostics.Stopwatch();
    watch.Start();
    solver.solve(input);
    watch.Stop();

    Console.WriteLine($"// {day.Key}, Took {watch.Elapsed.TotalMilliseconds} ms");
}

interface Solver
{
    void solve(String input);
}