pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => brackets.push(c),
            ')' => {
                if brackets.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if brackets.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if brackets.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }
    brackets.is_empty()
}

fn main() {
    let test_cases = vec![
        "[]",
        "([{}])",
        "{[()]}",
        "{[}]",
        "((())",
        "[({})]",
        "",
        "no brackets here"
    ];

    for case in test_cases {
        println!("Input: \"{}\" => Balanced: {}", case, brackets_are_balanced(case));
    }
}
