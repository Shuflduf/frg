int add_two = (int target) {
    return target + 2
}

int add_four = (int target) {
    return add_two(target)
}

int result = add_four(6)
