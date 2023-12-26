foreach (var path in new List<string> { "input/9-ex", "input/9" })
{
    Console.WriteLine(path);

    var file = Path.Combine(Environment.CurrentDirectory, path);
    var document = File.ReadLines(file).ToList();

    var (forward, backward) = document.Select(Extrapolate).Aggregate((0L, 0L),
        (tuple, valueTuple) => (tuple.Item1 + valueTuple.Item1, tuple.Item2 + valueTuple.Item2));

    Console.WriteLine($"forward: {forward}. backward: {backward}");
}

internal partial class Program
{
    private static (long, long) Extrapolate(string history)
    {
        var series = new List<List<long>> { history.Split(' ').Select(long.Parse).ToList() };
        var last = series[0];

        while (last.Count > 1 && last.Any(x => x != 0))
        {
            var next = new List<long>();
            for (var i = 1; i < last.Count; i++) next.Add(last[i] - last[i - 1]);
            series.Add(next);

            last = series[^1];
        }

        long placeholder = 0;
        for (var i = series.Count - 1; i >= 0; i--)
        {
            placeholder += series[i][^1];
            series[i].Add(placeholder);
        }

        long startPlaceholder = 0;
        for (var i = series.Count - 1; i >= 0; i--)
        {
            startPlaceholder = series[i][0] - startPlaceholder;
            series[i].Add(startPlaceholder);
        }

        return (placeholder, startPlaceholder);
    }
}