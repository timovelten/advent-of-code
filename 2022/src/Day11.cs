using Sprache;
class Day11 : Solver {
    class Monkey {
        public int No { get; set; }

        public List<long> Items { get; set; }

        public Func<long, long> Operation { get; set; }

        public long DivisbleBy { get; set; }

        public int ThrowsToTrue { get; set; }

        public int ThrowsToFalse { get; set; }

        public Monkey() {
            this.Operation = x => x;
            this.Items = new List<long>();
        }
    }

    public void solve(string input) {
        // No reasonable person would write a parser for this, but it is what it is

        var monkeys1 = MonkeyParser.Monkey.Many().Parse(input)
            .ToList();

        var monkeys2 = MonkeyParser.Monkey.Many().Parse(input)
            .ToList();

        for (var i = 0; i < monkeys1.Count; i++) {
            if (monkeys1[i].No != i)
                throw new Exception("order the monkeys correctly");
        }

        var prob1 = doTheWork(monkeys1, 20, x => x / 3);
        Console.WriteLine($"Problem 1: {prob1}");

        var mod = monkeys2.Select(m => m.DivisbleBy).Aggregate(1L, (acc, x) => acc * x);

        // Reduction: Note that we could just store the "worriedness level" x in "components"
        // i.e. we store (x mod 19, x mod 17, x mod 11, ...) and so on. We then apply every operation
        // to each component and, most importantly, we can see if it is disible by 19 by looking at
        // the x mod 19 component.
        // But, assuming the "test divisble by integers" are coprime, we can just do this instead
        // (chinese remainder theorem).
        var prob2 = doTheWork(monkeys2, 10000, x => x % mod);
        Console.WriteLine($"Problem 2: {prob2}");
    }

    long doTheWork(List<Monkey> monkeys, int rounds, Func<long, long> reduce) {
        var inspections = Enumerable.Repeat(0L, monkeys.Count).ToList();

        foreach (var _ in Enumerable.Repeat(0, rounds)) {
            for (var i = 0; i < monkeys.Count; i++) {
                var monkey = monkeys[i];

                var items = monkey.Items;
                monkey.Items = new List<long>();

                foreach (var item in items) {
                    var transformed = reduce(monkey.Operation(item));
                    if (transformed % monkey.DivisbleBy == 0) {
                        monkeys[monkey.ThrowsToTrue].Items.Add(transformed);
                    }
                    else {
                        monkeys[monkey.ThrowsToFalse].Items.Add(transformed);
                    }
                }

                inspections[i] += items.Count;
            }
        }

        inspections.Sort();
        return inspections[inspections.Count - 1] * inspections[inspections.Count - 2];
    }

    static class MonkeyParser {
        static Func<long, long> Adder(long y) {
            return x => {
                return x + y;
            };
        }

        static Func<long, long> Multiplier(long y) {
            return x => {
                return x * y;
            };
        }

        static Func<long, long> Square = x => x * x;

        static readonly Parser<long> Number = Parse
            .Digit
            .AtLeastOnce()
            .Text()
            .Select(x => long.Parse(x));

        static readonly Parser<long> OneNumber =
            from number in Number
            from comma in Parse.Char(',').Optional()
            from ws in Parse.WhiteSpace.Many()
            select number;

        static readonly Parser<IEnumerable<long>> Numbers = OneNumber.Many();

        public static readonly Parser<IEnumerable<long>> StartingItems =
            from ws1 in Parse.WhiteSpace.Many()
            from leading in Parse.String("Starting items:")
            from ws2 in Parse.WhiteSpace.Many()
            from nos in Numbers
            select nos;

        static readonly Parser<int> Identifier =
            from _ in Parse.String("Monkey ")
            from number in Number
            from colon in Parse.Char(':')
            select (int)number;

        static readonly Parser<long> DivisibleBy =
            from ws1 in Parse.WhiteSpace.Many()
            from leading in Parse.String("Test: divisible by ")
            from ws2 in Parse.WhiteSpace.Many()
            from number in Number
            select number;

        static readonly Parser<Func<long, long>> AddOperation =
            from leading in Parse.String("new = old + ")
            from no in Number
            select Adder(no);

        static readonly Parser<Func<long, long>> MultiplyOperation =
            from leading in Parse.String("new = old * ")
            from no in Number
            select Multiplier(no);

        static readonly Parser<Func<long, long>> SquareOperation =
            from leading in Parse.String("new = old * old")
            select Square;

        static readonly Parser<Func<long, long>> Operation =
            from ws1 in Parse.WhiteSpace.Many()
            from leading in Parse.String("Operation:")
            from ws2 in Parse.WhiteSpace.Many()
            from op in AddOperation.Or(MultiplyOperation).Or(SquareOperation)
            select op;

        static readonly Parser<int> ThrowsToTrue =
            from ws1 in Parse.WhiteSpace.Many()
            from leading in Parse.String("If true: throw to monkey ")
            from no in Number
            select (int)no;

        static readonly Parser<int> ThrowsToFalse =
            from ws1 in Parse.WhiteSpace.Many()
            from leading in Parse.String("If false: throw to monkey ")
            from no in Number
            select (int)no;

        public static readonly Parser<Monkey> Monkey =
            from ws in Parse.WhiteSpace.Many()
            from no in Identifier
            from starting in StartingItems
            from operation in Operation
            from test in DivisibleBy
            from iftrue in ThrowsToTrue
            from iffalse in ThrowsToFalse
            select new Monkey {
                No = no,
                Items = starting.ToList(),
                Operation = operation,
                DivisbleBy = test,
                ThrowsToTrue = iftrue,
                ThrowsToFalse = iffalse
            };
    }
}