// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::seq::SliceRandom;

const LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
const BEFORE_AFTER: [&str; 2] = ["before", "after"];

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_answer(answer: &str, before_after: &str, letter: &str) -> (String, bool) {
    let correct_answer: char = match before_after {
        "before" => {
            LETTERS[LETTERS
                .iter()
                .position(|&x| x == letter.chars().next().unwrap())
                .unwrap()
                - 1]
        },
        "after" => {
            LETTERS[LETTERS
                .iter()
                .position(|&x| x == letter.chars().next().unwrap())
                .unwrap()
                + 1]
        },
        _ => panic!("I have no idea how it chose something other than before or after. Perhaps I have accidentally created a sentient AI somehow? Anyway, here's the value it gave: {}", before_after),
    };
    if answer.to_uppercase().chars().next().unwrap_or('A') == correct_answer {
        (
            format!(
                "{} is correct! Good job!",
                answer.to_uppercase().chars().next().unwrap_or('A')
            ),
            true,
        )
    } else {
        (
            format!(
                "{} is incorrect! Try again.",
                answer.to_uppercase().chars().next().unwrap_or('A')
            ),
            false,
        )
    }
}

#[tauri::command]
fn skip_question(before_after: &str, letter: &str) -> (char, String, String) {
    let correct_answer: char = match before_after {
        "before" => {
            LETTERS[LETTERS
                .iter()
                .position(|&x| x == letter.to_uppercase().chars().next().unwrap())
                .unwrap()
                - 1]
        },
        "after" => {
            LETTERS[LETTERS
                .iter()
                .position(|&x| x == letter.to_uppercase().chars().next().unwrap())
                .unwrap()
                + 1]
        },
        _ => panic!("I have no idea how it chose something other than before or after. Perhaps I have accidentally created a sentient AI somehow? Anyway, here's the value it gave: {}", before_after),
    };
    let y = get_new_question();
    (correct_answer, y.0, y.1)
}

#[tauri::command]
fn get_new_question() -> (String, String) {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let mut before_after = *BEFORE_AFTER.choose(&mut rng).unwrap();
    let letter = *LETTERS.choose(&mut rng).unwrap();

    if letter == 'A' {
        before_after = "after";
    }
    if letter == 'Z' {
        before_after = "before";
    }

    (before_after.to_string(), letter.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_answer,
            get_new_question,
            skip_question
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
