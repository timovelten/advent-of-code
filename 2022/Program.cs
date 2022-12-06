// See https://aka.ms/new-console-template for more information
var solver = new Day6();
var input = File.ReadAllText("input/day6", System.Text.Encoding.UTF8);

var watch = new System.Diagnostics.Stopwatch();
watch.Start();
solver.solve(input);
watch.Stop();

Console.WriteLine($"Took {watch.Elapsed.TotalMilliseconds} ms");


interface Solver {
    void solve(String input);
}