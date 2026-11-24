mod other_file = @import("modules.frg") // a COPY of the module. ex. increment only increases the counter of this copy of this module

int|| get_x = other_file.increment
int x = other_file.increment()
int y = result
int z = other_file.maths.add(7, 11)

# use other_file.maths.add
// int result = add(7, 11)

// eventually:
// mod std = @import("std")
// use std.debug.print
// print(4)

