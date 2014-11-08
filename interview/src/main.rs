
/* ------------------- BEGIN IS PALINDROME ----------------------
 * Given a string, return true if it is a palindrome, false
 * otherwise.
 * -------------------------------------------------------------- */
fn is_palindrome(input: &str) -> bool {
    let reversed_input = input.chars().rev().collect::<String>();
    return input == reversed_input.as_slice();
}

#[test]
fn test_is_palindrome() {
    assert_eq!(true, is_palindrome("anna"));
    assert_eq!(false, is_palindrome("apple"));
    assert_eq!(false, is_palindrome("banana"));
}

/* -------------------- END IS PALINDROME ----------------------- */

/* --------------------- BEGIN FIZZ BUZZ ------------------------
 * Given an input number n, print out "Fizz" if the number is a
 * multiple of 3, "Buzz" if it is a multiple of 5, and "FizzBuzz"
 * if it is a multiple of both 3 and 5. If none of the above
 * apply, print out just the number.
 * -------------------------------------------------------------- */

fn fizz_buzz(n: int) {
    for i in range(1i, n) {
        match i {
            _ if i % 5 == 0 && i % 3 == 0 => { println!("FizzBuzz"); }
            _ if i % 5 == 0 => { println!("Buzz"); }
            _ if i % 3 == 0 => { println!("Fizz"); }
            _ => { println!("{}", i); }
        }
    }
}

#[test]
fn test_fizz_buzz() {
    fizz_buzz(20);
}

 /* --------------------- END FIZZ BUZZ ------------------------ */

#[cfg(not(test))]
fn tests_verbose() {
    // testing fizz_buzz
    println!("\nTesting fizz_buzz:");
    fizz_buzz(20);

    // testing is_palindrome
    println!("\nTesting is_palindrome:");
    let inputs = ["apple", "banana", "anna", "anaconda"];
    for input in inputs.iter() {
        println!("Is `{}` a palindrome? -> {}", input, is_palindrome(*input));
    }
}

// only compile this code if we're not testing
#[cfg(not(test))]
fn main() {
    println!("To run the tests, use the cargo command `cargo test`.");

    tests_verbose();
}
