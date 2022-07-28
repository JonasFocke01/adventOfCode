with open("input.txt", "r") as file:
    prevline = 0
    count = -1
    for line in file.readlines():
        line = line.replace("\n", "")
        if int(line) > prevline:
            prevline = int(line)
            line += " (up)"
            count += 1
        else:
            prevline = int(line)
            line += " (down)"
        print(line)
    print(count)
#right Answer: count