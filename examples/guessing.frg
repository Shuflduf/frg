int target = @rand(1, 100)
int current = 0

@println("The goal is to guess a random number between 1 and 100. You get hints depending on whether your guess is higher or lower than the target number.")
while current != target {
    current = @int(@input("What's your guess? "))
    if current < target {
        @println("{current} is too low!")
    } else if current > target {
        @println("{current} is too high!")
    } else {
        @println("Correct!")
    }
}
