int recursive = (int start) {
    if start <= 0 {
        return 0
    }
    return start + recursive(start - 1)
}

int result = recursive(3)
