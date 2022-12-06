class Day5 : Solver
{

    void printStacks(List<Stack<char>> stacks)
    {
        foreach (var stack in stacks)
        {
            foreach (var x in stack)
            {
                Console.Write(x);
            }
            Console.WriteLine();
        }
    }

    public void solve(string input)
    {
        // Note that we cannot trim the start here
        var lines = input.TrimEnd().Split("\n").ToList();
        var emptyLineIdx = lines.FindIndex(line => String.IsNullOrWhiteSpace(line));

        var infoLine = lines[emptyLineIdx - 1];
        var N = infoLine.Split(" ", StringSplitOptions.RemoveEmptyEntries)
            .Select(x => int.Parse(x))
            .Max();
        var width = (infoLine.Length + 1) / N;

        var stacks1 = Enumerable.Range(0, N).Select(_ => new Stack<char>()).ToList();
        var stacks2 = Enumerable.Range(0, N).Select(_ => new Stack<char>()).ToList();

        for (var i = emptyLineIdx - 2; i >= 0; i--)
        {
            var indexedChunks = lines[i].Chunk(width)
                .Select((x, i) => (i, x));

            foreach (var (j, chunk) in indexedChunks)
            {
                if (!char.IsWhiteSpace(chunk[1]))
                {
                    stacks1[j].Push(chunk[1]);
                    stacks2[j].Push(chunk[1]);
                }
            }
        }

        // Now we can actually parse the moves
        for (var i = emptyLineIdx + 1; i < lines.Count; i++)
        {
            var split = lines[i].Split(' ');
            var count = int.Parse(split[1]);
            var from = int.Parse(split[3]) - 1;
            var to = int.Parse(split[5]) - 1;

            // Problem 1
            for (var j = 0; j < count; j++)
            {
                var x = stacks1[from].Pop();
                stacks1[to].Push(x);
            }

            // Problem 2, use another stack for lack of a better way
            var temp = new Stack<char>();
            for (var j = 0; j < count; j++)
            {
                temp.Push(stacks2[from].Pop());
            }
            for (var j = 0; j < count; j++)
            {
                stacks2[to].Push(temp.Pop());
            }
        }

        var prob1 = new String(stacks1.Select(stack => stack.Peek()).ToArray());
        Console.WriteLine($"Problem 1: {prob1}");

        var prob2 = new String(stacks2.Select(stack => stack.Peek()).ToArray());
        Console.WriteLine($"Problem 2: {prob2}");
    }
}