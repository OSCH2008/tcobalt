const RAW: &str = include_str!("strings.txt");

pub fn get_help<'a>() -> String {
    let mut string = String::new();
    for line in RAW.lines().into_iter() {
        if line.chars().collect::<Vec<char>>().first() == Some(&'[') { break }
        string.push_str(line);
        string.push('\n');
    }
    remove_trailing_whitespace(string)
}

fn remove_trailing_whitespace(string: impl ToString) -> String {
    let mut string = string.to_string();
    let chars = string.chars().collect::<Vec<char>>();
    let mut i = string.len()-1;
    while chars.get(i) == Some(&'\n') {
        string.pop();
        i -= 1;
    }
    string
}
