mod utils;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;

fn main() {
    println!("Hello Advent of Code 2023!");
    let (d1p1, d1p2) = d01::solve_d1();
    println!("Day1 result: part 1 = {d1p1}");
    println!("Day1 result: part 2 = {d1p2}");

    // let (d2p1, d2p2) = d02::solve_d2();
    // println!("Day2 result: part 1 = {d2p1}");
    // println!("Day2 result: part 2 = {d2p2}");

    // let (d3p1, d3p2) = d03::solve_d3();
    // println!("Day3 result: part 1 = {d3p1}");
    // println!("Day3 result: part 2 = {d3p2}");

    // let (d4p1, d4p2) = d04::solve_d4();
    // println!("Day4 result: part 1 = {d4p1}");
    // println!("Day4 result: part 2 = {d4p2}");

    // let (d5p1, d5p2) = d05::solve_d5();
    // println!("Day5 result: part 1 = {d5p1}");
    // println!("Day5 result: part 2 = {d5p2}");

    let (d6p1, d6p2) = d06::solve_d6();
    println!("Day6 result: part 1 = {d6p1}");
    println!("Day6 result: part 2 = {d6p2}");

    let (d7p1, d7p2) = d07::solve_d7();
    println!("Day7 result: part 1 = {d7p1}");
    println!("Day7 result: part 2 = {d7p2}");

}
