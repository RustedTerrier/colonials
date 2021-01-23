extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

//Main function
fn main() {
    let mut character = 'm';
    let mut ypos = 0;
    let mut xpos = 0;
    let mut menu: bool = false;
    //Initialize the vector with the standard colonies
    let mut colonys = init();
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Welcome to Colonials!\n\rQ to quit. Escape to get help and to exit.{}",
        // Clear the screen.
        termion::clear::All,
        // Goto (1,1).
        termion::cursor::Goto(1, 1),
        // Hide the cursor.
        termion::cursor::Hide
    )
    .unwrap();
    // Flush stdout
    stdout.flush().unwrap();

    for c in stdin.keys() {
        // Clear the screen
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
        //Check keys
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Left => xpos = xpos - 1,
            Key::Right => xpos = xpos + 1,
            Key::Up => ypos = ypos + 1,
            Key::Down => ypos = ypos - 1,
            Key::Esc => {
                if menu == false {
                    menu = true
                } else {
                    menu = false
                }
            }
            _ => character = '_',
        }
        //Read the vector containing the colonies
        render(menu, &colonys);
        // Flush stdout
        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit and clear the screen.
    write!(stdout, "{}{}", termion::cursor::Show, termion::clear::All).unwrap();
}

fn init() -> Vec<Colony> {
    let mut colonies = Vec::new();
    let mut n = 0;
    while n < 4 {
        let colony = Colony {
            selected: 0,
            owner: 0,
        };
        colonies.push(colony);
        n += 1;
    }
    //return a vector containing the colonies
    colonies
}

fn render(display: bool, colony: &Vec<Colony>) {
    //If it's in a menu:
    if display {
        let options = Menu { selected: 1 };
        if options.selected == 1 {
            println!("\n\r\n\r\n\r    ########\n\r    # Menu #\n\r    ########\n\r    # {red}Quit{reset} #\n\r    # Play #\n\r    # Rules#\n\r    ########",
            red   = color::Fg(color::Red),
            reset = color::Fg(color::Reset));
        } else {
            if options.selected == 2 {
                println!("\n\r\n\r\n\r    ########\n\r    # Menu #\n\r    ########\n\r    # Quit #\n\r    # {red}Play{reset} #\n\r    # Rules#\n\r    ########",
                red   = color::Fg(color::Red),
                reset = color::Fg(color::Reset));
            } else {
                println!("\n\r\n\r\n\r    ########\n\r    # Menu #\n\r    ########\n\r    # Quit #\n\r    # Play #\n\r    # {red}Rules{reset}#\n\r    ########",
                red   = color::Fg(color::Red),
              reset = color::Fg(color::Reset));
            }
        }
    } else {
        //Print the top colonies
        print!(
            " {one}/##\\ {two}/##\\\n\r{reset}             ###\n\r",
            one = color::Fg(color::Red),
            two = color::Fg(color::Blue),
            reset = color::Fg(color::Reset)
        );
        //Initialize a loop that reads from the vector and renders it.
        let mut i = 0;
        while i < 4 {
            let colony_s = &colony[i];

            if colony_s.selected == 0 {
                if colony_s.owner == 0 {
                    println!("<#########>  # #\n\r             ###\r");
                } else {
                    if colony_s.owner == 1 {
                        println!(
                            "{red}<#########>{reset}  #{red}1{reset}#\n\r             ###\r",
                            red = color::Fg(color::Red),
                            reset = color::Fg(color::Reset)
                        );
                    } else {
                        println!(
                            "{blue}<#########>{reset}  #{blue}2{reset}#\n\r             ###\r",
                            blue = color::Fg(color::Blue),
                            reset = color::Fg(color::Reset)
                        );
                    };
                };
            } else {
                if colony_s.selected == 1 {
                    if colony_s.owner == 0 {
                        println!("<#########>  # #\n\r             ###\r");
                    } else {
                        if colony_s.owner == 1 {
                            println!(
                                "<#########>  #{red}1{reset}#\n\r             ###\r",
                                red = color::Fg(color::Red),
                                reset = color::Fg(color::Reset)
                            );
                        } else {
                            println!(
                                "<#########>  #{blue}2{reset}#\n\r             ###\r",
                                blue = color::Fg(color::Blue),
                                reset = color::Fg(color::Reset)
                            );
                        };
                    };
                } else {
                    if colony_s.owner == 0 {
                        println!(
                            "{red}<#########>{reset}  # #\n\r             ###\r",
                            red = color::Fg(color::Red),
                            reset = color::Fg(color::Reset)
                        );
                    } else {
                        if colony_s.owner == 1 {
                            println!(
                                "{red}<#########>{reset}  #{red}1{reset}#\n\r             ###\r",
                                red = color::Fg(color::Red),
                                reset = color::Fg(color::Reset)
                            );
                        } else {
                            println!(
                                "{red}<#########>{reset}  #{blue}2{reset}#\n\r             ###\r",
                                red = color::Fg(color::Red),
                                blue = color::Fg(color::Blue),
                                reset = color::Fg(color::Reset)
                            );
                        };
                    };
                };
            };
            i += 1;
        }
    };
}

struct Menu {
    selected: u8,
}

struct KeysPressedLast {
    right: bool,
    left: bool,
    up: bool,
    down: bool,
    space: bool,
}

struct Colony {
    selected: u8,
    owner: u8,
}
