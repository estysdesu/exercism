from typing import List


def is_pangram(phrase: str) -> bool:
    """Determines if all 26 letters of the alphabet are present in a sentence/phrase."""

    return len(set(filter(str.isalpha, phrase.upper()))) == 26
