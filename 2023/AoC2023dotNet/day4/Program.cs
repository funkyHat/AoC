using System.Text.RegularExpressions;

var document = File.ReadLines(Path.Combine(Environment.CurrentDirectory, "input/4"));
var lines = document as List<string> ?? document.ToList();

Console.WriteLine(lines.Select(Points).Sum());
Console.WriteLine(TotalScratchcards(lines));

internal static partial class Program
{
    private static int Points(string card)
    {
        var details = new CardDetails(card);

        if (details.OurWins == 0) return 0;
        var points = (int)Math.Pow(2, details.OurWins - 1);

        return points;
    }

    private static int TotalScratchcards(IEnumerable<string> cards)
    {
        var total = 0;
        var cardsByNumber = cards.Select(x => new CardDetails(x)).ToDictionary(x => x.CardNumber, x => x);
        var queue = new Queue<CardDetails>();
        foreach (var (_, card) in cardsByNumber) queue.Enqueue(card);

        while (queue.Count > 0)
        {
            var card = queue.Dequeue();
            total++;
            for (var i = card.CardNumber + 1; i < card.CardNumber + card.OurWins + 1; i++)
                queue.Enqueue(cardsByNumber[i]);
        }

        return total;
    }

    public struct CardDetails
    {
        public CardDetails(string card)
        {
            var all = card.Split(": ");
            var cardNumber = int.Parse(Regex.Match(all[0], @"\d+").Groups[0].Value);
            var parts = all[1].Split(" | ");
            var winning = new HashSet<int>(parts[0].Split(" ").Where(x => x.Length > 0).Select(int.Parse));
            var ourNumbers = parts[1].Split(" ").Where(x => x.Length > 0).Select(int.Parse).ToList();
            var ourWins = ourNumbers.Count(x => winning.Contains(x));

            CardNumber = cardNumber;
            OurWins = ourWins;
        }

        internal int CardNumber { get; }
        internal int OurWins { get; }
    }
}