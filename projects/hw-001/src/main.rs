use std::vec::Vec;

fn print_result(result: &Vec<String>) {
    for r in result {
        println!("{}", r)
    }
}

fn functional_approach(count: usize) -> Vec<String> {
    (1..=count)
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            (_, _) => format!("{}", i),
        })
        .collect()
}

fn imperative_approach(count: usize) -> Vec<String> {
    let mut result = Vec::with_capacity(count);
    for i in 1..=count {
        let s = match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            (_, _) => format!("{}", i),
        };
        result.push(s);
    }

    result
}

fn main() {
    let count: usize = 100;

    let result_i = imperative_approach(count);
    let result_f = functional_approach(count);

    assert_eq!(result_i, result_f);

    print_result(&result_i);
}
