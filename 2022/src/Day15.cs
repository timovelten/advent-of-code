using System.Text.RegularExpressions;
using Microsoft.Z3;

class Day15 : Solver {
    static Regex LineRx 
        = new Regex(@"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)");

    static int Row1 = 2_000_000;
    static int Box2 = 2 * Row1;

    public void solve(string input) {
        var data = new List<(int x, int y, int r)>();
        var beacons = new List<(int x, int y)>();
        
        foreach (var line in input.Trim().Split("\n")) {
            var groups = LineRx.Match(line).Groups;
            (int x, int y) pos = (int.Parse(groups[1].Value), int.Parse(groups[2].Value));
            (int x, int y) beaconPos = (int.Parse(groups[3].Value), int.Parse(groups[4].Value));
            var r = Math.Abs(pos.x - beaconPos.x) + Math.Abs(pos.y - beaconPos.y);

            data.Add((pos.x, pos.y, r));
            beacons.Add(beaconPos);
        }

        // Determine some kind of bounding box
        var loX = data.Select(res => res.x - res.r).Min();
        var hiX = data.Select(res => res.x + res.r).Max() + 1;

        // Part 1

        // This is the number of positions that lie outside of all of the circles, i.e.
        // a beacon could be present there.
        var validLocations1 = compute(Row1, loX, hiX, data, 0, null);
        // ...but some beacons are also inside of the circles, these are also valid
        // locations and we have to add them.
        var beaconsInRow = beacons.Where(pos => pos.y == Row1)
            .ToHashSet()
            .Count();
        // Find the invalid locations by subtracting the valid ones from all positions
        var prob1 = (hiX - loX) - (validLocations1 + beaconsInRow);
        Console.WriteLine($"Problem 1: {prob1}");

        // Part 2

        Console.WriteLine($"Problem 2: {part2Z3(data)}");
        // part2Naive is really slow, but whatever
    }

    // This is not even "that" naive, but it is still really slow
    static long part2Naive(List<(int x, int y, int r)> data) {
        var validLocations2 = new List<(int x, int y)>();
        for (var row = 0; row <= Box2; row++) {
            compute(row, 0, Box2 + 1, data, 0, validLocations2);
        }
        return ((long) validLocations2[0].x) * 4_000_000L + ((long) validLocations2[0].y);
    }

    // This determines all points (x, y) such that
    //      |(x, y) - (x_i, y_i)|_1 > r_i && loX <= x < hiX
    // If results is not null, these points are then also added to the list in results.
    static int compute(int y, int loX, int hiX, List<(int x, int y, int r)> data, int i, List<(int x, int y)>? results) {
        if (loX >= hiX) return 0;
        if (i >= data.Count) {
            if (results != null) {
                for (var x = loX; x < hiX; x++) {
                    results.Add((x, y));
                }
            }
            return hiX - loX;
        }

        // Process the constraints one at a time...
        var (xi, yi, ri) = data[i];
        var a = ri + 1 - Math.Abs(yi - y);

        // So y is given, we have to simplify |xi - x| + |yi - y| >= ri + 1 
        // Of course, we can replace > ri with >= ri + 1 and then this becomes
        // |xi - x| >= a      with        a = ri + 1 - |yi - y|
        // x >= xi && x >= a + xi                                       <=> x >= max(xi, a + xi)
        // x < xi && xi - a >= x        <=> x < xi && x < xi - a + 1    <=> x < min(xi, xi - a + 1), 
        // and this "split up" space we now check (combined with the already existing loX and hiX bounds).
        var ret = 0;
        if (a <= 0) {
            // This does not add any constraints, just move on
            ret += compute(y, loX, hiX, data, i + 1, results);
        } else {
            var newLo = Math.Max(loX, Math.Max(xi, a + xi));
            var newHi = Math.Min(hiX, Math.Min(xi, xi - a + 1));

            ret += compute(y, loX, newHi, data, i + 1, results);
            ret += compute(y, newLo, hiX, data, i + 1, results);
        }

        return ret;
    }

    static long part2Z3(List<(int x, int y, int r)> data) {
        // This is here so that if we run all solutions, its reasonably fast...
        using (Context ctx = new Context()) {
            var s = ctx.MkSolver();
            var x = (IntExpr) ctx.MkConst("x", ctx.IntSort);
            var y = (IntExpr) ctx.MkConst("y", ctx.IntSort);

            s.Add(ctx.MkGe(x, ctx.MkInt(0)));
            s.Add(ctx.MkLe(x, ctx.MkInt(Box2)));
            s.Add(ctx.MkGe(y, ctx.MkInt(0)));
            s.Add(ctx.MkLe(y, ctx.MkInt(Box2)));

            foreach (var item in data) {
                var (xi, yi, ri) = (ctx.MkInt(item.x), ctx.MkInt(item.y), ctx.MkInt(item.r));
                var constr = ctx.MkGt(Abs(ctx, xi - x) + Abs(ctx, yi - y), ri);
                s.Add(constr);
            }

            s.Check();
            var m = s.Model;

            var xx = long.Parse(m.Evaluate(x).ToString());
            var yy = long.Parse(m.Evaluate(y).ToString());

            return xx * 4_000_000 + yy;
        }
    }

    static ArithExpr Abs(Context ctx, ArithExpr expr) {
        var zero = ctx.MkReal(0);
        return (ArithExpr) ctx.MkITE(ctx.MkGe(expr, zero), expr, -expr);
    }
}