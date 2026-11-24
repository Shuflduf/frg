mod maths = mod {
	// constant (can only be acessed from this maths module and cant be assigned to)
	float PI = 3.1415

	int|int, int| add = |a, b| {
		return a + b
	}

	int|int, int| sub = |a, b| {
		return a - b
	}
}

// private variables (cant be accessed from other modules)
int _internal = 1
int _INTERNAL_CONST = 1225

int result = maths.sub(7, 11)

int|| increment = || {
	_internal += 1
	return _internal
}

# use maths.add
// int result2 = add(7, 11)
// int result2 = 7.add(11)

# use maths.*
// int result3 = 7.sub(11)

