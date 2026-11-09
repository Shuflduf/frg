mod std = @import("std")
mod my_lib = @import("funcs.frg")

type Frog = struct {
	float age,
	int leg_count,
	str name,
} 

// there is only one valid gender
type Gender = enum {
	Frog,
	Invalid,
}

# multiple ways to represent same thing
type Colour = enum {
	Red,
	RGB[int, int, int], -- tuple enum
	RedGreenBlue { <!-- struct enum -->
		int red,
		int green,
		int blue,
	}
}

Colour red = Colour.Red
Colour green = Colour.RGB[0, 255, 0]
Colour blue = Colour.RedGreenBlue {
	red = 0,
	green = 0,
	blue = 0,
}

type Thing = struct {
	struct {
		int thing1,
		str name,
		arr(int) thingies,
	} qualities,
	map(str, int) stats,
}

Thing my_thing = Thing {
	qualities = {
		thing1 = 752,
		name = "chosh",
		thingies = [1, 4, 6, 2]
	},
	stats = {
		"health" = 100,
		"defense" = 20
	}
}

Frog my_frg = Frog {
	age = 3.14,
	leg_count = 4,
	name = "Rocket",
}

float PI = 3.1
int counter = 2
int MAX_SIZE = 10
Gender mine = Gender.Frog

void(&frg_struct) print_name = (self) {
	@println(self.name)
}

int(int)(int) build_func = (index) {
	return (mult) { mult * index }
}

// tests must be of type bool() and return the success of the test
@test
bool() floating_test = () {
	return PI * 2 == 6.2
}

my_frg.print_name()
print_name(&my_frg)
