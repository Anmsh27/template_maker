use colored::*;
use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Options {
    PygameBoilerplate,
    PygameObjectOriented,
}

impl Options {
    pub fn print_options() {
        for (x, y) in Options::iter().enumerate() {
            println!("    {}", format!("[{:?}] {:?}", x, y));
        }
    }

    pub fn generate(choice: &Options) -> String {
        use Options::*;

        let mut code = r#""#.to_string();

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
                "#.to_owned(); 
            }

            &_ => {
                println!("{}", "Error: Not an option".red());
                panic!("");
            }
        }

        code
    }

    pub fn make_file(option: &Options, code: &String) {
        let mut filename = String::new();

        match option {
            Options::PygameBoilerplate => filename = "main.py".to_string(),
            Options::PygameObjectOriented => filename = "main.py".to_string(),
            &_ => {
                println!("{}", "Error: Option not found".red());
                panic!();
            }
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
