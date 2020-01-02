def convert(number):
    drops = []
    sounddict = {3:'Pling', 5:'Plang', 7:'Plong'} 
    for x in sounddict.keys():
        if (number % x == 0):
            drops.append(sounddict[x])
    if len(drops) == 0:
        return str(number)
    else:
        return ''.join(drops)