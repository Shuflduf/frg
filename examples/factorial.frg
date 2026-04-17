int(int) factorial = (index) {
    int a = 1;

    (1..=index).for_each((i) {
        a = a * i
    })
    a
}

int target_fac = @int(@input("What factorial? "))
int result = factorial(target_fac)
@print("{target_fac}th factorial is {result}")

