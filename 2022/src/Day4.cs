class Day4 : Solver
{
    class Range {
        long Start { get; set; }
        long End { get; set; }

        public Range(long start, long end) {
            if (end < start) {
                throw new ArgumentException("start must be smaller than end");
            }

            Start = start;
            End = end;
        }

        public bool Contains(long x) {
            return Start <= x && x <= End;
        }

        public bool IsSubsetOf(Range other) {
            return Start >= other.Start && End <= other.End;
        }

        public bool Intersects(Range other) {
            var a = this;
            var b = other;

            if (b.Start < a.Start) {
                (a, b) = (b, a);
            }

            return a.Contains(b.Start);
        }
    }

    public void solve(string input)
    {
        var assignments = input
            .Trim()
            .Split("\n")
            .Select(line => {
                var assignments = line.Split(',')
                    .Select(assignment => {
                        var leftAndRight = assignment.Split("-")
                            .Select(x => long.Parse(x))
                            .GetEnumerator();

                        leftAndRight.MoveNext();
                        var left = leftAndRight.Current;

                        leftAndRight.MoveNext();
                        var right = leftAndRight.Current;

                        return (left, right);
                    })
                    .ToList();

                var ((s1, s2), (t1, t2)) = (assignments[0], assignments[1]);
                return (new Range(s1, s2), new Range(t1, t2));
            });

        var prob1 = 0;
        var prob2 = 0;

        foreach (var ranges in assignments) {
            var (left, right) = ranges;

            if (left.Intersects(right)) {
                prob2 += 1;

                if (left.IsSubsetOf(right) || right.IsSubsetOf(left)) {
                    prob1 += 1;
                }
            }
        }

        Console.WriteLine($"Problem 1: {prob1}");
        Console.WriteLine($"Problem 2: {prob2}");
    }
}