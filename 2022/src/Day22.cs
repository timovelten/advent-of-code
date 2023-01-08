using Sprache;

class Day22 : Solver {
    interface IMove {
        Position? Move(Position pos);
    }

    static int mod(long x, int m) {
        int r = (int)(x % m);
        return x >= 0 ? r : r + m;
    }

    record Vec3(int x, int y, int z) {
        // These rotations are all ccw if the axis points right right into the eye of the observer

        public Vec3 RotateAroundX() {
            return this with { y = -z, z = y };
        }

        public Vec3 RotateAroundY() {
            return this with { x = z, z = -x };
        }

        public Vec3 RotateAroundZ() {
            return this with { x = -y, y = x };
        }

        public static Vec3 operator -(Vec3 a) {
            return new Vec3(-a.x, -a.y, -a.z);
        }

        public Vec3 RotateAbout(Vec3 axis, int times) {
            Vec3 Apply(Func<Vec3, Vec3> f, int n) {
                var ret = this;
                while (n > 0) {
                    ret = f(ret);
                    n--;
                }
                return ret;
            }

            times = mod(times, 4);
            int minusTimes = mod(-times, 4);

            return axis switch {
                Vec3(1, 0, 0) => Apply(p => p.RotateAroundX(), times),
                Vec3(-1, 0, 0) => Apply(p => p.RotateAroundX(), minusTimes),
                Vec3(0, 1, 0) => Apply(p => p.RotateAroundY(), times),
                Vec3(0, -1, 0) => Apply(p => p.RotateAroundY(), minusTimes),
                Vec3(0, 0, 1) => Apply(p => p.RotateAroundZ(), times),
                Vec3(0, 0, -1) => Apply(p => p.RotateAroundZ(), minusTimes),
                _ => throw new InvalidOperationException("only rotation about axes are supported")
            };
        }
    }

    record Position(int x, int y, int dx, int dy) {
        public Position TurnCcw() {
            // Normally, multiplication by is rotation ccw, but this coordinate system is weird 
            // because y grows downwards, so it is multiplication by (-i)
            // (dx + i dy) * (-i) = d y - dx * i
            return this with { dx = dy, dy = -dx }; 
        }

        public Position TurnCw() {
            // (dx + i dy) * i = -dy + dx i
            return this with { dx = -dy, dy = dx };
        }

        // Rotate the x, y coordinates around the natural mid point ((w - 1)/2, (w-1)/2) if (x, y)
        // is a coordinate on the face of a cube (with "origin" (0, 0)) in ccw direction
        public Position RotateFaceOnce(int w) {
            return this with {
                    x = y,
                    y = w - 1 - x,
                    dx = dy,
                    dy = -dx
            };
        }

        public Position RotateFace(int w, int times) {
            times = mod(times, 4);

            var ret = this;
            for (var i = 0; i < times; i++) {
                ret = ret.RotateFaceOnce(w);
            }
            return ret;
        }

        public Position Offset((int x, int y) p) {
            return this with { x = x + p.x, y = y + p.y };
        }
    }

    class Map : IMove {
        public static int OpenTile = 1;
        public static int SolidWall = 2;

        Dictionary<(int x, int y), int> map = new Dictionary<(int, int), int>();
        Dictionary<int, int> xMin = new Dictionary<int, int>();
        Dictionary<int, int> xMax = new Dictionary<int, int>();
        Dictionary<int, int> yMin = new Dictionary<int, int>();
        Dictionary<int, int> yMax = new Dictionary<int, int>();

        public int this[(int, int) p] {
            get {
                return map.GetValueOrDefault(p, 0);
            }
        }

        public void AddOpenTile(int x, int y) {
            map[(x, y)] = OpenTile;
            UpdateMinMax(x, y);
        }

        public void AddSolidWall(int x, int y) {
            map[(x, y)] = SolidWall;
            UpdateMinMax(x, y);
        }

        void UpdateMinMax(int x, int y) {
            xMin[y] = Math.Min(x, xMin.GetValueOrDefault(y, int.MaxValue));
            xMax[y] = Math.Max(x, xMax.GetValueOrDefault(y, int.MinValue));

            yMin[x] = Math.Min(y, yMin.GetValueOrDefault(x, int.MaxValue));
            yMax[x] = Math.Max(y, yMax.GetValueOrDefault(x, int.MinValue));
        }

        public Position? Move(Position pos) {
            var (x, y, dx, dy) = pos;
            if (dx*dx + dy*dy > 1) throw new ArgumentException();

            var (nx, ny) = (x + dx, y + dy);
            if (nx > xMax[y]) nx = xMin[y];
            if (nx < xMin[y]) nx = xMax[y];
            if (ny > yMax[x]) ny = yMin[x];
            if (ny < yMin[x]) ny = yMax[x];

            if (map[(nx, ny)] != OpenTile) return null;

            return new Position(nx, ny, dx, dy);
        }

        public Cube FoldUp() {
            return new Cube(map);
        }

