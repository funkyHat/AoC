using System.Text.RegularExpressions;

var document = File.ReadLines(Path.Combine(Environment.CurrentDirectory, "input/3"));
var lines = document as List<string> ?? document.ToList();

var parts = GetParts(lines);
Console.WriteLine(parts.Select(x => x.Item1).Aggregate(0, (x, agg) => x + agg));
Console.WriteLine(GetGears(parts.SelectMany(x => x.Item2)).Sum());

internal static partial class Program
{
    private static List<(int, List<Symbol>)> GetParts(List<string> lines)
    {
        var parts = new List<(int, List<Symbol>)>();
        var paddedLines = PadInput(lines);
        foreach (var (line, i) in paddedLines.Select((item, index) => (item, index)))
        {
            var matches = Regex.Matches(line, @"\d+");
            foreach (Match match in matches)
            {
                var (part, symbols) = GetPartDetails(match, i, line, paddedLines);
                if (symbols.Any())
                {
                    Console.WriteLine(part.ToString());
                    parts.Add((part, symbols));
                }
            }
        }

        return parts;
    }

    private static (int, List<Symbol>) GetPartDetails(Match match, int i, string line, List<string> paddedLines)
    {
        var part = int.Parse(match.Value);
        var symbols = new List<Symbol>();
        var neighbours = new List<(int, int, int)>
        {
            (i, match.Index - 1, 1),
            (i, match.Index + match.Value.Length, 1),
            (i - 1, match.Index - 1, match.Length + 2),
            (i + 1, match.Index - 1, match.Length + 2)
        };
        foreach (var (y, x, len) in neighbours)
        foreach (Match symbolMatch in Regex.Matches(paddedLines[y].Substring(x, len), @"[^.]"))
            symbols.Add(new Symbol(symbolMatch.Value, y, x + symbolMatch.Index, part));

        return (part, symbols);
    }

    private static List<int> GetGears(IEnumerable<Symbol> symbols)
    {
        var stars = new Dictionary<(int, int), List<int>>();
        foreach (var symbol in symbols.Where(x => x.Value == "*"))
        {
            var pos = (symbol.X, symbol.Y);
            stars[pos] = stars.GetValueOrDefault(pos, new List<int>()).Append(symbol.Part).ToList();
        }

        return stars.Where(pair => pair.Value.Count == 2).Select(pair => pair.Value[0] * pair.Value[1]).ToList();
    }


    /// <summary>
    ///     Add a char `.` of padding all around so we don't have to bother checking for bounds
    /// </summary>
    /// <param name="input"></param>
    /// <returns></returns>
    private static List<string> PadInput(List<string> input)
    {
        var lineLength = input[0].Length + 2;
        var output = new List<string> { new('.', lineLength) };
        foreach (var line in input) output.Add('.' + line + '.');
        output.Add(new string('.', lineLength));

        return output;
    }

    private readonly struct Symbol
    {
        public Symbol(string v, int x, int y, int part)
        {
            Value = v;
            X = x;
            Y = y;
            Part = part;
        }

        internal string Value { get; }
        internal int X { get; }
        internal int Y { get; }
        internal int Part { get; }

        public override string ToString()
        {
            return $"({Value}, ({X}, {Y}), {Part})";
        }
    }
}