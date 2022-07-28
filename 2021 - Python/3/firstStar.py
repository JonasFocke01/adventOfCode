data = []
with open("input.txt", "r") as file:
    for line in file.readlines():
        data.append(line.replace("\n", ""))
    resGamma = ""
    resEpsilon = ""
    for index in range(12):
        counter1 = 0
        counter2 = 0
        for element in data:
            if element[index] == "1":
                counter1 += 1
            else:
                counter2 += 1
        if counter1 > counter2:
            resGamma += "1"
            resEpsilon += "0"
        else:
            resGamma += "0"
            resEpsilon += "1"
        print(int(resGamma, 2) * int(resEpsilon, 2))