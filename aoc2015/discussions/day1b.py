with open("day01_example.txt", "r") as file:
    lines = file.readlines()

    target_open = '('
    target_closed =')'
    count = 0
    index = 0
    for line in lines:
        for char in line:
            index += 1
            if char == target_open:
                count += 1
            if char == target_closed:  
                count -= 1
            if count == -1:
                print(index,count)
