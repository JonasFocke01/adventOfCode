with open("input.txt", "r") as file:
    data = []
    count = 0
    lines = file.readlines()

    for line in lines:
        data.append(line.replace("\n", ""))
    for i in range(len(data)):
        if i != 0:
            try:
                if sum([int(data[i-1]), int(data[i]), int(data[i+1])]) < sum([int(data[i]), int(data[i+1]), int(data[i+2])]):
                    count += 1
                    print(sum([int(data[i-1]), int(data[i]), int(data[i+1])]), "groesser")
                else:
                    print(sum([int(data[i-1]), int(data[i]), int(data[i+1])]), "kleiner")
            except:
                print("error")
    print(count)