        public (int x, int y) StartPoint() {
            var y = map.Keys.Select(p => p.y).Min();
            var x = xMin[y];
            while (this[(x, y)] != OpenTile) x++;
            return (x, y);
        }
    }

    class Cube : IMove {
        private const int TopEdge = 0;
        private const int RightEdge = 1;
        private const int BottomEdge = 2;
        private const int LeftEdge = 3;

        Dictionary<(int x, int y), int> points;
        int width;

        List<(int x, int y)> faces = new List<(int x, int y)>();
        int[][] edges;
        int[][] rotation;

        public Cube(Dictionary<(int x, int y), int> points) {
            this.points = points;
            edges = new int[6][];
            rotation = new int[6][];
            for (var i = 0; i < 6; i++) {
                edges[i] = new int[] { -1, -1, -1, -1 };
                rotation[i] = new int[] { -1, -1, -1, -1 };
            }

            // Determine width and a sort of tile map
            var xMax = points.Keys.Select(x => x.x).Max();
            for (var i = 1; i < 6; i++) {
                faces.Clear();
                if ((xMax + 1) % i != 0) continue;
                this.width = (xMax + 1) / i;

                for (var y = 0; y < 4; y++) {
                    for (var x = 0; x < 4; x++) {
                        if (points.GetValueOrDefault((x * this.width, y * this.width), 0) != 0) {
                            faces.Add((x * width, y * width));
                        }
                    }
                }

                if (faces.Count == 6) {
                    break;
                }
            }
            if (faces.Count != 6) throw new InvalidOperationException("could not identify 6 faces");

            // Build the (partial) adjacency list
            for (var i = 0; i < 6; i++) {
                var (x, y) = faces[i];
                var j = -1;
                if ((j = faces.IndexOf((x + width, y))) != -1) edges[i][RightEdge] = j;
                if ((j = faces.IndexOf((x - width, y))) != -1) edges[i][LeftEdge] = j;
                if ((j = faces.IndexOf((x, y + width))) != -1) edges[i][BottomEdge] = j;
                if ((j = faces.IndexOf((x, y - width))) != -1) edges[i][TopEdge] = j;
            }

            // We use a coordinate system like this
            // https://upload.wikimedia.org/wikipedia/commons/thumb/6/69/Coord_system_CA_0.svg/1024px-Coord_system_CA_0.svg.png)
            // If you want a chance to visualise this (when I say "flattened faces" I mean the puzzle input)
            // 1. First, draw an "up" arrow and an index into the "flattened" face of cube
            // 2. Imagine wrapping that around an actual cube (do this by getting some tape
            //      and literally drawing the arrow onto the side of a rubiks cube)
            
            // In the above coordinate system, the following are the coordinates of the faces (or rather their midpoints)
            // (1, 0, 0) front, (0, 0, 1) top, (-1, 0, 0) back, 
            // (0, 0, -1) bottom,(0, -1, 0) left, (0, 1, 0) right

            // For each side, we now determine a coordinate system (where the axis might be given
            // the following interpretation: zAxis is "up", yAxis is "right" and the xAxis gives
            // the coordinates of the current face)
            var coordSystems = new (Vec3 xAxis, Vec3 yAxis, Vec3 zAxis)?[6];
            coordSystems[0] = (new Vec3(1, 0, 0), new Vec3(0, 1, 0), new Vec3(0, 0, 1));

            var queue = new Queue<int>();
            queue.Enqueue(0);
            while (queue.Count > 0) {
                var x = queue.Dequeue();
                var c = coordSystems[x]!.Value;

                for (var j = 0; j < 4; j++) {
                    var y = edges[x][j];
                    if (y == -1 || coordSystems[y] != null) continue;
                    var rotateAbout = j switch {
                        TopEdge => -c.yAxis,
                        BottomEdge => c.yAxis,
                        LeftEdge => -c.zAxis,
                        RightEdge => c.zAxis,
                        _ => throw new InvalidOperationException()
                    };
                    coordSystems[y] = (
                        c.xAxis.RotateAbout(rotateAbout, 1),
                        c.yAxis.RotateAbout(rotateAbout, 1),
                        c.zAxis.RotateAbout(rotateAbout, 1)
                    );

                    queue.Enqueue(y);
                }
            }

            // xAxis always gives the "real" coordinates of the current face,
            // we need a mapping "real face" to index in the faces list
            var coordinateSystems = coordSystems.Select(x => x!.Value).ToArray();

            // Now that we have an orientation for each face, we can complete the adjacency list
            // and also, while we are at it, determine the rotation we need to make in 2d if
            // a point "goes over the edge"
            for (var i = 0; i < 6; i++) {
                var c = coordinateSystems[i];
                for (var j = 0; j < 4; j++) {
                    var rotateAbout = j switch {
                        TopEdge => -c.yAxis,
                        BottomEdge => c.yAxis,
                        LeftEdge => -c.zAxis,
                        RightEdge => c.zAxis,
                        _ => throw new InvalidOperationException()
                    };

                    // This is fast because that array has only 6 elements
                    var otherFace = c.xAxis.RotateAbout(rotateAbout, 1);
                    var otherFaceIdx = Enumerable.Range(0, 6)
                        .First(j => coordinateSystems[j].xAxis == otherFace);

                    if (edges[i][j] == -1) {
                        edges[i][j] = otherFaceIdx;
                    }

                    if (edges[i][j] != otherFaceIdx) throw new InvalidOperationException("inconsistent");
                    
                    // Determine rotation
                    var rotatedUp = c.zAxis.RotateAbout(rotateAbout, 1);
                    var otherC = coordinateSystems[otherFaceIdx];

                    if (rotatedUp == otherC.zAxis) {
                        rotation[i][j] = 0;
                    } else if (rotatedUp == -otherC.zAxis) {
                        rotation[i][j] = 2;
                    } else if (rotatedUp == otherC.yAxis) {
                        rotation[i][j] = 3;
                    } else if (rotatedUp == -otherC.yAxis) {
                        rotation[i][j] = 1;
                    } else {
                        throw new InvalidOperationException("what the fuck");
                    }
                }
            }
        }

