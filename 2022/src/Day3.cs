class Day3 : Solver
{
    const string AlphabetLower = "abcdefghijklmnopqrstuvwxyz";
    const string AlphabetUpper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    int charToPriority(char c)
    {
        if (Char.IsLower(c))
        {
            return AlphabetLower.IndexOf(c) + 1;
        }
        else
        {
            return AlphabetUpper.IndexOf(c) + 27;
        }
    }

    public void solve(string input)
    {
        var prob1 = input
            .Trim()
            .Split("\n")
            .Select(line =>
            {
                var mid = line.Length / 2;
                var left = line.Substring(0, mid);
                var right = line.Substring(mid, mid);

                // The input strings seem really small, does not seem worth it
                // to create a HashSet for looking up characters
                foreach (var c in right)
                {
                    if (left.Contains(c))
                    {
                        return c;
                    }
                }

                throw new Exception();
            })
            .Select(commonCharacter => charToPriority(commonCharacter))
            .Sum();

        Console.WriteLine($"Problem 1: {prob1}");

        var prob2 = input
            .Trim()
            .Split("\n")
            .Chunk(3)
            .Select(lines =>
            {
                var first = true;
                var set = new HashSet<char>();

                foreach (var line in lines)
                {
                    if (first)
                    {
                        set.UnionWith(line);
                        first = false;
                    }
                    else
                    {
                        set.IntersectWith(line);
                    }
                }

                if (set.Count != 1)
                {
                    throw new Exception("set should contain exactly one element as per the problem statement");
                }

                return charToPriority(set.ElementAt(0));
            })
            .Sum();


        Console.WriteLine($"Problem 2: {prob2}");
    }
}