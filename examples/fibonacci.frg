int fibonacci = (int index) {
    if index <= 1 {
        return index
    } else {
        return fibonacci(index - 1) + fibonacci(index - 2)
    }
}

int three = fibonacci(3)
int four = fibonacci(4)
int five = fibonacci(5)
int six = fibonacci(6)
int seven = fibonacci(7)
int eight = fibonacci(8)
