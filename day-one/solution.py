def main():

	total = 0

	with open("inputs", 'r') as input_file:
		for value in input_file.read().split('\n'):
			if len(value) > 1:
				if value[0] == '+':
					total += int(value[1:])
				else:
					total -= int(value[1:])

	with open("solution", "w+") as output:
		print "The solution is: {}".format(total)
		output.write("{}".format(total))

if __name__ == "__main__":
    main()