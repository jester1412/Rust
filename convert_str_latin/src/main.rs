// apple => apple-hay | start with vowel
// first => irst-fay | other case

fn main() {
    let input_str = String::from("first");
    let convert_str = convert_str_lation(input_str);
    println!("{}", convert_str);
}

fn convert_str_lation(str_input: String) -> String {
    let mut new_str = str_input.to_owned();
    let first_char = str_input.chars().nth(0).unwrap();
    // TODO: find out how to write multiple condition better ( regex or match ??)
    if first_char == 'a'
        || first_char == 'e'
        || first_char == 'i'
        || first_char == 'o'
        || first_char == 'u'
    {
        let add_str = "-hay".to_owned();
        new_str.push_str(&add_str);
    } else {
        let mut tmp_str = String::new();
        let str_len = str_input.len();
        // TODO: change from for loop to cut the 1st char from str_input and add to tmp_str
        for i in 1..str_len {
            let nth_char = str_input.chars().nth(i).unwrap();
            tmp_str.push(nth_char);
        }
        tmp_str.push('-');
        let new_first_char = first_char.to_owned();
        tmp_str.push(new_first_char);
        tmp_str.push_str("ay");
        new_str = tmp_str.to_owned();
    }
    return new_str;
}
