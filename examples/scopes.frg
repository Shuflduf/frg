int value = 1
&int ref = &value

void(int) print = |value| {
	// prints value
}

void() print_ref = || {
	
}

{
	print(value); // 1
	value = 2
	print(value) // 2
}

print(value) // 2

{
	print(value); // 2
	int value = 3
	print(value) // 3
}

print(value) // 2
