// fonttools.rs

use std::{process::Command, str};
use regex::Regex;

pub fn get_unicode(fname: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("otfinfo -u {fname}"))
        .output()
        .expect("Failed to execute");
    let stdout = str::from_utf8(&output.stdout).unwrap();

    let re = Regex::new(r"(?x) 
        uni([[:alnum:]]+)   # Unicode
        \s                  # White space
        ([[:alnum:]]+)      # Glyph number
        \s                  # White space
        ([[:alnum:]-]+)     # Name
    ").unwrap();

    for cap in re.captures_iter(stdout) {
        let unicode_value = std::char::from_u32(
            u32::from_str_radix(&cap[1], 16)
                .unwrap()
        ).unwrap();

        println!("{} - {}", &cap[3], unicode_value);
    }
}