use std::ffi::OsStr;

pub fn get_line_col(src: &String, index: usize) -> (usize, usize) {
    let slices = src.split_at(index);
    let prev:&str = slices.0;

    let mut cur_index = 0;

    let mut line = 0;
    let mut index_of_line = 0;

    for ch in prev.chars() {
        match ch {
            '\n' => {line = line + 1; index_of_line = cur_index;},
            _ => ()
        }
        cur_index = cur_index + 1;
    }

    let column = index - index_of_line;

    (line+1, column+1) // Line and column aren't typically 0 based
}

pub fn generate_syntax_error(src : &String, location : (usize, usize)) -> String {
    generate_syntax_error_with_slug(src, location, "")
}

pub fn generate_syntax_error_with_slug<'a>(src: &String, location: (usize, usize), slug: &'a str) -> String {
    let mut out : String = "Syntax Error at [".to_string();
    out.push_str(&(location.0.to_string()));
    out.push_str(", ");
    out.push_str(&(location.1.to_string()));
    out.push_str("] : ");

    let prelength = out.len();

    out.push_str(src.lines().skip(location.0 - 1).next().unwrap());

    // Location.1 is column, which is not 0 based for display but should be for spacing
    let mut pointing : String = String::from_utf8(vec![b' ';prelength + location.1 - 1]).unwrap();
    pointing.push_str("^~~~ ");
    pointing.push_str(slug);
    out.push_str("\n");
    out + &pointing
}

pub fn generate_syntax_error_with_slug_from_index<'a>(src: &String, index: usize, slug: &'a str) -> String {
    generate_syntax_error_with_slug(src, get_line_col(src, index), slug)
}

pub fn generate_syntax_error_from_index(src: &String, index: usize) -> String {
    generate_syntax_error(src, get_line_col(src, index))
}

pub fn osstr_to_string (input: &OsStr) -> Option<String> {
    match input.to_str() {
        None => None,
        Some(val) => Some(val.into())
    }
}
