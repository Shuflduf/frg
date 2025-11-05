int add_two = (int second_target) {
    return second_target + 2
}

int add_four = (int first_target) {
    return add_two(first_target)
}

int result = add_four(6)
