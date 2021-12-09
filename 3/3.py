# achievement unlocked:  collect developer tears
# with a oneliner reading a file into a matrix 
# of int:
#
# [ 
#   [1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0], 
#   [0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0], 
#   [1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1],
#   ...
# ]
inputs = [list(map(int, list(l.strip()))) for l in open('./input.txt', 'r')]

# further increase our power level by feeding off
# the rage of developers via nested list comprehension
# on a single line in order to transpose our half
# baked matrix of inputs:
#
# [
#   [ 1, 0, 1, ... ],
#   [ 0, 1, 1, ... ],
#   [ 0, 1, 1, ... ],
#   ...
# ]
transposed_inputs = [[inputs[j][i] for j in range(len(inputs))] for i in range(len(inputs[0]))]

gamma_idx   = [] # most common
epsilon_idx = [] # least common
	
# count the ones and zeros in the row (formerly
# column) and record them in gamma/epsilon based
# on frequency	
for row in transposed_inputs:
	freq_1   = row.count(1)
	freq_0 = row.count(0)

	if freq_1 > freq_0:
		gamma_idx.append(1)
		epsilon_idx.append(0)
	else:
		gamma_idx.append(0)
		epsilon_idx.append(1)

# convert our results to "binary" represented as a
# string and coerce that mess to an integer.
#
# this would be way hella moar cooler if we actually
# used python's binary types.
gamma_rate   = int(''.join(map(str,gamma_idx)), 2)
epsilon_rate = int(''.join(map(str,epsilon_idx)), 2)

power_consumption = gamma_rate * epsilon_rate

print(power_consumption)