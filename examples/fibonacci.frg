int(int) fibonacci = (index) {
    int a = 0
    int b = 1
    if index == 1 {
        return 1
    } else {
        (2..index).for_each(() {
            int c = a + b
            a = b
            b = c
        })
    }
    b
}

int result = fibonacci(10)
@print("10th fibonacci is {result}")
