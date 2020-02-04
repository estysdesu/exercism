from typing import List


def is_pangram(phrase: str) -> bool:
    """Determines if all 26 letters of the alphabet are present in a sentence/phrase."""

    letters_seen: List[str] = []
    for ch in filter(str.isalpha, phrase.upper()):
        if ch not in letters_seen:
            letters_seen.append(ch)
    return len(letters_seen) == 26
