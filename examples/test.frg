int(int) fibonacci = (index) {
    int a = 1
    int b = 1
    if index == 1 {
        return 1
    } else {
        (2..index).for_each((_) {
            int c = a + b
            a = b
            b = c
        })
    }
    b
}

int target_fib = @int(@input("What fib number? "))
int result = fibonacci(target_fib)
@print("{target_fib}th fibonacci is {result}")

