using System.Text.RegularExpressions;

var file = Path.Combine(Environment.CurrentDirectory, "input/1");
var document = File.ReadLines(file);
IEnumerable<string> lines = document as string[] ?? document.ToArray();
var sum = lines.Sum(GetValue);

Console.WriteLine(sum);

sum = lines.Sum(GetValuePart2);

Console.WriteLine(sum);


internal static partial class Program
{
    private static readonly Dictionary<string, string> Numbers = new()
    {
        { "one", "1" },
        { "two", "2" },
        { "three", "3" },
        { "four", "4" },
        { "five", "5" },
        { "six", "6" },
        { "seven", "7" },
        { "eight", "8" },
        { "nine", "9" }
    };

    private static readonly string NumbersPattern = string.Join("|", Numbers.Keys);

    private static int GetValue(string line)
    {
        var matches = Digit().Matches(line);
        var number = matches.First().Value + matches.Last().Value;

        return int.Parse(number);
    }

    private static int GetValuePart2(string line)
    {
        var matches = Regex.Matches(line, @$"(?=(\d|{NumbersPattern}))");

        var first = GetDigit(matches[0].Groups[1].Value);
        var last = GetDigit(matches[^1].Groups[1].Value);

        return int.Parse(first + last);
    }

    private static string GetDigit(string digit)
    {
        return Numbers.GetValueOrDefault(digit, digit);
    }

    [GeneratedRegex("\\d")]
    private static partial Regex Digit();
}