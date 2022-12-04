
print("AOC 2022 Day 01")
with open('../input/day01/input.txt') as file:
    l = [sum(int(i) for i in s.split()) for s in file.read().split("\n\n")]
    print('\tP1  = ' + str(max(l)))

    total = 0
    for _ in range(1,4):
        n = max(l)
        total += n
        l.remove(n)
    
    print('\tP2  = ' + str(total))