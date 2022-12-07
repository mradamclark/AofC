import sys
from collections import defaultdict

lines = []
with open("../input/day07/input.txt") as f:
    lines = [x.strip() for x in f.readlines()]

directory_sizes = defaultdict(int)
path = []
for line in lines:
    parts = line.strip().split()
    if parts[1] == 'cd':
        #will manage th path here. 
        if parts[2] == '..':
            path.pop()
        else:
            path.append(parts[2])
    elif parts[1] == 'ls' or parts[0] == 'dir':
        # I don't care about the ls cmd or the directory names, this is just cruft.
        continue
    else:
        # we are left with files here, so for every file, the size can be added to each dir in the path
        # putting into dict so i don't need to check if it was added or not, and if what there then i can just keep add to it.
        for i in range(1, len(path)+1):
            directory_sizes['/'.join(path[:i])] += int(parts[0])


max_fs_size = 70000000
need_space = 30000000
need_to_free = directory_sizes['/'] - (max_fs_size - need_space)

p1 = 0
p2 = sys.maxint 

# I can solve both parts in one loop.
for k,v in directory_sizes.items():
    if v <= 100000:
        p1 += v
    if v >= need_to_free:
        # find the smallest size >= to the needed to free space.
        p2 = min(p2, v)
        
print("2022d07p01: " + str(p1))
print("2022d07p02: " + str(p2))