        public int this[(int, int) p] {
            get {
                return points.GetValueOrDefault(p, 0);
            }
        }

        public Position? Move(Position absPos) {
            var currentFace = Enumerable.Range(0, 6).First(i => 
                faces[i].x <= absPos.x && absPos.x < faces[i].x + width &&
                faces[i].y <= absPos.y && absPos.y < faces[i].y + width
            );

            var pos = new Position(
                absPos.x - faces[currentFace].x, absPos.y - faces[currentFace].y,
                absPos.dx, absPos.dy
            );

            var nextPos = new Position(mod(pos.x + pos.dx, width), mod(pos.y + pos.dy, width), pos.dx, pos.dy);
        
            var ret = nextPos.Offset(faces[currentFace]);
            var edge = -1;
            if (pos.y == 0 && pos.dy == -1) {
                edge = TopEdge;
            } else if (pos.x == width - 1 && pos.dx == 1) {
                edge = RightEdge;
            } else if (pos.y == width - 1 && pos.dy == 1) {
                edge = BottomEdge;
            } else if (pos.x == 0 && pos.dx == -1) {
                edge = LeftEdge;
            }
            
            if (edge != -1) {
                ret = nextPos.RotateFace(width, rotation[currentFace][edge]).Offset(faces[edges[currentFace][edge]]);
            }

            if (this[(ret.x, ret.y)] != Map.OpenTile) return null;
            return ret;
        }
    }

    public void solve(string input) {
        var map = new Map();
        var split = input.Split("\n\n");
        var mapLines = split[0].TrimEnd().Split("\n");
        var instructions = ParseInstruction.Parse(split[1]).ToList();

        for (var y = 0; y < mapLines.Length; y++) {
            var line = mapLines[y];
            for (var x = 0; x < line.Length; x++) {
                var c = line[x];
                if (Char.IsWhiteSpace(c)) continue;

                if (c == '.') {
                    map.AddOpenTile(x, y);
                } else if (c == '#') {
                    map.AddSolidWall(x, y);
                }
            }
        }

        var start = map.StartPoint();
        Console.WriteLine($"Problem 1: {f(instructions, map, start)}");
        
        var prob2 = f(instructions, map.FoldUp(), start);
        Console.WriteLine($"Problem 2: {prob2}");
    }

    static long f(List<Instruction> instructions, IMove map, (int x, int y) start) {
        var pos = new Position(start.x, start.y, 1, 0);
        
        foreach (var inst in instructions) {
            if (inst is Turn t) {
                // This is bit weird, because of how our coordinate system is set up (adding positive
                // values to y means going down...)
                if (t.c == 'L') {
                    pos = pos.TurnCcw();
                } else {
                    pos = pos.TurnCw();
                }
            } else if (inst is StepCount s) {
                var remaining = s.x;
                while (remaining > 0) {
                    Position? nextPos = map.Move(pos);
                    if (nextPos == null) break;

                    pos = nextPos;
                    remaining--;
                }
            }
        }

        var facing = (pos.dx, pos.dy) switch {
            (1, 0) => 0,
            (0, 1) => 1,
            (-1, 0) => 2,
            (0, -1) => 3,
            _ => throw new Exception()
        };

        return (pos.y + 1) * 1000 + 4 * (pos.x + 1) + facing;
    }

    record Instruction {}
    record StepCount(int x) : Instruction {}
    record Turn(char c) : Instruction {}

    static readonly Parser<Instruction> ParseStepCount = 
        from nos in Parse.Digit.AtLeastOnce().Text()
        select new StepCount(int.Parse(nos));

    static readonly Parser<Instruction> ParseTurn =
        from c in Parse.Chars("LR")
        select new Turn(c);

    static readonly Parser<IEnumerable<Instruction>> ParseInstruction = (
        ParseStepCount.Or(ParseTurn)
    ).Many();
}