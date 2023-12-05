fn main() {
    let result: u32 = include_str!("../resources/input_1.prod")
        .split("\n")
        .map(|slice| get_number_from_string(slice))
        .collect::<Vec<u32>>()
        .iter()
        .sum();

    println!("Result is {}", result);
}

fn get_number_from_string(slice: &str) -> u32 {
    //println!("Input {}", slice);

    // Part Two
    let data = slice
        .to_string()
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");

    let radix = 10;

    let numbers: Vec<char> = data
        .chars()
        .filter_map(|c| c.to_digit(radix))
        .map(|digit| char::from_digit(digit, radix).unwrap())
        .collect();

    let first_element = numbers.first().unwrap().to_string();
    let mut second_element = first_element.clone();

    if numbers.len() > 1 {
        second_element = numbers.last().unwrap().to_string();
    }

    let result = format!("{}{}", first_element, second_element);
    let value = result.parse::<u32>().unwrap();

    //println!("Value: {}", value);

    return value;
}
