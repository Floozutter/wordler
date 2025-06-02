from argparse import ArgumentParser
from collections import Counter
from collections.abc import Set
from enum import Enum
from functools import cache
from pathlib import Path

class Hint(Enum):
    NOWHERE = 0
    OTHERWHERE = 1
    THERE = 2

def guess_by_letter_frequency(_words: Set[str], remaining: Set[str]) -> str:
    # score guesses by how common their unique letters are (regardless of position)
    total_counter = Counter(c for w in remaining for c in w)
    t_score = lambda w: sum(total_counter[c] for c in frozenset(w))
    # score guesses by how common each letter is in their respective column
    column_counters: tuple[Counter[str], ...] = tuple(map(Counter, zip(*remaining)))
    c_score = lambda w: sum(counter[c] for c, counter in zip(w, column_counters))
    # choose the guess that maximizes the sum of both scores
    return max(remaining, key = lambda w: t_score(w) + c_score(w))

def guess_by_mean_filtered(words: Set[str], remaining: Set[str]) -> str:
    def mean_filtered(word: str) -> float:
        return sum(
            len(remaining) - len(filter_words(remaining, word, get_hints(solution, word)))
            for solution in remaining
        ) / len(remaining)
    return max(remaining, key = mean_filtered)

@cache
def get_hints(solution: str, guess: str) -> tuple[Hint, ...]:
    counter = Counter(solution)
    hints = [Hint.NOWHERE for _ in solution]
    for i in range(len(hints)):
        if guess[i] == solution[i]:
            hints[i] = Hint.THERE
            counter[guess[i]] -= 1
    for i in range(len(hints)):
        if counter[guess[i]] > 0:
            hints[i] = Hint.OTHERWHERE
            counter[guess[i]] -= 1
    return tuple(hints)

def filter_words(words: Set[str], guess: str, hints: tuple[Hint, ...]) -> frozenset[str]:
    return frozenset(w for w in words if get_hints(w, guess) == hints)

def parse_args() -> tuple[Path, int, str | None]:
    parser = ArgumentParser()
    parser.add_argument("wordlist", type=Path)
    parser.add_argument("-s", "--solution", type=str)
    parser.add_argument("-n", "--size", type=int, default=5)
    args = parser.parse_args()
    return args.wordlist, args.size, args.solution.upper() if args.solution else None

def main(wordlist: Path, n: int, solution: str | None) -> None:
    words = frozenset(w.upper() for w in wordlist.read_text().strip().split() if len(w) == n)
    print(f"parsed {len(words)} size-{n} words.")
    remaining = words
    if not remaining:
        return
    if solution is None:
        print("no solution provided; using manual hint input.")
        print("input hints for each guess as a string consisting of `0`s, `1`s, and `2`s.")
        print("`0` = nowhere, `1` = otherwhere, `2` = there.")
        print("if the guessed word is invalid, input an `!` character.")
    i = 0
    while remaining:
        guess = guess_by_mean_filtered(words, remaining)
        if solution is not None:
            hints = get_hints(solution, guess)
            print(f"{i+1}: {guess} {''.join(str(h.value) for h in hints)}")
        else:
            response = prompt_hints(guess)
            if response is None:
                remaining -= {guess,}
                continue
            hints = response
        if all(h is Hint.THERE for h in hints):
            break
        remaining = filter_words(remaining, guess, hints)
        i += 1

def prompt_hints(guess: str) -> tuple[Hint, ...] | None:
    while True:
        s = input(f"{guess}?: ").strip().lower()
        if s == "!":
            return None
        elif len(s) == len(guess) and all(c in "012" for c in s):
            return tuple(Hint(int(c)) for c in s)

if __name__ == "__main__":
    main(*parse_args())
