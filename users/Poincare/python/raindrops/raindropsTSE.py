def convert(number):
    drops = []
    sounddict = {3: "Pling", 5: "Plang", 7: "Plong"}
    for k in sounddict.keys():
        if number % k == 0:
            drops.append(sounddict[k])
    if len(drops) == 0:
        return str(number)
    return "".join(drops)

