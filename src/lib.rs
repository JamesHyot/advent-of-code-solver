mod days;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn get_day1_result(input: &str) -> String {
    days::day1::solver::solve(input)
}

#[wasm_bindgen]
pub fn get_day2_result(input: &str) -> String {
    days::day2::solver::solve(input)
}

#[wasm_bindgen]
pub fn get_day3_result(input: &str) -> String {
    days::day3::solver::solve(input)
}

#[wasm_bindgen]
pub fn get_day4_result(input: &str) -> String {
    days::day4::solver::solve(input)
}

#[wasm_bindgen]
pub fn get_day5_result(input: &str) -> String {
    days::day5::solver::solve(input)
}

#[wasm_bindgen]
pub fn get_day6_result(input: &str) -> String {
    days::day6::solver::solve(input)
}

#[wasm_bindgen]
pub fn get_day7_result(input: &str) -> String {
    days::day7::solver::solve(input)
}

#[wasm_bindgen]
pub fn get_day8_result(input: &str) -> String {
    days::day8::solver::solve(input)
}

#[wasm_bindgen]
pub fn get_day1_default_input () -> String {
    days::day1::input::get_input()
}

#[wasm_bindgen]
pub fn get_day2_default_input () -> String {
    days::day2::input::get_input()
}

#[wasm_bindgen]
pub fn get_day3_default_input () -> String {
    days::day3::input::get_input()
}

#[wasm_bindgen]
pub fn get_day4_default_input () -> String {
    days::day4::input::get_input()
}

#[wasm_bindgen]
pub fn get_day5_default_input () -> String {
    days::day5::input::get_input()
}

#[wasm_bindgen]
pub fn get_day6_default_input () -> String {
    days::day6::input::get_input()
}

#[wasm_bindgen]
pub fn get_day7_default_input () -> String {
    days::day7::input::get_input()
}

#[wasm_bindgen]
pub fn get_day8_default_input () -> String {
    days::day8::input::get_input()
}