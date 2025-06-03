import wordler
from collections.abc import Callable, Set

def h(*args: int) -> tuple[wordler.Hint, ...]:
    return tuple(map(wordler.Hint, args))

def solves(gf: Callable[[Set[str], Set[str]], str], words: Set[str], solution: str) -> bool:
    remaining = frozenset(words)
    while remaining:
        guess = gf(words, remaining)
        hints = wordler.get_hints(solution, guess)
        if all(h is wordler.Hint.THERE for h in hints):
            return True
        remaining = wordler.filter_words(guess, hints, remaining)
    return False

def test_get_hints() -> None:
    assert wordler.get_hints("", "") == h()
    assert wordler.get_hints("A", "Z") == h(0)
    assert wordler.get_hints("A", "A") == h(2)
    assert wordler.get_hints("AB", "CC") == h(0, 0)
    assert wordler.get_hints("AB", "AB") == h(2, 2)
    assert wordler.get_hints("AB", "BA") == h(1, 1)
    assert wordler.get_hints("AB", "AA") == h(2, 0)
    assert wordler.get_hints("AB", "BB") == h(0, 2)
    assert wordler.get_hints("AB", "CA") == h(0, 1)
    assert wordler.get_hints("AB", "BC") == h(1, 0)
    assert wordler.get_hints("AAAAA", "ZZZZZ") == h(0, 0, 0, 0, 0)
    assert wordler.get_hints("ABCDE", "ZYXWV") == h(0, 0, 0, 0, 0)
    assert wordler.get_hints("AAAAA", "AAAAA") == h(2, 2, 2, 2, 2)
    assert wordler.get_hints("ABCDE", "ABCDE") == h(2, 2, 2, 2, 2)
    assert wordler.get_hints("AABAA", "BBBBB") == h(0, 0, 2, 0, 0)
    assert wordler.get_hints("AABAA", "AAAAA") == h(2, 2, 0, 2, 2)
    assert wordler.get_hints("ABBBB", "CCCCA") == h(0, 0, 0, 0, 1)
    assert wordler.get_hints("ABBBB", "CCACA") == h(0, 0, 1, 0, 0)
    assert wordler.get_hints("ABBBB", "ACACA") == h(2, 0, 0, 0, 0)
    assert wordler.get_hints("ABBBB", "BBBBA") == h(1, 2, 2, 2, 1)
    assert wordler.get_hints("AAABB", "BBBAA") == h(1, 1, 0, 1, 1)
    assert wordler.get_hints("BUNNY", "NANNY") == h(0, 0, 2, 2, 2)
    assert wordler.get_hints("BUNNY", "INNER") == h(0, 1, 2, 0, 0)
    assert wordler.get_hints("BUNNY", "ENNUI") == h(0, 1, 2, 1, 0)
    assert wordler.get_hints("BUNNY", "SUNNY") == h(0, 2, 2, 2, 2)
    assert wordler.get_hints("BUNNY", "BNNNY") == h(2, 0, 2, 2, 2)
    assert wordler.get_hints("BUNNY", "BNUUY") == h(2, 1, 1, 0, 2)
    assert wordler.get_hints("BUNNY", "BUNNY") == h(2, 2, 2, 2, 2)

def test_guess_functions() -> None:
    guess_functions = (wordler.guess_by_letter_frequency, wordler.guess_by_mean_filtered)
    words = {"BNNNY", "BNUUY", "BUNNY", "ENNUI", "INNER", "NANNY"}
    for gf in guess_functions:
        assert not solves(gf, words, "?????")
        for solution in words:
            assert not solves(gf, frozenset(), solution)
            assert not solves(gf, words-{solution}, solution)
            assert solves(gf, words, solution)
