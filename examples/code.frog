struct Person = {
	str name,
	int age,
}

Person jog = {
	name: "jogua",
	age: 82,
}

str name = jog.name

int number = 32
float pi = 3.1
str message = "mmmnm i love frogs"
vec(int) numbers = [2, 3, 4, 5, 4]
map(str, int) team_scores = { "yellow": 2, "red": 5  }
set(int) passing_teams = { 2, 4, 5 }

void() main = () {
	print(message + str(number))
}

void(&int) increment = (num) {
	num.* += 1
}

int(int) plus_one = (num) {
	num + 1
}

// type of function that returns a void function
// void()()

// type of function that returns a function that returns map(int, int)
// map(int, int)()()

// type of function that takes a map of strs to functions that return ints, which returns a function which takes a float and a str and returns an int
// int(float, str)(map(str, int()))
