class Day7 : Solver
{
    abstract class FS
    {
        public String Name { get; private set; }

        private FS(String name)
        {
            this.Name = name;
        }

        public class File : FS
        {
            public long Size { get; private set; }

            public File(string name, long size) : base(name)
            {
                this.Size = size;
            }
        }

        public class Directory : FS
        {
            public Directory? Parent { get; private set; }

            Dictionary<String, FS> children = new Dictionary<string, FS>();

            private Directory(String name, Directory? parent) : base(name)
            {
                this.Parent = parent;
            }

            public static Directory GetRootDirectory()
            {
                return new Directory("", null);
            }

            public void AddFile(String name, long size)
            {
                var file = new File(name, size);
                this.children.Add(name, file);
            }

            public void AddDir(String name)
            {
                var dir = new Directory(name, this);
                this.children.Add(name, dir);
            }

            public FS? GetSubdirectory(String name)
            {
                if (this.children.ContainsKey(name))
                {
                    return this.children[name];
                }

                return null;
            }

            public IEnumerable<FS> Children()
            {
                return this.children.Values;
            }

            public long TotalSize()
            {
                long ret = 0;

                foreach (var child in Children())
                {
                    if (child is FS.Directory dir)
                    {
                        ret += dir.TotalSize();
                    }
                    else if (child is FS.File f)
                    {
                        ret += f.Size;
                    }
                }

                return ret;
            }
        }
    }


    public void solve(string input)
    {
        FS.Directory root = FS.Directory.GetRootDirectory();
        FS.Directory cwd = root;

        foreach (var commandAndOutput in input.Trim().Split("$ "))
        {
            var lines = commandAndOutput.Trim().Split("\n");
            var command = lines[0].Trim().Split(" ");
            if (command[0] == "cd")
            {
                if (command[1] == "/")
                {
                    cwd = root;
                }
                else if (command[1] == "..")
                {
                    cwd = cwd.Parent!;
                }
                else
                {
                    if (cwd.GetSubdirectory(command[1]) is FS.Directory nextDir && nextDir != null)
                    {
                        cwd = nextDir;
                    }
                    else
                    {
                        throw new Exception("not found");
                    }
                }
            }
            else if (lines[0].StartsWith("ls"))
            {
                foreach (var line in lines[1..])
                {
                    var lineParts = line.Split(" ");
                    if (lineParts[0] == "dir")
                    {
                        cwd.AddDir(lineParts[1]);
                    }
                    else
                    {
                        cwd.AddFile(lineParts[1], long.Parse(lineParts[0]));
                    }
                }
            }
        }

        var prob1 = SolveProblem1(root);
        Console.WriteLine($"Problem 1: {prob1}");

        var usedspace = root.TotalSize();
        var prob2 = SolveProblem2(usedspace - 40000000, root);
        Console.WriteLine($"Problem 2: {prob2}");
    }

    long SolveProblem1(FS.Directory dir)
    {
        long ret = 0;
        var totalSize = dir.TotalSize();

        if (totalSize < 100000)
        {
            ret += totalSize;
        }

        foreach (var child in dir.Children())
        {
            if (child is FS.Directory childDir)
            {
                ret += SolveProblem1(childDir);
            }
        }

        return ret;
    }

    long SolveProblem2(long minSize, FS.Directory dir)
    {
        var best = long.MaxValue;

        var totalSize = dir.TotalSize();
        if (totalSize > minSize)
        {
            best = totalSize;
        }

        foreach (var child in dir.Children())
        {
            if (child is FS.Directory childDir)
            {
                best = Math.Min(best, SolveProblem2(minSize, childDir));
            }
        }

        return best;
    }
}