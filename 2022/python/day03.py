rucksacks = []
with open("../input/day03/input.txt") as f:
    rucksacks = [x.strip() for x in f.readlines()]
    
def score_common(items):
    sum = 0
    for c in items:
        #scoring is simple since it was a linear point scale from 1-52, zero based index lookup.
        sum += "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".index(c) + 1
    return sum

common = []
for r in rucksacks:
    #devide each line into the two compartments, and makes them set of unique items in each.
    compartment_a_set, compartment_b_set = (set(r[0 : len(r) // 2]), set(r[len(r) // 2 : ]))
    #find the common items from the intersection of both compartments
    common.append(next(item for item in (compartment_a_set & compartment_b_set)))
print("2022d03p01: " + str(score_common(common)))

common = []
#get every 3rd index from the range of indexes.
for i in range(0, len(rucksacks), 3):
    #starting with every 3rd index we can create unique set of the rucksack
    elfA = set(rucksacks[i])
    elfB = set(rucksacks[i+1])
    elfC = set(rucksacks[i+2])   
    #find the common item from the intersection of all three elfs
    common.append(next(item for item in (elfA & elfB & elfC)))
print("2022d03p02: " + str(score_common(common)))
    