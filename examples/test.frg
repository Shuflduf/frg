struct Frog = {
	str name,
	int age,
	int leg_count,
}

int(&Frog) jump = (jumping_frg) {
	if jumping_frg.leg_count > 0 {
		jumping_frg.leg_count -= 1
	}
	jumping_frg.leg_count
}

map(str, int) frog_ages = { "greg": 2, "grog": 1, "josch": 712 }	
vec(Frog) swamp = []
Frog wisest_frg = { name: "NONE", age: 0, leg_count: -1 }
frog_ages.iter().for_each((elem) {
	Frog new_frg = {
	name: elem.0.*
	age: elem.1.*,
	leg_count: 4,
	}
	if new_frg.age > wisest_frg.age {
		wisest_frg = new_frg.clone()
	}
	swamp.push(new_frg)
})

int jumps_remaining = jump(&wisest_frg)
@print("{jumps_remaining} jumps left on {}", wisest_frg.name)
