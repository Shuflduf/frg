type Frog = struct {
	int legs,
	float age,,
}

Frog mine = Frog {
	legs = 4
	age = 0.4,,       ,
}

int my_legs = mine.legs

struct{str value} anon = struct{str value} { value = "hiii" }
str anon_message = anon.value

type|int| free_types = |index| {
	if index == 0 {
		return struct {
			int peak
		}
	} else {
		return struct {
			float peak
		}
	}
}

free_types(0) ts_is_int = free_types(0) { peak = 8 }

type float_struct_thing = free_types(924397)
float_struct_thing ts_float = float_struct_thing { peak = 3.144444 }

type|type| vec = |inner| {
	# do some stuff in here
}

vec(int) nums = [3, 1, 2] # vec will be a builtin type that can be constructed with []

vec(int)|vec(int)| sort = |og_vec| {
	# do some sorting stuff
	return [1, 2, 3]
}

vec(int) sorted = nums.sort()
# OR
vec(int) sorted = sort(nums)

void|&vec(int)| sort_in_place = |og_vec| {
	# sorting stuff
	*og_vec = [1, 2, 3]
}

sort_in_place(&nums)
// OR
nums.sort_in_place()

int ts_useless_but_yknow = struct {int some_value} {some_value = 4}.some_value
// equal to 
// int ts_useless_but_yknow = (struct {int some_value} {some_value = 4}).some_value

int|| normal_func = || {
	return 4
}

// not allowed since the type is 4
# normal_func() illegal = ??
# 4 illegal = ??

// allowed since the type is a type
# free_types(0) legal = 1


type|| fuck = || {
	return type
}

fuck()|| || || shit = || {
	return () {
		return () {
			return fuck()
		}
	}
}

# function that returns a function that returns a function that returns a function that returns a function that returns (a function that returns type) <- this is fuck
fuck()|| || || || || || big_shit = || {
	return () {
		return () {
			return () {
				return () {
					return fuck
				}
			}
		}
	}
}



# testing
# type testing = struct { int legs }
# void|int| jflkdsfj = |testing| {}

