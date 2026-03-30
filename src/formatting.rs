use crate::utils::fatal;
use lazy_regex::regex_replace_all;

pub struct Variables {
    pub path: String,
    pub line: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
}

#[derive(Default)]
pub struct LongestValueSizes { 
    pub path: usize,
    pub line: usize,
    pub status: usize,
    pub description: usize,
}

pub fn format_line(string: &str, vars: Variables, lvs: &LongestValueSizes) -> String {
    let string = regex_replace_all!(
        r"\((.*?)\{([a-z]+)(?::(.)(.)?)?\}(.*?)\)",
        string,
        |_whole, left, var_name, align, align_char, right| {
            replacer(&vars, lvs, left, var_name, align, align_char, right)
        }
    );
    let string = regex_replace_all!(
        r"\{([a-z]+)(?::(.)(.)?)?\}",
        string.as_ref(),
        |_whole, var_name, align, align_char: &str| {
            replacer(&vars, lvs, "", var_name, align, align_char, "")
        }
    );
    string.into_owned()
}

fn replacer(vars: &Variables, lvs: &LongestValueSizes, left: &str, var_name: &str, align: &str, align_char: &str, right: &str) -> String {
            if let (Some(value), min_size) = value_and_min_size(vars, lvs, var_name) {
                let align_char = if align_char.is_empty() { " " } else { align_char };
                let value = match align {
                    "" => value,
                    "<" => align_left(value.as_str(), min_size, align_char),
                    ">" => align_right(value.as_str(), min_size, align_char),
                    "-" => align_center(value.as_str(), min_size, align_char),
                    _ => fatal(format!("Invalid formatting `{align}` (valid ones are: `<`, `>`, `-`)"))
                };
                format!("{left}{value}{right}")
            } else {
                String::new()
            }
        }

fn value_and_min_size(vars: &Variables, lvs: &LongestValueSizes, name: &str) -> (Option<String>, usize) {
    match name {
        "path" => (Some(vars.path.clone()), lvs.path),
        "line" => (vars.line.clone(), lvs.line),
        "status" => (vars.status.clone(), lvs.status),
        "description" => (vars.description.clone(), lvs.description),
        _ => {
            // println!("(Warning: unknown variable in format string {name:?}, ignored)");
            (None, 0)
        }
    }
}

fn align_left(string: &str, min_size: usize, align_char: &str) -> String {
    let mut output = String::with_capacity(min_size);
    output.push_str(string);
    if min_size > string.len() {
        let diff = min_size - string.len();
        output.push_str(align_char.repeat(diff).as_str());
    }
    output
}

fn align_right(string: &str, min_size: usize, align_char: &str) -> String {
    let mut output = String::with_capacity(min_size);
    if min_size > string.len() {
        let diff = min_size - string.len();
        output.push_str(align_char.repeat(diff).as_str());
    }
    output.push_str(string);
    output
}

fn align_center(string: &str, min_size: usize, align_char: &str) -> String {
    let mut output = String::with_capacity(min_size);
    if min_size > string.len() {
        let diff = (min_size - string.len()) as f64 / 2.0;
        output.push_str(&align_char.repeat(diff.floor() as usize));
        output.push_str(string);
        output.push_str(&align_char.repeat(diff.ceil() as usize));
    } else {
        output.push_str(string);
    }
    output
}
