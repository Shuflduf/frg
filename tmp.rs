fn main() {
    #[derive(Clone)]
    struct Frog {
        name: &'static str,
        age: i32,
        leg_count: i32,
    }
    let mut jump: fn(&mut Frog) -> i32 = |jumping_frg| {
        if (jumping_frg.leg_count > 0) {
            jumping_frg.leg_count -= 1;
        }
        jumping_frg.leg_count
    };
    let mut frog_ages: std::collections::HashMap<&'static str, i32> =
        std::collections::HashMap::from([("greg", 2), ("grog", 1), ("josch", 712)]);
    let mut swamp: Vec<Frog> = vec![];
    let mut wisest_frg: Frog = Frog {
        name: "NONE",
        age: 0,
        leg_count: -1,
    };
    frog_ages.iter().for_each(|elem| {
        let mut new_frg: Frog = Frog {
            name: *elem.0,
            age: *elem.1,
            leg_count: 4,
        };
        if (new_frg.age > wisest_frg.age) {
            wisest_frg = new_frg.clone();
        }
        swamp.push(new_frg)
    });
    let mut jumps_remaining: i32 = jump(&mut wisest_frg);
    println!("{jumps_remaining} jumps left on {}", wisest_frg.name);
}
