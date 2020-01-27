def is_isogram(phrase: str) -> bool:
    filtered_phrase = phrase.replace(" ", "").replace("-", "").lower()
    return False if len(set(filtered_phrase)) < len(filtered_phrase) else True
