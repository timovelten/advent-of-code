class Day20 : Solver {
    static int mod(long x, int m) {
        int r = (int)(x % m);
        return x >= 0 ? r : r + m;
    }

    static void swap<T>(T[] arr, int i, int j) {
        var temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }

    long mixNaive(IEnumerable<long> input, int rounds) {
        // This is a very naive way to do this, but it works so we leave it in here.
        // The "right" way to do this is in the other implementation.
        var labels = input.ToArray();
        var n = labels.Length;

        // x[i] = j means that the vertex j is now at index i
        var xs = Enumerable.Range(0, n).ToArray();

        for (var round = 0; round < rounds; round++) {
            for (var v = 0; v < n; v++) {
                var idx = Enumerable.Range(0, n).First(i => xs[i] == v);
                // The mod n - 1 is really weird here, but seems to be correct so who am I to judge
                var dest = mod(idx + labels[v], n - 1);

                // Move idx to dest by repeatedly swapping
                int delta = Math.Sign(dest - idx);
                while (idx != dest) {
                    swap(xs, idx, idx + delta);
                    idx += delta;
                }
            }
        }

        var idxOfZero = Enumerable.Range(0, n).First(i => labels[xs[i]] == 0);
        long prob1 = 0; 
        for (var i = 1; i <= 3; i++) {
            prob1 += labels[xs[mod(idxOfZero + i * 1000, n)]];
        }
        return prob1;
    }

    long mix(IEnumerable<long> input, int rounds) {
        var labels = input.ToArray();
        var n = labels.Length;

        // The vertex set is 0...n - 1. The actual numbers are the "labels" of these vertices,
        // and we move around these vertices (instead of the labels).
        var pred = new int[n];
        var succ = new int[n];

        for (var i = 0; i < n; i++) {
            pred[i] = mod(i - 1, n);
            succ[i] = mod(i + 1, n);
        }

        for (var round = 0; round < rounds; round++) {
            for (var i = 0; i < n; i++) {
                // Oberservation: moving n - 1 forward or backward does not change anything
                // We can therefore convert moving forward into moving backwards. Not only that,
                // we can avoid moving more than n - 1 steps at a time. We optimise even more
                // and always pick the shorter distance here.
                // (this does roughly halve the runtime)
                var moveByFwd = mod(labels[i], n - 1);
                var moveByBck = moveByFwd - (n - 1);
                var moveBy = Math.Abs(moveByBck) < Math.Abs(moveByFwd) ? moveByBck : moveByFwd; 

                // we insert this vertex i between p and q (i.e. such that p is its 
                // predecessor and q its successor).
                int p = pred[i];
                int q = succ[i];

                // Remove vertex i from the list, we have to do that at some point anyways
                succ[p] = q;
                pred[q] = p;

                // Move "forward" if moveBy is positive
                while (moveBy > 0) {
                    (p, q) = (succ[p], succ[q]);
                    moveBy--;
                }

                // ..and backward if moveBy is negative
                while (moveBy < 0) {
                    (p, q) = (pred[p], pred[q]);
                    moveBy++;
                }

                pred[i] = p;
                succ[p] = i;

                succ[i] = q;
                pred[q] = i;
            }
        }

        var idxOfZero = Enumerable.Range(0, n).First(i => labels[i] == 0);
        long ret = 0;

        var v = idxOfZero;
        for (var i = 0; i <= 3000; i++) {
            if (i == 1000 | i == 2000 | i == 3000) ret += labels[v];
            v = succ[v];
        }

        return ret;
    }

    public void solve(string input) {
        var labels = input.Trim().Split("\n")
            .Select(line => long.Parse(line)).ToArray();

        Console.WriteLine($"Problem 1: {mix(labels, 1)}");
        Console.WriteLine($"Problem 1: {mix(labels.Select(x => x * 811589153), 10)}");
    }

    static void print<T>(IEnumerable<T> xs) {
        Console.WriteLine(String.Join(" ", xs.Select(x => x.ToString())));
    }
}