using System.Text;

class Day10 : Solver {
    public void solve(string input) {
        var instructions = input.Trim().Split("\n");

        long prob1 = 0;
        StringBuilder prob2 = new StringBuilder();

        // This is 1-based so that we can use the same counting as the
        // problem description.
        var cycle = 1;

        long? addInNextCycle = null;
        // Line of current instruction
        var pc = 0;

        long x = 1;
        while (pc < instructions.Length) {
            var inst = instructions[pc];

            // Problem 1
            if (cycle >= 20 && (cycle - 20) % 40 == 0) {
                prob1 += cycle * x;
            }

            // Problem 2
            var positionInRow = (cycle - 1) % 40;
            if (Math.Abs(positionInRow - x) <= 1) {
                prob2.Append("#");
            }
            else {
                prob2.Append(".");
            }

            if (positionInRow == 40 - 1) {
                prob2.AppendLine();
            }

            // General
            if (addInNextCycle != null) {
                x += (long)addInNextCycle;
                pc += 1;
                addInNextCycle = null;
            }
            else if (inst == "noop") {
                pc += 1;
            }
            else if (inst.StartsWith("addx")) {
                addInNextCycle = int.Parse(inst.Split(" ")[1]);
            }
            else {
                throw new Exception("unexpected instruction " + inst);
            }

            cycle += 1;
        }

        Console.WriteLine($"Problem 1: {prob1}");
        Console.WriteLine("Problem 2");
        Console.WriteLine(prob2);
    }
}