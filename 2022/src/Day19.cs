using Sprache;

class Day19 : Solver {
    // cost1 is the cost in ore, cost2 is the cost in clay and cost3 is the cost in obsidian
    record Blueprint(
        int i,
        int oreBotCostOre,
        int clayBotCostOre,
        int obsBotCostOre, int obsBotCostClay,
        int geodeBotCostOre, int geodeBotCostObs
    ) {
        public int MaxOreCost = Math.Max(Math.Max(clayBotCostOre, oreBotCostOre), Math.Max(obsBotCostOre, geodeBotCostOre));
        public int MaxClayCost = obsBotCostClay;
        public int MaxObsCost = geodeBotCostObs;
    }

    record Balance(int ore, int clay, int obsidian) : IComparable<Balance> {
        public int CompareTo(Balance? other) {
            int x = this.ore.CompareTo(other?.ore);
            if (x == 0) {
                x = this.clay.CompareTo(other?.clay);
                if (x == 0) {
                    x = this.obsidian.CompareTo(other?.obsidian);
                }
            }
            return x;
        }

        public int Norm() {
            return ore*ore + clay*clay + obsidian*obsidian;
        }
    }

    public void solve(string input) {
        var blueprints = input.Trim().Split("\n").Select(line => {
                var (i, specs) = ParseBlueprint.Parse(line);
            var specsDict = specs.ToDictionary(x => x.Item1, x => x.Item2);

            return new Blueprint(
                i,
                specsDict["ore"].ore,
                specsDict["clay"].ore,
                specsDict["obsidian"].ore, specsDict["obsidian"].clay,
                specsDict["geode"].ore, specsDict["geode"].obsidian
            );
        });

        var startState1 = new State(0, 0, 0, 0, 1, 0, 0, 24);
        var prob1 = blueprints.Select(bp => bp.i * EvaluateState(startState1, bp)).Sum();
        Console.WriteLine($"Problem 1: {prob1}");

        var startState2 = new State(0, 0, 0, 0, 1, 0, 0, 32);
        var prob2 = blueprints.Take(3)
            .Select(bp => EvaluateState(startState2, bp))
            .Aggregate(1, (acc, x) => acc * x);

        Console.WriteLine($"Problem 2: {prob2}");
    }

    static int EvaluateState(State s, Blueprint bp) {
        int f(State s, int buildNext, int best, Blueprint bp) {
            if (best < s.geodes) {
                best = s.geodes;
            } else {
                // Check if the current best is so high that we can never even hope to reach it
                // (even if we use the remaining time to constantly build more geode bots)
                // This prunes A LOT of nodes, more than one would initially think.
                var bestCase = s.geodes + ((s.remaining - 1) * s.remaining) / 2;
                if (bestCase < best) return best;
            }
            
            if (s.remaining <= 1) return best;

            State? nextState = null;
            if (buildNext == 0) {
                // No need to build more bots if we are already producing as much as we could
                // possibly consume per minute
                if (s.oreBots >= bp.MaxOreCost) return -1;
                nextState = s.buildOreBot(bp);
            } else if (buildNext == 1) {
                if (s.clayBots >= bp.MaxClayCost) return best;
                nextState = s.buildClayBot(bp);
            } else if (buildNext == 2) {
                if (s.obsidianBots >= bp.MaxObsCost) return best;
                // We need clay to build an obsidian bot. If we do not have any,
                // and also no clay bots this will never work.
                if (s.clay == 0 && s.clayBots == 0) return best;
                nextState = s.buildObsidianBot(bp);
            } else if (buildNext == 3) {
                if (s.obsidian == 0 && s.obsidianBots == 0) return best;
                nextState = s.buildGeodeBot(bp);
            }

            if (nextState != null) {
                for (var buildThen = 0; buildThen < 4; buildThen++) {
                    var val = f(nextState, buildThen, best, bp);
                    if (val > best) best = val;
                }

                return best;
            } else {
                var val = f(s.passTime(), buildNext, best, bp);
                if (val > best) best = val;
            }

            return best;
        } 
        
        // Build an ore or a clay bot first
        return Math.Max(f(s, 0, 0, bp), f(s, 1, 0, bp));
    }

    record State(
        int ore, int clay, int obsidian, int geodes,
        int oreBots, int clayBots, int obsidianBots,
        int remaining
    ) {
        public State? buildOreBot(Blueprint bp) {
            if (ore < bp.oreBotCostOre) return null;
            return this with {
                oreBots = oreBots + 1,

                ore = ore + oreBots - bp.oreBotCostOre,
                clay = clay + clayBots,
                obsidian = obsidian + obsidianBots,
                remaining = remaining - 1
            };
        }

        public State? buildClayBot(Blueprint bp) {
            if (ore < bp.clayBotCostOre) return null;
            return this with {
                clayBots = clayBots + 1,

                ore = ore + oreBots - bp.clayBotCostOre,
                clay = clay + clayBots,
                obsidian = obsidian + obsidianBots,
                remaining = remaining - 1
            };
        }

        public State? buildObsidianBot(Blueprint bp) {
            if (ore < bp.obsBotCostOre || clay < bp.obsBotCostClay) return null;
            return this with {
                obsidianBots = obsidianBots + 1,

                ore = ore + oreBots - bp.obsBotCostOre,
                clay = clay + clayBots - bp.obsBotCostClay,
                obsidian = obsidian + obsidianBots,
                remaining = remaining - 1
            };
        }

        public State? buildGeodeBot(Blueprint bp) {
            if (ore < bp.geodeBotCostOre || obsidian < bp.geodeBotCostObs) return null;
            return this with {
                ore = ore + oreBots - bp.geodeBotCostOre,
                clay = clay + clayBots,
                obsidian = obsidian + obsidianBots - bp.geodeBotCostObs,
                geodes = geodes + remaining - 1,
                remaining = remaining - 1
            };
        }

        public State passTime() {
            return this with {
                ore = ore + oreBots,
                clay = clay + clayBots,
                obsidian = obsidian + obsidianBots,
                remaining = remaining - 1
            };
        }
    }

    static readonly Parser<(int, string)> ParseCost =
        from no in Parse.Digit.AtLeastOnce().Text()
        from _ws in Parse.WhiteSpace.AtLeastOnce()
        from what in Parse.Letter.Many().Text()
        select (int.Parse(no), what);

    // Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 4 ore and 19 obsidian.
    static readonly Parser<(string, (int, int, int))> ParseRobotSpec =
        from _a1 in Parse.String("Each ")
        from type in Parse.Letter.Many().Text()
        from _a2 in Parse.String(" robot costs ")
        from costs in (
            from cost in ParseCost
            from delim in Parse.String(" and ").Optional()
            select cost
        ).Many()
        from _end in Parse.Char('.')
        from _ws in Parse.WhiteSpace.Many()
        let costsDict = costs.ToDictionary(x => x.Item2, x => x.Item1)
        select (type, (
            costsDict.GetValueOrDefault("ore", 0),
            costsDict.GetValueOrDefault("clay", 0),
            costsDict.GetValueOrDefault("obsidian", 0)
        ));

    static readonly Parser<(int no, IEnumerable<(string, (int ore, int clay, int obsidian))>)> ParseBlueprint =
        from _a1 in Parse.String("Blueprint ")
        from no in Parse.Digit.AtLeastOnce().Text()
        from delim in Parse.String(": ")
        from specs in ParseRobotSpec.Many()
        select (int.Parse(no), specs);
}