using System.Diagnostics;
using System.Text.RegularExpressions;

var watch = Stopwatch.StartNew();
var file = Path.Combine(Environment.CurrentDirectory, "input/5");
var document = File.ReadAllText(file);
var (seeds, maps) = Parse(document);

var time = watch.ElapsedMilliseconds;
Console.WriteLine("parsed " + time);

Console.WriteLine(seeds.Select(x => Trace(x, maps)).Min());

time = watch.ElapsedMilliseconds - time;
Console.WriteLine("part 1 " + time);


Console.WriteLine(seeds.Where((_, i) => i % 2 == 0).Zip(seeds.Where((_, i) => i % 2 == 1))
    .SelectMany((s, _) => SeedsFromRange(s.First, s.Second)).Select(s => Trace(s, maps)).Min());

time = watch.ElapsedMilliseconds - time;
Console.WriteLine("part 2 " + time);

internal static partial class Program
{
    private static (List<long>, List<CategoryMap>) Parse(string input)
    {
        var seeds = input.Split('\n').First().Split(": ").Skip(1).First().Split(" ").Select(long.Parse).ToList();
        var maps = input.Split("\n\n").Skip(1).Select(x => new CategoryMap(x)).ToList();

        return (seeds, maps);
    }

    private static IEnumerable<long> SeedsFromRange(long start, long length)
    {
        var to = start + length;
        for (var i = start; i <= to; i++) yield return i;
    }

    private static long Trace(long start, List<CategoryMap> maps)
    {
        return maps.Aggregate(start, (i, map) => map.DoMap(i));
    }
}

internal struct MapRange
{
    internal long From;
    internal long To;
    internal long Length;
    internal long Offset;

    internal MapRange(long from, long to, long length, long offset)
    {
        From = from;
        To = to;
        Length = length;
        Offset = offset;
    }
}

internal class CategoryMap
{
    private readonly List<MapRange> _mappings;
    private long? _currentOffset;
    private long? _currentRangeEnd;
    private long? _currentRangeStart;
    internal string From;
    internal string To;

    internal CategoryMap(string input)
    {
        var lines = input.Split('\n');
        var mapNames = Regex.Match(lines[0], @"^(.+)-to-(.+) map:$");
        From = mapNames.Groups[1].Value;
        To = mapNames.Groups[2].Value;
        _mappings = new List<MapRange>();

        foreach (var line in lines.Skip(1))
        {
            if (line.Length == 0) break;
            var numbers = line.Split(' ').Select(long.Parse).ToList();
            var (to, from, length) = (numbers[0], numbers[1], numbers[2]);
            _mappings.Add(new MapRange(from, from + length, length, to - from));
        }

        _mappings = _mappings.OrderBy(m => m.From).ToList();
    }

    public override string ToString()
    {
        return $"CategoryMap({From}, {To}";
    }

    internal long DoMap(long start)
    {
        if (_currentOffset.HasValue && _currentRangeStart.HasValue && _currentRangeEnd.HasValue)
            if (_currentRangeStart.Value <= start && start <= _currentRangeEnd)
                return start + _currentOffset.Value;
        foreach (var map in _mappings)
            if (map.From <= start && start <= map.To)
            {
                _currentRangeStart = map.From;
                _currentRangeEnd = map.To;
                _currentOffset = map.Offset;

                return start + map.Offset;
            }

        return start;
    }
}