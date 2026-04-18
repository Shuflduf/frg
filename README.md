# ![icon](frg-icon.png)
*The best programming language since it's named after frogs*

# Table of Contents
1. [Showcase](#showcase)
2. [Purpose](#purpose)
3. [Features](#features)
4. [Installation](#installation)
5. [Editor Setup](#editor-setup)
6. [Usage](#usage)
7. [Style Guide](#style-guide)

# Showcase
```rs
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
```

# Purpose
frg was made for one reason:
- *The silly factor*

Basically i took some Rust code, and sillyfied it. Heres an example
```rs
fn jump(jumping_frg: &mut Frog) -> i32 {
	if jumping_frg.leg_count > 0 {
		jumping_frg.leg_count -= 1;
	}
	jumping_frg.leg_count
}
```

First, remove anything that doesn't have to be there in order to function. This includes any commas, semicolons, keywords, stuff like that
```rs
fn jump(jumping_frg: &Frog) -> i32 {
	jumping_frg.leg_count > 0 {
		jumping_frg.leg_count -= 1
	}
	jumping_frg.leg_count
}
```

Next, seperate the declaration from the implementation (make it a closure)
```rs
// we still keep types!!
let jump: fn(&Frog) -> i32 = |jumping_frg| {
	jumping_frg.leg_count > 0 {
		jumping_frg.leg_count -= 1
	}
	jumping_frg.leg_count
}
```

Lastly, rearrange everything and hide the exact data type (i32 -> int)
```rs
int(&Frog) jump = (jumping_frg) {
	jumping_frg.leg_count > 0 {
		jumping_frg.leg_count -= 1
	}
	jumping_frg.leg_count
}
```

You don't like not having an if keyword? Does that make you annoyed? Have three.
```rs
int(&Frog) jump = (jumping_frg) {
	if if if jumping_frg.leg_count > 0 {
		jumping_frg.leg_count -= 1
	}
	jumping_frg.leg_count
}
```
You want to add another parameter? And a loop? Your greed is insulting.
```rs
int(&Frog,  ,int) jump = (jumping_frg jump_count,,) {
	if if if jumping_frg.leg_count - jump_count >= 0 {
		// rust styled loops because theyre objectively better
		(0..=jump_count).for_each((_) {
			jumping_frg.leg_count -= 1
			@print("{} legs remaining", jumping_frg.leg_count)
		})
	}
	jumping_frg.leg_count
}
```

If you're insecure about the size of your -= or your >=, you should be, but also frg is perfect for you!
```rs
int(&Frog,  ,int) jump = (jumping_frg jump_count,,) {
	if if if jumping_frg.leg_count - jump_count >>>==== 0 {
		jumping_frg.leg_count --====== jump_count
	}
	jumping_frg.leg_count
}
```

If you can't comprehend the last expression in a function being the returned value, you can use the `return` keyword.
```rs
return jumping_frg.leg_count
```

"Oh but how does it know when one statement ends and one begins! I don't have my fucking semicolons!". This system works on hopes and dreams and is flawless.
```rs
int your_iq = 5 int(int int) recalc = (value_a value_b) { return value_a - value_b } your_iq -= recalc(,,112 your_iq)
```

But you can add semicolons if you want i guess.
```rs
int your_iq = 5;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
```

> [!NOTE]
> semicolons can be used to guide treesitter since its stupid and needs them sometimes :(

## More examples
Take some normal rust code, for example a vector declaration.
```rs
let mut number_list: Vec<i32> = vec![5, 8, 8, 9 - 3]
number_list[2] -= number_list[0]
```
(do everything above)

And done! Ideal frg code!
```rs
vec(int   ) number_list = [,5 8 8 9 - 3]
number_list [[[ 2] ---==== number_list [ 0 ]]]
```
The parser will not trust you after this one.

# Features
## Types
- Integers (`int`)
- Floats (`float`)
- Strings (`str`)
- Booleans (`bool`)
- Vectors (`vec(int)`)
- Maps (`map(str, int)`)
- Functions (`int(str)`)
- Structs (`StructName`)
- References (`&int`)

These type aliases since you'll forget `map` somehow:
- `map`: `obj`, `hashmap`, `dict`, `dictionary`
- `vec`: `array`, `arr`, `list`

## Plagiarized
Some pointer bullshit you can probably pull off (from C):
```rs
// this is one of the tests in this projects
&people[get_index()].pets[0].* += calculate(x.* + y[2] - z.age / 3 * 2) >= threshold * active
```

Functions as values (from Java i think):
```rs
// this is also a test
int(float, str)(map(str, int())) average_java_developer = (get_score_map) { (my_int, my_float) { 5 } }
```

Parentheses (invented by Lisp):
```rs
(5)
```

Semicolons (invented by javascript because i hate javascript and i hate semicolons):
```rs
// scroll up to the part about semicolons i hate talking about them more than i need too
```

STRUCTS!! FROM HIT LANGUAGE RUST!!
```rs
struct Person = {
	int age,
	str name,
}
Person josch = {
	age: 912,
	name: "ojshsc"
}
```

Dereferencing from Zig bc that syntax is nicee
```rs
&int var = &5
void(&int) increment = (to_update) {
	to_update.* += 1
}
increment(var)
```

## If Statements
Keywords? I think? im not sure i blacked out when i was adding this part. this code might ont even run but its in my tests (from python? maybe lua?)
```rs
if thing {
} else if other_thing {
} else {
}
```

wait no keywords are optional
```rs
if if if thing {
} else else else if if other_thing {
} else if if {
}
```

> [!NOTE]
>
> `else` is required due to technical limitations (treesitter).
> Getting rid of `if` is fine though.
> ```
> thing {
> } else other_thing {
> } else {
> }
> ```

## Structs and Maps
also maps and structs share the same syntax so they have to be declared in their declaration
```rs
// THIS IS FINE
Frog frg = { name: "frg", legs: 8, age: 3 }
map(str, int) slur_count = { "fuck": 9857, "shit: 8234", "i dont like rust": 494 }

// THIS WONT WORK
frg = { name: "no", legs: -199999, age: 94787367 }
slur_count = { "error": 9001 }

// I THINK REASSIGNING THEM IS FINE THOUGH
Frog frg = { name: "not frg", legs: -8, age: -3 }
map(str, int,, ) slur_count = { "darn": 9840928341 }
```

You can declare struct literals like its Rust if you need them inline:
```rs
Frog new_frg = mitose(Frog { name: "freg", legs: 1, age: 80 })
```

Also from Zig, builtins:
`@print`: Uses Rust's string interpolotation to `println!`
`@<type>`: Try casting to a certain BUILT IN type

## treesitter is begging for its dear life
Basically anytime you see something that doesn't need to be there you can probably remove it. Or add extra.
```rs
// example: commas in maps
map(float, int) round_table = {
	0.0: 0,,,,,
	0.1: 0,,
	0.2: 0,
	0.3: 0
	0.4: 0
	0.5: 1,
	0.6: 1,,
	0.7: 1,,,,,
	0.8: 1,,,,,,
	0.9: 1,,,
	1.0: 1,
}
```

## Voiding Values
When you call a function, its required to use all of it's values. If you don't need to use them, assign them to the special `_` variable, which discards its value when its set:
```rs
_ = returns_value()	
```

Theres a special keyword for this action: `void`
```rs
void returns_value()
```
Yes, it's the same void as the type.

> [!NOTE]
> The void keyword is optional
> ```rs
>	returns_value()
> ```


# Installation

> [!IMPORTANT]
> Rust is required to use this project.
> You can install Rust through [the official website](https://rust-lang.org/tools/install/).

## Cargo (Recommended)
*You need Rust installed to use this project anyways.*

1. `cargo install frg`
2. Only 29 dependencies, installs quickly and works on any platform
3. Execute with `frg` anywhere

## Linux
1. Go to the [latest release](https://github.com/Shuflduf/frg/releases/latest)
2. Download `frg-linux-[ARCH]`
3. Rename to `frg`
4. Execute with `./frg` while in the directory with the binary

## Linux
1. Go to the [latest release](https://github.com/Shuflduf/frg/releases/latest)
2. Download `frg-macos-[ARCH]`
3. Rename to `frg`
4. Execute with `./frg` while in the directory with the binary

## Windows
1. Go to the [latest release](https://github.com/Shuflduf/frg/releases/latest)
2. Download `frg-windows-[ARCH].exe`
3. Rename to `frg.exe`
4. Execute with `frg.exe` while in the directory with the binary


# Editor Setup

## Helix
1. Add this to your `languages.toml`:
```toml
[[grammar]]
name = "frg"
source = { git = "https://github.com/Shuflduf/frg", rev = "*", subpath = "tree-sitter-frg" }

[[language]]
name = "frg"
scope = "source.frg"
file-types = ["frg"]
comment-tokens = ["//"]
indent = { tab-width = 4, unit = "    " }
```
2. Run `helix --grammar fetch` and `helix --grammar build`

## VSCode
1. Download the extension from [OpenVSX](https://open-vsx.org/extension/Shuflduf/frg)
   
	<img height="100" src="https://github.com/user-attachments/assets/b68c7ce1-140c-4bc0-98b2-e8edf2485b27" />
2. Go the extensions panel in the sidebar (`Ctrl + Shift + X`)
3. Open the three-dot menu and select `Install from VSIX`

   <img height="200" src="https://github.com/user-attachments/assets/5d517ff2-40b2-4cc3-bbda-c6249fbf5d33" />
4. Select the downloaded extension.



# Usage
Invoke the binary with its name, usually `frg`.

```
frg Transpiler and Runner (v2.0.0)

Usage: frg [FILE]

Options:
  -h, --help     Print this help menu.
  -v, --verbose  Show input code and intermediary treesitter, AST, and Rust.
  -n, --no-exec  Don't run generated Rust. Usually used with -v.
  -e, --example  Run an example. No args provided will list the examples.

Examples:
  frg my_code.frg        Execute a frg file.
  frg --example          List examples
  frg -e fibonacci       Run the fibonacci example
  frg -v -n my_code.frg  Show the process of a frg file being turned into Rust.
```

To write your own frg code, use your [editor](#editor-setup) of choice, and add anything you want to a text file.
Having the file extension be `.frg` is not required, but is recommended.

If something isn't working as expected, use the `-v` flag to inspect the generated Rust to see if everything works.

## Examples
- `fibonacci`: Calculate a target fibonacci number
- `factorial`: Calculate a target factorial number
- `primes`: Lists numbers 1 - 50 and tells you if they're prime
- `guessing`: Higher or lower guessing game, this is the BIG one you SHOULD TEST


# Style Guide
fuck around and find out
