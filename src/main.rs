use std::arch::x86_64::_mm256_load_pd;
use std::hash::Hash;
use std::{io, ptr};
use std::io::Read;
use std::ops::ControlFlow::Break;
use std::process;
use std::ptr::null;
use ansi_term::*;
use std::{thread, time};

fn main() {

    print_introduction();

    println!("Do you want to play? [Y/N]: ");

    loop {
        let mut decision: String = String::new();

        io::stdin()
            .read_line(&mut decision)
            .expect("Failed to read line");

        decision = decision.trim().to_string();

        if decision == "N" || decision == "n" {
            println!("Oh, that's sad :( Now I will kill my self");
            end_the_game()
        }

        if decision == "Y" || decision == "y" {
            println!("Thank you for playing this awesome game\n");
            ask_for_word();
            break
        }

        println!("{}", decision);

        println!("Please type Y/y or N/n: ")
    }
}

fn print_introduction() {

    println!(" _
| |
| |__   __ _ _ __   __ _ _ __ ___   __ _ _ __
| '_ \\ / _` | '_ \\ / _` | '_ ` _ \\ / _` | '_ \\
| | | | (_| | | | | (_| | | | | | | (_| | | | |
|_| |_|\\__,_|_| |_|\\__, |_| |_| |_|\\__,_|_| |_|
                    __/ |
                   |___/                       \n");

    println!("Welcome to a Hangman-Terminal game :)\n");
    println!("{} \n", Style::new().bold().paint("How to play?"));
    println!("One player thinks of a word or phrase; the others try to guess what it is one letter at a time. The player draws a number of dashes equivalent to the number of letters in the word. If a guessing player suggests a letter that occurs in the word, the other player fills in the blanks with that letter in the right places. If the word does not contain the suggested letter, the other player draws one element of a hangman’s gallows. As the game progresses, a segment of the gallows and of a victim is added for every suggested letter not in the word. The number of incorrect guesses before the game ends is up to the players, but completing a character in a noose provides a minimum of six wrong answers until the game ends. The first player to guess the correct answer thinks of the word for the next game.")
}

fn ask_for_word() {

    println!("Gamemaster, please make sure you are the only one who can read the following lines.");

    loop {
        println!("Please type in the word you want to play with: ");

        let mut word: String = String::new();

        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        word = word.trim().to_string();

        println!("Are you sure you want to use '{}' as the word [Y/N]: ", word);

        let mut decision: String = String::new();

        io::stdin()
            .read_line(&mut decision)
            .expect("Failed to read line");

        decision = decision.trim().to_string();

        if decision == "Y" || decision == "y" {
            println!("Ok, then let's start the game :) \n");
            play_game(word);
            break;
        }
    }
}

fn play_game(word: String) {

    let mut word_to_fill = get_word_to_fill(&word);
    let mut wrong_count: u16 = 0;

    println!("To all other players, this is the empty word you have to guess the letters: \n");

    loop {
        println!("{} \n", word_to_fill);

        if !word_to_fill.contains('_') { break }

        println!("Player, please guess a letter: \n");

        let mut letter: String = String::new();

        io::stdin()
            .read_line(&mut letter)
            .expect("Failed to read line");

        letter = letter.trim().to_string();

        if word.contains(&letter) {

            let indices = get_indices_of_letter(&word, letter.chars().nth(0).unwrap());

            word_to_fill = get_new_word_to_fill(word_to_fill, letter.chars().nth(0).unwrap(), &indices);

            check_if_word_is_found(&word, &word_to_fill);

            continue
        }

        wrong_count += 1;
        println!("Sorry, but the letter '{}' isn't part of the word :(", letter);
        print_hangman(&wrong_count, &word);
    }
}

fn get_word_to_fill(word: &String) -> String {

    let length = word.len();

    let mut word_to_fill: Vec<char> = Vec::new();

    for i in 0..length {
        word_to_fill.push('_')
    }

    word_to_fill.into_iter().collect()
}

fn check_if_word_is_found(word: &String, word_to_fill: &String) {

    if word.eq_ignore_ascii_case(word_to_fill) {
        println!("Wohoo, the player found the word :)");
        println!("The player won. \n");
        println!("I will now end the game.");
        thread::sleep(time::Duration::from_secs(5));
        end_the_game();
        return;
    }

    println!("Congrats, you have found one letter");
}

fn get_indices_of_letter(word: &String, letter: char) -> Vec<usize> {

    let mut word_as_chars: Vec<char> = word.chars().collect();
    let mut counter = 0;

    let mut indices_of_letter: Vec<usize> = Vec::new();

    for char in word_as_chars {

        if char.eq_ignore_ascii_case(&letter) {
            indices_of_letter.push(counter);
        }

        counter += 1;
    }

    indices_of_letter
}

fn get_new_word_to_fill(word_to_fill: String, letter: char, indices: &Vec<usize>) -> String {

    let mut new_word_to_fill1: String = String::new();

    for i in indices {

        let mut new_word_to_fill: Vec<char> = if new_word_to_fill1.is_empty() { word_to_fill.chars().collect() } else { new_word_to_fill1.chars().collect() } ;

        let got = std::mem::replace(&mut new_word_to_fill[*i], letter);

        let mut word_to_fill_as_string: String = new_word_to_fill.into_iter().collect();

        new_word_to_fill1 = word_to_fill_as_string;
    }

    new_word_to_fill1
}

fn print_hangman(wrong_counter: &u16, word: &String) {

    match wrong_counter {
        1 => {
            println!("######################
#                    #
#                    #
#                    #
#                    #
#                    #
#                    #
#                    #
#  _________         #
#                    #
######################\n");
        },
        2 => {
            println!("######################
#                    #
#      |             #
#      |             #
#      |             #
#      |             #
#      |             #
#      |             #
#  ____|____         #
#                    #
######################\n");
        },
        3 => {
            println!("######################
#      _________     #
#      |             #
#      |             #
#      |             #
#      |             #
#      |             #
#      |             #
#  ____|____         #
#                    #
######################\n");
        },
        4 => {
            println!("######################
#      _________     #
#      |/            #
#      |             #
#      |             #
#      |             #
#      |             #
#      |             #
#  ____|____         #
#                    #
######################\n");
        },
        5 => {
            println!("######################
#      _________     #
#      |/      |     #
#      |             #
#      |             #
#      |             #
#      |             #
#      |             #
#  ____|____         #
#                    #
######################\n");
        },
        6 => {
            println!("######################
#      _________     #
#      |/      |     #
#      |     (°_°)   #
#      |             #
#      |             #
#      |             #
#      |             #
#  ____|____         #
#                    #
######################\n");
        },
        7 => {
            println!("######################
#      _________     #
#      |/      |     #
#      |     (°_°)   #
#      |       |     #
#      |       |     #
#      |             #
#      |             #
#  ____|____         #
#                    #
######################\n");
        },
        8 => {
            println!("######################
#      _________     #
#      |/      |     #
#      |     (°_°)   #
#      |       |/    #
#      |       |     #
#      |             #
#      |             #
#  ____|____         #
#                    #
######################\n");
        },
        9 => {
            println!("######################
#      _________     #
#      |/      |     #
#      |     (°_°)   #
#      |      \\|/    #
#      |       |     #
#      |             #
#      |             #
#  ____|____         #
#                    #
######################\n");
        },
        10 => {
            println!("######################
#      _________     #
#      |/      |     #
#      |     (°_°)   #
#      |      \\|/    #
#      |       |     #
#      |      /      #
#      |             #
#  ____|____         #
#                    #
######################\n");
        },
        11 => {
            println!("######################
#      _________     #
#      |/      |     #
#      |     (°_°)   #
#      |      \\|/    #
#      |       |     #
#      |      / \\    #
#      |             #
#  ____|____         #
#                    #
######################\n");
            println!("YOU LOST :(");
            println!("The word was '{}'", word);
            println!("Congrats Gamemaster, you win :)\n");
            println!("I will now end the Game");
            thread::sleep(time::Duration::from_secs(5));
            end_the_game()
        },
        _ => {

        }
    }
}

fn end_the_game() {
    process::exit(0x0100)
}
