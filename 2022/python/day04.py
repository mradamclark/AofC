print("AOC 2022 Day 04")
assignments = []
with open("../input/day04/input.txt") as f:
    assignments = [x.strip() for x in f.readlines()]
 
fully_contained = 0
disjoint = 0
for asg in assignments:
    g = asg.split(",")

    elfA = set(range(int(g[0].split("-")[0]), int(g[0].split("-")[1])+1))
    elfB = set(range(int(g[1].split("-")[0]), int(g[1].split("-")[1])+1))

    if elfA.isdisjoint(elfB):
        disjoint += 1
        
    if elfA.issubset(elfB) or elfB.issubset(elfA):
        fully_contained += 1
        
print("\tP1: " + str(fully_contained))
print("\tP2: " + str(len(assignments) - disjoint))    
    