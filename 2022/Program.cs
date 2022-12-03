// See https://aka.ms/new-console-template for more information
var solver = new Day3();

var input = File.ReadAllText("input/day3", System.Text.Encoding.UTF8);
solver.solve(input);

interface Solver {
    void solve(String input);
}