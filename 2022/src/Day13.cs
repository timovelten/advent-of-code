using Sprache;

class Day13 : Solver {
    abstract class PacketData {
        private protected PacketData() { }
    }

    class PacketInt : PacketData {
        public int Inner { get; init; }

        public override string ToString() {
            return Inner.ToString();
        }

        public PacketList ToPacketList() {
            return new PacketList { Inner = new List<PacketData>() { this } };
        }
    }

    class PacketList : PacketData {
        public List<PacketData> Inner { get; init; } = new List<PacketData>();

        public override string ToString() {
            var core = string.Join(',', Inner.Select(x => x.ToString()));
            return $"[{core}]";
        }
    }

    // Returns a < b :<=> cmp(a, b) < 0
    private static int ComparePackets(PacketData a, PacketData b) {
        switch (a, b) {
            case (PacketInt i, PacketInt j):
                return i.Inner.CompareTo(j.Inner);
            case (PacketList l, PacketInt i):
                return ComparePackets(l, i.ToPacketList());
            case (PacketInt i, PacketList l):
                return ComparePackets(i.ToPacketList(), l);
            case (PacketList l1, PacketList l2):
                foreach (var (left, right) in l1.Inner.Zip(l2.Inner)) {
                    var res = ComparePackets(left, right);
                    if (res != 0) {
                        return res;
                    }
                }

                return l1.Inner.Count.CompareTo(l2.Inner.Count);
        }

        throw new Exception();
    }

    static class PacketParser {
        static readonly Parser<int> ParseInt =
            from no in Parse.Digit.AtLeastOnce().Text()
            select int.Parse(no);

        public static readonly Parser<PacketInt> ParsePacketInt = ParseInt.Select(x => new PacketInt { Inner = x });

        public static readonly Parser<PacketList> ParsePacketList =
            from leading in Parse.Char('[')
            from data in ParseData().Many()
            from trailing in Parse.Char(']')
            select new PacketList { Inner = data.ToList() };

        public static Parser<PacketData> ParseData() {
            var ret =
                from data in ParsePacketList.Select(x => (PacketData)x).Or(ParsePacketInt)
                from trailing in Parse.Char(',').Optional()
                select data;

            return ret;
        }
    }

    static Parser<PacketList> PacketWithTrailingWhitespace =
        from first in PacketParser.ParsePacketList
        from nl1 in Parse.WhiteSpace.Many()
        select first;

    public void solve(string input) {
        var packets = PacketWithTrailingWhitespace.Many().Parse(input).ToList();

        var prob1 = packets.Chunk(2)
            .Select((xs, i) => {
                if (ComparePackets(xs[0], xs[1]) < 0) {
                    return (i + 1);
                } else {
                    return 0;
                }
            })
            .Sum();

        Console.WriteLine($"Problem 1: {prob1}");

        var d1 = PacketParser.ParsePacketList.Parse("[[2]]");
        var d2 = PacketParser.ParsePacketList.Parse("[[6]]");
        packets.Add(d1);
        packets.Add(d2);

        packets.Sort(ComparePackets);

        var prob2 = packets.Select((x, i) => new { Index = i, Data = x })
            .Where(item => item.Data == d1 || item.Data == d2)
            .Select(data => data.Index + 1)
            .Aggregate(1L, (acc, x) => acc * x);

        Console.WriteLine($"Problem 2: {prob2}");
    }
}