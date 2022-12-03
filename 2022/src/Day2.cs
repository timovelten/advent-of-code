class Day2 : Solver
{
    enum Shape
    {
        Rock,
        Paper,
        Scissors
    }

    Shape WinsAgainst(Shape m)
    {
        return m switch
        {
            Shape.Paper => Shape.Rock,
            Shape.Rock => Shape.Scissors,
            Shape.Scissors => Shape.Paper,
            _ => throw new ArgumentException()
        };
    }

    Shape LosesAgainst(Shape m)
    {
        return m switch
        {
            Shape.Paper => Shape.Scissors,
            Shape.Rock => Shape.Paper,
            Shape.Scissors => Shape.Rock,
            _ => throw new ArgumentException()
        };
    }

    public void solve(string input)
    {
        var rounds = input
            .Trim()
            .Split("\n")
            .Select(line => line.Split(" "))
            .Select(split_line => (split_line[0], split_line[1]));

        solve1(rounds);
        solve2(rounds);
    }

    void solve1(IEnumerable<(string, string)> rounds)
    {
        var parsedRounds = rounds
            .Select(x =>
            {
                var (left, right) = x;

                var leftMove = left switch
                {
                    "A" => Shape.Rock,
                    "B" => Shape.Paper,
                    "C" => Shape.Scissors,
                    _ => throw new ArgumentException(),
                };

                var rightMove = right switch
                {
                    "X" => Shape.Rock,
                    "Y" => Shape.Paper,
                    "Z" => Shape.Scissors,
                    _ => throw new ArgumentException(),
                };

                return (leftMove, rightMove);
            });

        var totalScore = parsedRounds.Select(round =>
        {
            var (move1, move2) = round;
            return scoreRound(move1, move2);
        }).Sum();

        Console.WriteLine($"Problem 1: {totalScore}");
    }

    void solve2(IEnumerable<(string, string)> rounds)
    {
        var totalScore = rounds
            .Select(x =>
            {
                var (left, right) = x;

                var move1 = left switch
                {
                    "A" => Shape.Rock,
                    "B" => Shape.Paper,
                    "C" => Shape.Scissors,
                    _ => throw new ArgumentException(),
                };

                var move2 = right switch
                {
                    "Y" => move1,
                    "X" => WinsAgainst(move1),
                    "Z" => LosesAgainst(move1),
                    _ => throw new ArgumentException()
                };

                return scoreRound(move1, move2);
            })
            .Sum();

        Console.WriteLine($"Problem 2: {totalScore}");
    }

    int scoreRound(Shape move1, Shape move2)
    {
        var outcomeScore = 3;

        if (WinsAgainst(move1) == move2)
        {
            outcomeScore = 0;
        }
        else if (WinsAgainst(move2) == move1)
        {
            outcomeScore = 6;
        }

        var myScore = move2 switch
        {
            Shape.Rock => 1,
            Shape.Paper => 2,
            Shape.Scissors => 3,
            _ => throw new ArgumentException(),
        };

        return outcomeScore + myScore;
    }
}