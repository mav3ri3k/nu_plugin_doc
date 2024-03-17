use std::collections::HashMap;
mod util;
mod parser;

trait Clean {
    fn clean(&self) -> Self;
}

impl Clean for String {
    fn clean(&self) -> Self {
        let mut output = String::new();
        for ch in self.chars() {
            if ch == '.' { break; }
            output.push(ch);
        }

        output
    }
}

fn find_categories(categories: &mut HashMap<String, Vec<String>>) {
    let path_categories = String::from("/Users/apurva/projects/nu_doc/commands/categories/");

    for category in util::dir_walk(path_categories) {
        let category = category.file_name().into_string().unwrap()
            .clean();
        categories.insert(category, vec![]);
    }
}

// finds list of commands and add to appropriate category
// Command title and category is parsed through markdown header
fn find_commands(categories: &mut HashMap<String, Vec<String>>) {
    let path_commands = String::from("/Users/apurva/projects/nu_doc/commands/docs/");

    for command in util::dir_walk(path_commands) {
        let content = util::file_content(command.path().into_os_string());
        let (title, category) = parser::tokens(&content).unwrap();
        categories.entry(category)
            .or_insert(Vec::new())
            .push(title);
    }

    for val in categories.values_mut() {
        val.sort();
    }
}

// Finds all the categories for commands
// Then find list of commands and add then to categories
pub fn discover(categories: &mut HashMap<String, Vec<String>>) {
    find_categories(categories);
    find_commands(categories);
}
