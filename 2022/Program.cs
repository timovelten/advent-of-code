// See https://aka.ms/new-console-template for more information
var solver = new Day1();

var input = File.ReadAllText("input/day1", System.Text.Encoding.UTF8);
solver.solve(input);

interface Solver {
    void solve(String input);
}