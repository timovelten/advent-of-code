class Day6 : Solver
{
    public void solve(string input)
    {
        input = input.Trim();
        
        var prob1 = findPacket(input, 4) + 1;
        Console.WriteLine($"Problem 1: {prob1}");

        var prob2 = findPacket(input, 14) + 1;
        Console.WriteLine($"Problem 1: {prob2}");
    }

    // A naive solution, but its fast enough and I do not want to spend more time on this.
    int findPacket(string packets, int n)
    {
        var cyclicalBuf = packets.Substring(0, n).ToCharArray();

        bool hasDuplicates(char[] chars) {
            for (var i = 0; i < chars.Length; i++)
            {
                for (var j = i + 1; j < chars.Length; j++)
                {
                    if (chars[i] == chars[j])
                    {
                        return true;
                    }
                }
            }

            return false;
        }
        
        if (!hasDuplicates(cyclicalBuf)) {
            return 0;
        }

        for(var t = n; t < packets.Length; t++)
        {
            cyclicalBuf[t % n] = packets[t];
            if (!hasDuplicates(cyclicalBuf)) {
                return t;
            }
        }

        return -1;
    }
}