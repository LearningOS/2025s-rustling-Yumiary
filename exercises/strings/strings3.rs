// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let array_input = input.as_bytes();
    let mut head = 0;
    let mut tail = array_input.len();
    let n: usize = array_input.len();
    for i in 0..n {
        if array_input[i] == b' ' {
            head += 1;
        } else {
            
            break;
        }
    }
    for i in 0..n {
        println!("{}", i);
        if array_input[n - 1 - i] == b' ' {
            tail -= 1;
        } else {
            //println!("there");
            break;
        }
    }
    if head < tail {

        //println!("here");
        input[head..tail].to_string()
    } else {
        input.to_string()
    }
}


fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut string = input.to_string();
    string.push_str(" world!");
    string
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let ori = input.to_string();
    let ans = ori.replace("cars", "balloons");
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
