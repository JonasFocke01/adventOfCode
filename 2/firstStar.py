with open("input.txt", "r") as file:
    # posX = 0
    posY = 0
    posZ = 0
    for line in file.readlines():
        line = line.replace("\n", "")
        if "down" in line:
            posY += int(line[-1])
        if "up" in line:
            posY -= int(line[-1])
        if "forward" in line:
            posZ += int(line[-1])
    print(posY * posZ)