
def p1(f):
    points = 0
    for l in f.read().split("\n"):
        opp, you = l.split() 
        opp = "ABC".index(opp)
        you = "XYZ".index(you)

        points += you + 1
        #print(f'opp={opp}, you={you}, mod=' + str((you-opp) %3))
        match (you - opp) % 3: #in python mod always returns the positive number
            case 0: points += 3
            case 1: points += 6
            case 2: points += 0
          
    return points

def p2(f):
    points = 0
    for l in f.read().split("\n"):
        opp, state = l.split()
        opp = "ABC".index(opp)
        
        match state:
            case "X": 
                points += 0 
                points += (opp - 1) % 3 + 1
            case "Y": 
                points += 3
                points += opp + 1
            case "Z":
                points += 6
                points += (opp + 1) % 3 + 1
        
    return points
    

if __name__ == '__main__':
    input = '../input/day02/input.txt'
    
    with open(input) as file:
        print('P1  = ' + str(p1(file)))
        
    with open(input) as file:
        print('P2  = ' + str(p2(file)))