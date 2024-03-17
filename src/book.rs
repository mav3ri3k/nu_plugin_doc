use std::collections::HashMap;
use std::fs;

fn sanitize_names(name: &String) -> String {
    let mut output = String::new();
    for ch in name.chars() {
        if ch == ' ' {
            output.push('_');
            continue;
        }
        output.push(ch);
    }
    output
}

pub fn add(categories: &HashMap<String, Vec<String>>) {
    let path_readme = "/Users/apurva/projects/nu_doc/commands/README.md".to_string();
    let path_categories = "/Users/apurva/projects/nu_doc/commands/categories/".to_string();
    let path_commands = "/Users/apurva/projects/nu_doc/commands/docs/".to_string();

    let path_summary = "/Users/apurva/projects/nu_doc/docs/src/SUMMARY.md".to_string();
    let path_src = "/Users/apurva/projects/nu_doc/docs/src/".to_string();

    let mut input = String::from("# Summary\n\n[Introduction](README.md)\n\n");
    fs::copy(path_readme, path_src.clone() + "README.md").unwrap();

    for (category, val) in categories.iter() {
        let category = sanitize_names(&category);

        input.push_str("- [");
        input.push_str(&category);
        input.push_str("](");
        input.push_str(&category);
        input.push_str(".md)\n");
        let full_path_catg_s = path_categories.clone() + &category + ".md";
        let full_path_catg_d = path_src.clone() + &category + ".md"; 
        fs::copy(full_path_catg_s, full_path_catg_d).unwrap();

        for command in val {
            let command = sanitize_names(&command);
            input.push_str("    - [");
            input.push_str(&command);
            input.push_str("](");
            input.push_str(&command);
            input.push_str(".md)\n");
            let full_path_cmd_s = path_commands.clone() + &command + ".md";
            let full_path_cmd_d = path_src.clone() + &command + ".md";
            fs::copy(full_path_cmd_s, full_path_cmd_d).unwrap();
        }
        input.push_str("\n");

        fs::write(path_summary.clone(), &input).unwrap();
    }
}
