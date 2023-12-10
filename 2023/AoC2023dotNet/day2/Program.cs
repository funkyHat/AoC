// See https://aka.ms/new-console-template for more information

var document = File.ReadLines(Path.Combine(Environment.CurrentDirectory, "input/2"));
IEnumerable<string> lines = document as string[] ?? document.ToArray();

Console.WriteLine(Part1(lines));
Console.WriteLine(Part2(lines));

internal static partial class Program
{
    private static readonly Dictionary<string, int> Maxes = new()
        { { "red", 12 }, { "green", 13 }, { "blue", 14 } };

    private static int Part1(IEnumerable<string> lines)
    {
        var total = 0;
        foreach (var line in lines)
        {
            var (gameNumber, requiredCubes) = MinimumCounts(line);
            if (requiredCubes.Where(arg =>
                {
                    var (cube, count) = arg;
                    return Maxes[cube] < count;
                }).Any())
                continue;

            total += gameNumber;
        }

        return total;
    }

    private static int Part2(IEnumerable<string> lines)
    {
        var total = 0;
        foreach (var line in lines)
        {
            var (_, counts) = MinimumCounts(line);
            var power = counts.Values.Aggregate(1, (acc, val) => acc * val);
            total += power;
        }

        return total;
    }

    private static (int, Dictionary<string, int>) MinimumCounts(string line)
    {
        var maxes = new Dictionary<string, int>();
        var game = line.Split(": ", 2);
        var gameNum = int.Parse(game[0].Split(" ")[1]);
        var rounds = game[1].Split("; ");

        foreach (var round in rounds)
        {
            var counts = GetCounts(round);
            foreach (var (colour, count) in counts) maxes[colour] = Math.Max(maxes.GetValueOrDefault(colour, 0), count);
        }

        return (gameNum, maxes);
    }

    private static Dictionary<string, int> GetCounts(string round)
    {
        var dict = new Dictionary<string, int>();
        var counts = round.Split(", ");
        foreach (var cubes in counts)
        {
            var parts = cubes.Split(" ");
            var (count, colour) = (int.Parse(parts[0]), parts[1]);

            dict[colour] = count;
        }

        return dict;
    }
}