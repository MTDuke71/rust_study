with open("day01_example.txt", "r") as file:
    for line in file:
        count_open=line.count('(')
        count_closed=line.count(')')
        print(count_open,count_closed,count_open-count_closed)
        print(line.strip()) 
