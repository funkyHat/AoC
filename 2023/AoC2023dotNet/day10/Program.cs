using System.Diagnostics;

var file = Path.Combine(Environment.CurrentDirectory, "input/10");
var input = File.ReadLines(file).ToList();

var pipeLocations = GetPipeLayout(input);

Console.WriteLine(pipeLocations.Count / 2);

var pipeMap = GetPipeMap(input, pipeLocations);

var withInsideMarked = MarkLoopContents(pipeMap, pipeLocations);

Console.WriteLine(withInsideMarked.Select(l => l.Count(c => c == 'I')).Sum());


internal static partial class Program
{
    private static readonly (int, int)[] DirectionOffsets = { (1, 0), (0, 1), (-1, 0), (0, -1) };

    private static readonly string[] NextDirections = { "J-7", "L|J", "F-L", "7|F" };

    private static List<string> GetPipeMap(IReadOnlyList<string> input, IEnumerable<PipeSection> pipeSections)
    {
        Dictionary<(int x, int y), PipeSection> pipeLocations = pipeSections.ToDictionary(p => (p.X, p.Y), p => p);

        var onlyThePipe = new List<string>();
        for (var y = 0; y < input.Count; y++)
        {
            var newLine = new List<char>();
            var line = input[y];
            for (var x = 0; x < line.Length; x++) newLine.Add(pipeLocations.ContainsKey((x, y)) ? input[y][x] : '.');

            onlyThePipe.Add(string.Join("", newLine));
        }

        return onlyThePipe;
    }

    private static List<string> MarkLoopContents(IReadOnlyList<string> pipeMap, List<PipeSection> pipeSections)
    {
        var markedMap = new List<string>();
        foreach (var line in pipeMap)
        {
            var markedLine = new List<char>();
            var lineStart = '.';
            var inside = false;
            foreach (var c in line)
            {
                var toAdd = c == 'S' ? GetStartShape(pipeSections) : c;
                switch (toAdd)
                {
                    case '-':
                        break;
                    case 'F':
                    case 'L':
                        lineStart = toAdd;
                        break;
                    case '.':
                        if (inside) toAdd = 'I';
                        break;
                    case 'J':
                        inside = lineStart == 'L' ? inside : !inside;
                        break;
                    case '7':
                        inside = lineStart == 'F' ? inside : !inside;
                        break;
                    case '|':
                        inside = !inside;
                        break;
                    default:
                        throw new ArgumentException("unexpected char", $"{c}");
                }

                markedLine.Add(toAdd);
            }

            markedMap.Add(string.Join("", markedLine));
        }

        return markedMap;
    }

    private static List<PipeSection> GetPipeLayout(List<string> input)
    {
        var pipe = new List<PipeSection> { FindStart(input) };

        do
        {
            pipe.Add(FindNext(input, pipe[^1]));
        } while (pipe[^1].Direction != -1);

        pipe.RemoveAt(pipe.Count - 1);
        return pipe;
    }

    private static PipeSection FindStart(List<string> input)
    {
        int y;
        var x = 0;
        for (y = 0; y < input.Count; y++)
        {
            x = input[y].IndexOf('S');
            if (x != -1) break;
        }

        var direction = -1;
        for (var d = 0; d < DirectionOffsets.Length; d++)
        {
            var (xOffset, yOffset) = DirectionOffsets[d];
            var (xI, yI) = (x + xOffset, y + yOffset);
            if (xI >= 0 && yI >= 0 && xI < input[0].Length && yI < input.Count)
            {
                if (!NextDirections[d].Contains(input[yI][xI])) continue;
                direction = d;
                break;
            }
        }

        Debug.Assert(direction != -1, "didn't find a direction for S");
        return new PipeSection(x, y, direction);
    }

    private static char GetStartShape(List<PipeSection> pipeSections)
    {
        var prev = pipeSections[^1];
        var start = pipeSections[0];

        var directions = new List<int> { (prev.Direction + 2) % 4, start.Direction };
        directions.Sort();
        var directionPair = (directions[0], directions[1]);

        return directionPair switch
        {
            (0, 1) => 'F',
            (0, 2) => '-',
            (0, 3) => 'L',
            (1, 2) => '7',
            (1, 3) => '|',
            (2, 3) => 'J',
            _ => throw new ArgumentException("wat", nameof(pipeSections))
        };
    }

    private static PipeSection FindNext(List<string> input, PipeSection p)
    {
        var direction = DirectionOffsets[p.Direction];
        var xN = p.X + direction.Item1;
        var yN = p.Y + direction.Item2;

        var nextTile = input[yN][xN];

        // -1 signals we found the end of the loop.
        // a little modulo math to work out the direction of the next pipe section.
        var dN = nextTile == 'S' ? -1 : (p.Direction + NextDirections[p.Direction].IndexOf(nextTile) + 3) % 4;

        return new PipeSection(xN, yN, dN);
    }
}

internal struct PipeSection
{
    internal int X { get; }
    internal int Y { get; }
    internal int Direction { get; }

    public PipeSection(int x, int y, int direction)
    {
        X = x;
        Y = y;
        Direction = direction;
    }
}