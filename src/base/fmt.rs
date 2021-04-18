use crate::base::parser::check_file;
use crate::err;
use colored::Colorize;
use std::{
    fs::{read_to_string, write},
    path::Path,
};

pub fn formater(path: &Path) {
    if check_file(path) {
        let content_ = read_to_string(path);
        let content = match content_ {
            Ok(content) => content,
            Err(_) => {
                err!(&format!("parser couldn't read {}", path.display()));
            }
        };

        let split_content = content.split('\n');
        let mut final_data = "".to_owned();
        for i in split_content {
            if !i.trim().is_empty() {
                final_data.push_str(&format!(
                    "{}\n",
                    i.split_whitespace().collect::<Vec<_>>().join(" ")
                ));
            }
        }
        final_data.pop();

        write(path, &final_data).unwrap();

        println!("{}", content.red());
        println!("\n-> formated as ->\n");
        println!("{}", final_data);
    }
}
