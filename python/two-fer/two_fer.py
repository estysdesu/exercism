def two_fer(*name):
    if len(name) > 1:
        raise Exception("only one name can be passed")
    elif len(name) == 1:
        return f"One for {name[0]}, one for me."
    else:
        return "One for you, one for me."
