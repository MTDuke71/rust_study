import argparse

class Solution:
  filename_real_input = 'real_input.txt'
  filename_test_input = 'test_input.txt'
  
  def __init__(self, test=False):
    self.file = open(self.filename_test_input,'r').read() if test else open(self.filename_real_input,'r').read()
    self.lines = self.file.splitlines()
    
  def part1(self):
    index =0
    a=[]
    b=[]
    for line in self.lines:
      temp = line.split()
      a.append(int(temp[0]))
      b.append(int(temp[1]))
      print (temp,a[index],b[index])
      index = index + 1
    print (a,b)
    a_sorted = sorted(a)
    b_sorted = sorted(b)
    print(a_sorted, b_sorted)
    sum = 0
    for i in range(len(a_sorted)):
      sum = sum + abs(a_sorted[i]-b_sorted[i])
      print (sum)
    return sum
  
  def part2(self):
    from collections import Counter
    index =0
    a=[]
    b=[]
    for line in self.lines:
      temp = line.split()
      a.append(int(temp[0]))
      b.append(int(temp[1]))
      print (temp,a[index],b[index])
      index = index + 1
    print (a,b)
    a_sorted = sorted(a)
    b_sorted = sorted(b)
    print(a_sorted, b_sorted)
    c = Counter(b)
    print (c)
    sum = 0
    for i in range(len(a_sorted)):
      if a_sorted[i] in c:
        sum = sum + a_sorted[i]*c[a_sorted[i]]
        print (sum)
    return sum
    pass
  
if __name__ == '__main__':
  parser = argparse.ArgumentParser('Solution file')
  parser.add_argument('-part', required=True, type=int, help='Part (1/2)')
  parser.add_argument('-test', required=True, type=str, help='Test mode (True/False)')
  args = parser.parse_args()
  test = True if args.test in ['True','true'] else False
  solution = Solution(test=test)
  result = solution.part1() if args.part == 1 else solution.part2()
  print(f'Result for Part=={args.part} & Test=={test} : {result}')
