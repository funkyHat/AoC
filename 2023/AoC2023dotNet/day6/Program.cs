var file = Path.Combine(Environment.CurrentDirectory, "input/6");
var document = File.ReadAllText(file);
var races = Race.Parse(document);

var winningRanges = races.Select(r => r.WinningRange()).ToList();
winningRanges.ForEach(x => Console.WriteLine($"{x.Item1} {x.Item2}"));

var results = winningRanges.Select(r => r.Item2 - r.Item1 + 1).ToList();
results.ForEach(Console.WriteLine);

Console.WriteLine(results.Aggregate(1L, (c, n) => c * n));


// part 2
var part2Result = Race.ParsePart2(document).WinningRange();
var winningMoves = part2Result.Item2 - part2Result.Item1 + 1;
Console.WriteLine($"\nPart 2\n{winningMoves}");

internal class Race
{
    private readonly long _time;
    internal readonly long Distance;

    internal Race(long time, long distance)
    {
        _time = time;
        Distance = distance;
    }

    public override string ToString()
    {
        return $"Race(Time: {_time}, Record Distance: {Distance}";
    }

    public (long, long) WinningRange()
    {
        var first = RiskiestButtonPress(true);
        var last = RiskiestButtonPress(false);

        return (first, last);
    }

    private long RiskiestButtonPress(bool shortest)
    {
        long first = 1;
        var last = _time;
        long mid = 0;

        do
        {
            mid = last - (last - first) / 2;
            var score = ScoreForTime(mid);
            var adjacentScore = ScoreForTime(shortest ? mid - 1 : mid + 1);

            if (shortest)
            {
                if (adjacentScore < score)
                {
                    if (score < Distance)
                    {
                        first = mid;
                    }
                    else if (score > Distance && adjacentScore <= Distance)
                    {
                        first = mid;
                        last = mid;
                    }
                    else
                    {
                        last = mid;
                    }
                }
                else

                {
                    last = mid;
                }
            }
            else
            {
                if (adjacentScore < score)
                {
                    if (score < Distance)
                    {
                        last = mid;
                    }
                    else if (score > Distance && adjacentScore <= Distance)
                    {
                        first = mid;
                        last = mid;
                    }
                    else
                    {
                        first = mid;
                    }
                }
                else
                {
                    first = mid;
                }
            }
        } while (first < last);

        return last;
    }

    internal long ScoreForTime(long time)
    {
        return (_time - time) * time;
    }

    private static List<int> _parseLine(string line)
    {
        return line.Split(':')[1].Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToList();
    }

    internal static List<Race> Parse(string input)
    {
        var lines = input.Split('\n');
        var times = _parseLine(lines[0]);
        var distances = _parseLine(lines[1]);

        return times.Zip(distances).Select(x =>
        {
            var (t, d) = x;
            return new Race(t, d);
        }).ToList();
    }

    private static long _parseLine2(string line)
    {
        return long.Parse(line.Split(":")[1].Split(" ", StringSplitOptions.RemoveEmptyEntries)
            .Aggregate("", (c, n) => c + n));
    }

    internal static Race ParsePart2(string input)
    {
        var lines = input.Split('\n');
        var time = _parseLine2(lines[0]);
        var distance = _parseLine2(lines[1]);

        return new Race(time, distance);
    }
}