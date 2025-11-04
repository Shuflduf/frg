int fibonacci = (int index) {
    if index <= 0 {
        return 0
    } elif index == 1 {
        return 1
    } else {
        return fibonacci(index - 1) + fibonacci(index - 2)
    }
}

int result = fibonacci(8)
