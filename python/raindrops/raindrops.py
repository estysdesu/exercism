from collections import OrderedDict


def convert(num):
    # snd_dict = {3: "Pling", 5: "Plang", 7: "Plong"}
    snd_dict = OrderedDict([(3, "Pling"), (5, "Plang"), (7, "Plong")])
    snd = [snd_dict[k] for k in snd_dict if num % k == 0]
    if len(snd) == 0:
        return str(num)
    return "".join(snd)


# def convert(num):
#     snd_list = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
#     snd = [note for factor, note in snd_list if num % factor == 0]
#     if len(snd) == 0:
#         return str(num)
#     return "".join(snd)
