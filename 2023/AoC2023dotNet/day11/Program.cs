using System.Text.RegularExpressions;

var file = Path.Join(Environment.CurrentDirectory, "input/11");
var universe = File.ReadLines(file).ToList();

var expanded = ExpandUniverse(universe);
var galaxies = GalaxyLocations(expanded);

Console.WriteLine(GetPairs(galaxies).Select(pair =>
{
    var (a,b)=pair;
    return Distance(a,b);
}).Sum());

internal static partial class Program
{
    private static int Distance((int, int) galaxyA, (int, int) galaxyB)
    {
        return Math.Abs(galaxyA.Item1 - galaxyB.Item1) + Math.Abs(galaxyA.Item2 - galaxyB.Item2);
    }

    private static IEnumerable<(T,T)> GetPairs<T>(IReadOnlyList<T> input)
    {
        for (var i = 0; i < input.Count-1; i++)
            for (var j = i+1; j < input.Count; j++)
                yield return (input[i], input[j]);
    }

    private static List<(int, int)> GalaxyLocations(List<string> universe)
    {
        var result = new List<(int, int)>();
        for (int y = 0; y < universe.Count; y++)
        {
            var galaxies = Regex.Matches(universe[y], "#");
            foreach (Match galaxy in galaxies)
                result.Add((galaxy.Index, y));
        }
        return result;
    }
    
    private static List<string> ExpandUniverse(IReadOnlyList<string> universe)
    {
        var blanks = new string(' ', universe[0].Length).ToList();
        var expanded = new List<List<char>>();

        foreach (var line in universe)
        {
            expanded.Add(line.ToList());
            var galaxies = Regex.Matches(line, "#");
            
            if (galaxies.Count==0)
            {
                expanded.Add(new string(' ',line.Length).ToList());
            }
            else
            {
                foreach (Match galaxy in galaxies)
                    blanks[galaxy.Index] = '#';
            }
        }

        for (int x = blanks.Count-1; x >=0; x--)
        {
            if (blanks[x]=='#') continue;
            foreach (var line in expanded)
                line.Insert(x, ' ');
        }
        return expanded.Select(x=>string.Join("", x)).ToList();
    }
}