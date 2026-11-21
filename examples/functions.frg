int(str, float) my_func = (a, b) {
	std.debug.print(a, b * 2.0)
	return 4
}

list(int) just_a_list = [1, 5, 6]

// function that returns a type
type(int) custom_type_generator = (number) {
	if number < 0 {
		return int
	} else {
		return float
	}
}

type Frog = struct {
	legs: int,
	age: float,
}

custom_type_generator(-2) this_is_int = 1
custom_type_generator(3) this_is_float = 3.0
