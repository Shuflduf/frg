bool(int) is_prime = (index) {
    (2..index).all((i) {
        if index % i == 0 {
            return false
        }
        return true
    })
};

(2..=50).for_each((i) {
    @println("{i}: {}", is_prime(i))
})
