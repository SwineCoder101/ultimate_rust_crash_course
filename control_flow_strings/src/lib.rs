pub fn sum() {
    let mut sum = 0;
    // 2. Use a "for loop" to iterate through integers from 7 to 23 *inclusive* using a range
    // and add them all together (increment the `sum` variable).  Hint: You should get 255
    // Run it with `cargo run sum`

    for num in 7..=23 {
        sum += num;
    }
    println!("The sum is {}", sum);
}

pub fn double() {
    let mut count = 0;
    let mut x = 1;
    // 3. Use a "while loop" to count how many times you can double the value of `x` (multiply `x`
    // by 2) until `x` is larger than 500.  Increment `count` each time through the loop. Run it
    // with `cargo run double`  Hint: The answer is 9 times.

    while x <= 500 {
        x *= 2;
        count += 1;
    }


    println!("You can double x {} times until x is larger than 500", count);
}

pub fn count(arg: String) {
    // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
    // You will need to count your loops, somehow.  Run it with `cargo run bananas`
    //
    // print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.

    let mut count  = 1;
    loop {
        print!("{} {} ", count, arg);

        if count == 8 {
            break;
        }

        count += 1;
    }

    println!(); // This will output just a newline at the end for cleanliness.
}