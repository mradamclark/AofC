def p1(f):
    l = [sum(int(i) for i in s.split()) for s in f.read().split("\n\n")]
    return max(l)

def p2(f):
    l = [sum(int(i) for i in s.split()) for s in f.read().split("\n\n")]

    total = 0
    for _ in range(1,4):
        n = max(l)
        total += n
        l.remove(n)
    
    return total

if __name__ == '__main__':
    with open('../input/day01/input.txt') as file:
        print('P1  = ' + str(p1(file)))
        
    with open('../input/day01/input.txt') as file:
        print('P2  = ' + str(p2(file)))