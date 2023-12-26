using System.Text.RegularExpressions;

var document = File.ReadAllText(Path.Combine(Environment.CurrentDirectory, "input/8"));
var (directions, lookups) = Parse(document);

Console.WriteLine(Follow(directions, lookups, "AAA", "ZZZ$"));
Console.WriteLine(GhostsFollow(directions, lookups));

internal partial class Program
{
    private static long GhostsFollow(string directions, Dictionary<string, (string, string)> lookups)
    {
        var startingPoints = lookups.Keys.Where(x => x[2] == 'A');
        return startingPoints.Select(x => Follow(directions, lookups, x, "Z$")).Aggregate(LowestCommonMultiple);
    }

    private static long LowestCommonMultiple(long a, long b)
    {
        return Math.Abs(a * b) / GreatestCommonDenominator(a, b);
    }

    private static long GreatestCommonDenominator(long a, long b)
    {
        while (true)
        {
            if (b == 0) return a;
            var a1 = a;
            a = b;
            b = a1 % b;
        }
    }

    private static long Follow(string directions, Dictionary<string, (string, string)> lookups, string start,
        string match)
    {
        var steps = 0;
        var current = lookups[start];

        foreach (var direction in Cycle(directions.ToList()))
        {
            steps++;
            var next = direction == 'L' ? current.Item1 : current.Item2;
            current = lookups[next];

            if (Regex.Match(next, match).Success)
                break;
        }

        return steps;
    }

    private static (string, Dictionary<string, (string, string)>) Parse(string input)
    {
        var parts = input.Split("\n\n");
        var map = parts[1];

        var directions = parts[0];
        var lookups = new Dictionary<string, (string, string)>();

        foreach (var line in map.Split('\n', StringSplitOptions.RemoveEmptyEntries))
        {
            var match = MapLookupRegex().Match(line);
            var from = match.Groups[1].Value;
            var left = match.Groups[2].Value;
            var right = match.Groups[3].Value;

            lookups[from] = (left, right);
        }

        return (directions, lookups);
    }

    private static IEnumerable<T> Cycle<T>(List<T> items)
    {
        while (true)
            foreach (var item in items)
                yield return item;
        // ReSharper disable once IteratorNeverReturns
    }

    [GeneratedRegex(@"(\w+) = \((\w+), (\w+)\)")]
    private static partial Regex MapLookupRegex();
}