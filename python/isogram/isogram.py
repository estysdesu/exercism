def is_isogram(phrase: str) -> bool:
    """Determine if a string/phrase (multiple spaces/dashes allowed) is an isogram."""
    filtered = tuple(filter(str.isalpha, phrase.lower()))
    return len(set(filtered)) == len(filtered)
