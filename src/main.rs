use colored::*;
use std::io::*;
use template_maker::*;

fn main() {
    println!("{}", "Template Maker!\n".bold());

    let mut template = String::new();

    println!("Templates: ");
    Options::print_options();
    println!("\n");

    println!("Option: ");
    stdin().read_line(&mut template).unwrap_or_else(|error| {
        println!("{}", format!("Error: {}", error).red());
        0 as usize
    });

    match template.trim() {
        "0" => {
            let choice = Options::PygameBoilerplate;
            let code = Options::generate(&choice);
            println!("{}", format!("{}", &code).on_white().bright_blue());
            Options::make_file(&choice, &code);
        }
        &_ => todo!(),
    }
}
