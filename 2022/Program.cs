// See https://aka.ms/new-console-template for more information
var solver = new Day2();

var input = File.ReadAllText("input/day2", System.Text.Encoding.UTF8);
solver.solve(input);

interface Solver {
    void solve(String input);
}