using System.Diagnostics;

var file = Path.Combine(Environment.CurrentDirectory, "input/7");
var document = File.ReadLines(file).ToList();

foreach (var jokers in new List<bool> { false, true })
{
    var plays = document.Where(x => x.Length > 0).Select(x => x.Split(" "))
        .Select(line => (new Hand(line[0], jokers), int.Parse(line[1]))).ToList();
    plays.Sort();

    Console.WriteLine(plays.Select((tuple, i) => tuple.Item2 * (i + 1)).Sum());
}

internal class Hand : IComparable<Hand>
{
    private readonly Card[] _cards;
    private readonly HandType _type;

    internal Hand(string cards, bool jokers)
    {
        _cards = cards.ToCharArray().Select(c => _getCard(c, jokers)).ToArray();
        _type = _getHandType(_cards, jokers);
    }


    public int CompareTo(Hand? other)
    {
        Debug.Assert(other != null, nameof(other) + " != null");
        if (_type != other._type) return _type - other._type;

        foreach (var (our, their) in _cards.Zip(other._cards))
            if (our != their)
                return our - their;

        return 0;
    }

    private Card _getCard(char c, bool jokers)
    {
        switch (c)
        {
            case 'A': return Card.A;
            case 'K': return Card.K;
            case 'Q': return Card.Q;
            case 'J': return jokers ? Card.Joker : Card.Jack;
            case 'T': return Card.T;
            default:
                return (Card)Enum.ToObject(typeof(Card), (int)char.GetNumericValue(c));
        }
    }

    private HandType _getHandType(Card[] cards, bool jokers)
    {
        var cardsFound = new Dictionary<Card, int>();
        foreach (var card in cards) cardsFound[card] = cardsFound.GetValueOrDefault(card, 0) + 1;

        if (jokers && cardsFound.ContainsKey(Card.Joker) && cardsFound.Count > 1)
        {
            // just reassign the jokers' count to (one of) the highest counted of the other cards.
            var max = cardsFound.Where(kvp => kvp.Key != Card.Joker).MaxBy(x => x.Value);
            cardsFound[max.Key] = max.Value + cardsFound[Card.Joker];
            cardsFound.Remove(Card.Joker);
        }

        switch (cardsFound.Count)
        {
            case 1: return HandType.FiveOfAKind;
            case 2:
            {
                return cardsFound.First().Value is 1 or 4 ? HandType.FourOfAKind : HandType.FullHouse;
            }
            case 3:
            {
                return cardsFound.Values.Any(x => x == 3) ? HandType.ThreeOfAKind : HandType.TwoPair;
            }
            case 4: return HandType.OnePair;
            default: return HandType.HighCard;
        }
    }
}

public enum HandType
{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

public enum Card
{
    Joker,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    Jack,
    Q,
    K,
    A
}