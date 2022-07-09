use colored::*;
use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


#[derive(Debug, EnumIter)]
pub enum Options {  
    PygameBoilerplate,
    PygameObjectOriented,
    Java,
    Rust,
    CSharp,
    C,
    Cpp,
    Flask,
}

impl Options {
    pub fn print_options() {
        for (x, y) in Options::iter().enumerate() {
            println!("    {}", format!("[{:?}] {:?}", x, y));
        }
    }

    pub fn generate_flask() {
        let dirbuilder = fs::DirBuilder::new();

        dirbuilder.create("Flask_Project")
            .unwrap_or_else(|e| {
                println!("{}", format!("Error: {}", e).red());
                panic!("");
            });
        
        dirbuilder.create("Flask_Project/templates")
            .unwrap_or_else(|e| {
                println!("{}", format!("Error: {}", e).red());
                panic!("");
            });

        fs::File::create("Flask_Project/main.py")
            .unwrap_or_else(|e| {
                println!("{}", format!("Error: {}", e).red());
                panic!("");
            });
        
        fs::File::create("Flask_Project/run.py")
            .unwrap_or_else(|e| {
                println!("{}", format!("Error: {}", e).red());
                panic!("");
            });
        
        fs::File::create("Flask_Project/templates/index.html")
            .unwrap_or_else(|e| {
                println!("{}", format!("Error: {}", e).red());
                panic!("");
            });

        let main_content = r#"
from flask import Flask, render_template

app = Flask(__name__)

@app.route('/')
def home():
    return render_template('index.html')
        "#;

        let run_content = r#"
from main import app

app.run(debug=True)
        "#;

        let html_content = r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <meta http-equiv="X-UA-Compatible" content="ie=edge">
        <title>My Website</title>
    </head>
    <body>
        <h1>Hello, World!</h1>
    </body>
</html>
        "#;
        
        fs::write("Flask_Project/main.py", main_content)
            .unwrap_or_else(|e| {
                println!("{}", format!("Error: {}", e).red());
                panic!("");
            });
        
        fs::write("Flask_Project/run.py", run_content)
            .unwrap_or_else(|e| {
                println!("{}", format!("Error: {}", e).red());
                panic!("");
            });

        fs::write("Flask_Project/templates/index.html", html_content)
            .unwrap_or_else(|e| {
                println!("{}", format!("Error: {}", e).red());
                panic!("");
            });
    }

    pub fn generate(choice: &Options) -> String {
        use Options::*;

        let code;

        match &choice {
            PygameBoilerplate => {
                code = r#"
import pygame
from sys import exit
pygame.init()

screen = pygame.display.set_mode((500,500))
pygame.display.set_caption("Game")
clock = pygame.time.Clock()

while True:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            exit()
            pygame.quit()
    
    screen.fill('black')

    pygame.display.flip()
    clock.tick(60)
                "#
                .to_owned(); },
            
            Options::PygameObjectOriented => {
                code = r#"
import pygame
from sys import exit

class Game:
    def __init__(self):
        pygame.init()

        self.screen = pygame.display.set_mode((500,500))
        pygame.display.set_caption("Game")
        self.clock = pygame.time.Clock()

    def run(self):
        while True:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()
                    exit()

            self.screen.fill('black')

            pygame.display.update()
            self.clock.tick(60)

if __name__ == '__main__':
    game = Game()
    game.run()
                "#.to_owned(); },

            Options::Java => {
                code = r#"
public class Main {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
                "#.to_owned(); },

            Options::Rust => {
                code = r#"
fn main() {
    println!("Hello, World!");
}
                "#.to_owned(); },
            
            Options::CSharp => {
                code = r#"
using System;
using System.Collections.Generic;

namespace Program {
    class Program {
        static void Main(String[] args) {
            Console.WriteLine("Hello, World!");
        }
    }
}
                "#.to_owned(); },
            
            Options::C => {
                code = r#"
#include <stdio.h>
#include <stdlib.h>

int main() {
    printf("Hello, World!\n");

    return 0;
}
                "#.to_owned(); },
            
            Options::Cpp => {
                code = r#"
#include <iostream>

using namespace std;

int main() {
    cout << "Hello, World!" << endl;

    return 0;
}
                "#.to_owned(); },

        Options::Flask => { 
            Options::generate_flask();
            return String::new();
        }

        }
        code
    }

    pub fn make_file(option: &Options, code: &String) {
        let filename: String;

        match option {
            Options::PygameBoilerplate => filename = "main.py".to_string(),
            Options::PygameObjectOriented => filename = "main.py".to_string(),
            Options::Java => filename = "Main.java".to_string(),
            Options::Rust => filename = "main.rs".to_string(),
            Options::CSharp => filename = "Main.cs".to_string(),
            Options::C => filename = "main.c".to_string(),
            Options::Cpp => filename = "main.cpp".to_string(),
            Options::Flask => filename = "main.py".to_string()
        }

        fs::File::create(&filename).unwrap_or_else(|e| {
            println!("{}", format!("Error: {}", e).red());
            panic!("");
        });

        fs::write(&filename, code).unwrap_or_else(|e| {
            println!("{}", format!("Error: {}", e).red());
        });
    }
}
