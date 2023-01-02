class Day21 : Solver {
    (string, char, string) ParseOperation(string s) {
        var split = s.Split(" ");
        if (split.Length != 3) throw new ArgumentException(s);
        return (split[0].Trim(), split[1][0], split[2].Trim());
    }

    long Evaluate(string name, Dictionary<string, string> input) {
        var s = input[name];
        try {
            return long.Parse(s);
        }
        catch (Exception) { }

        var (l, op, r) = ParseOperation(s);
        var (lhs, rhs) = (Evaluate(l, input), Evaluate(r, input));

        return op switch {
            '+' => lhs + rhs,
            '-' => lhs - rhs,
            '*' => lhs * rhs,
            '/' => lhs / rhs,
            _ => throw new ArgumentException()
        };
    }

    long? SolveForHumn(string name, long eq, Dictionary<string, string> input) {
        if (name == "humn") return eq;
        var s = input[name];

        try {
            long.Parse(s);
            return null;
        }
        catch (Exception) { }

        // Is the LHS known?
        var (lhs, op, rhs) = ParseOperation(s);
        try {
            var known = Evaluate(lhs, input);
            var x = rhs;
            // known op x = eq
            return op switch {
                '+' => SolveForHumn(x, eq - known, input),
                '-' => SolveForHumn(x, known - eq, input),
                '*' => SolveForHumn(x, eq / known, input),
                '/' => SolveForHumn(x, known / eq, input),
                _ => throw new Exception()
            };
        }
        catch (Exception) { }

        // Is the RHS known?
        try {
            var known = Evaluate(rhs, input);
            var x = lhs;
            // x op known = eq
            return op switch {
                '+' => SolveForHumn(x, eq - known, input),
                '-' => SolveForHumn(x, eq + known, input),
                '*' => SolveForHumn(x, eq / known, input),
                '/' => SolveForHumn(x, eq * known, input),
                _ => throw new Exception()
            };
        }
        catch (Exception) { }

        throw new Exception("bot sides cannot be evaluated");
    }

    public void solve(string inputStr) {
        var input = new Dictionary<string, string>();
        foreach (var line in inputStr.Trim().Split("\n")) {
            var split = line.Split(": ");
            var (name, expr) = (split[0], split[1]);
            input[name] = expr;
        }

        var prob1 = Evaluate("root", input);
        Console.WriteLine($"Problem 1: {prob1}");

        // For part 2, pretend that humn does not exist
        input.Remove("humn");

        // Now evaluate all expressions so that we do not have to do it later
        foreach (var key in input.Keys) {
            try {
                var val = Evaluate(key, input);
                input[key] = val.ToString();
            }
            catch (Exception) { }
        }

        // Change root from x op y to x - y so that we can equate it to zero
        var (_, op, _) = ParseOperation(input["root"]);
        input["root"] = input["root"].Replace(op, '-');
        var prob2 = SolveForHumn("root", 0, input);
        Console.WriteLine($"Problem 2: {prob2}");
    }
}