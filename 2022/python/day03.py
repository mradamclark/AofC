lines = []
with open("../input/day03/input.txt") as f:
    lines = [x.strip() for x in f.readlines()]
    
common = []

for l in lines:
    #devide each line into the two compartments, and makes them set of unique items in each.
    compartment_a_set, compartment_b_set = (set(l[0 : len(l) // 2]), set(l[len(l) // 2 : ]))
    #find the common items from the intersection of both compartments
    common.append(next(item for item in (compartment_a_set & compartment_b_set)))

sum = 0
for c in common:
    #scoring is simple since it was a linear point scale from 1-52, zero based index lookup.
     sum += "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".index(c) + 1
    
print(sum)
    