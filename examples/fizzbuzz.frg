void(int) fizzbuzz = (index) {
    str output = ""
    if index % 3 == 0 {
        output = (output.to_owned() + "Fizz").leak()
    }
    if index % 5 == 0 {
        output = (output.to_owned() + "Buzz").leak()
    }
    if output == "" {
        output = (output.to_owned() + @str(index)).leak()
    }
    @println("{output}")
}

(1..=100).for_each((i) {
    fizzbuzz(i)
})
