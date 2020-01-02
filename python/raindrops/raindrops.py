def convert(num):
    snd_dict = {3: "Pling", 5: "Plang", 7: "Plong"}
    snd = [snd_dict[k] for k in snd_dict if num % k == 0]
    if len(snd) == 0:
        return str(num)
    return "".join(snd)

