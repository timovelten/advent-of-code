class Day25 : Solver {
    static char[] Digits = new char[] {'=', '-', '0', '1', '2'};

    public void solve(string input) {
        long sum = 0;
        foreach (var line in input.Trim().Split()) {
            sum += snafuToLong(line);
        }
        Console.WriteLine($"Problem 1: {longToSnafu(sum)}");
    }

    static long snafuToLong(string s) {
        long ret = 0;
        long b = 1;
        
        foreach (var c in s.Trim().Reverse()) {
            int val;
            if (char.IsDigit(c)) {
                val = (int) Char.GetNumericValue(c);
            } else if (c == '-') {
                val = -1;
            } else if (c == '=') {
                val = -2;
            } else throw new InvalidOperationException();
            ret += val * b;
            b *= 5;
        }

        return ret;
    }

    static string longToSnafu(long x) {
        // Convert that into a SNAFU number. Just observe that
        // 4 * 5^n = (5 - 1) * 5^n = 5^(n + 1) + (-1) * 5^n
        // 3 * 5^n = (5 - 2) * 5^n = 5^(n + 1) + (-2) * 5^n
        var res = new List<char>();
        while (x > 0) {
            var digit = x % 5;
            x = x / 5;

            if (digit == 0) {
                res.Add('0');
            } else if (digit == 1) {
                res.Add('1');
            } else if (digit == 2) {
                res.Add('2');
            } else if (digit == 3) {
                res.Add('=');
                x += 1;
            } else if (digit == 4) {
                res.Add('-');
                x += 1;
            }
        }

        res.Reverse();
        return String.Join("", res);
    }
}