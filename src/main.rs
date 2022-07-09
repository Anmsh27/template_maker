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
        "1" => {
            let choice = Options::PygameObjectOriented;
            let code = Options::generate(&choice);
            println!("{}", format!("{}", &code).on_white().bright_blue());
            Options::make_file(&choice, &code);
        }
        "2" => {
            let choice = Options::Java;
            let code = Options::generate(&choice);
            println!("{}", format!("{}", &code).on_white().bright_blue());
            Options::make_file(&choice, &code);
        },
        "3" => {
            let choice = Options::Rust;
            let code = Options::generate(&choice);
            println!("{}", format!("{}", &code).on_white().bright_blue());
            Options::make_file(&choice, &code);
        },
        "4" => {
            let choice = Options::CSharp;
            let code = Options::generate(&choice);
            println!("{}", format!("{}", &code).on_white().bright_blue());
            Options::make_file(&choice, &code);
        },
        "5" => {
            let choice = Options::C;
            let code = Options::generate(&choice);
            println!("{}", format!("{}", &code).on_white().bright_blue());
            Options::make_file(&choice, &code);
        },
        "6" => {
            let choice = Options::Cpp;
            let code = Options::generate(&choice);
            println!("{}", format!("{}", &code).on_white().bright_blue());
            Options::make_file(&choice, &code);
        }
        &_ => {
            println!("{}", "Error: Not an option".red());
            panic!("");
        },
    }
}
