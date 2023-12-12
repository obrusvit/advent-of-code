#![allow(dead_code)]
#![allow(unused_variables)]

mod utils;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;

fn main() {
    // println!("Hello Advent of Code 2023!");
    // let (d1p1, d1p2) = d01::solve_d1();
    // println!("Day1 result: part 1 = {d1p1}");
    // println!("Day1 result: part 2 = {d1p2}");

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

    // let (d6p1, d6p2) = d06::solve_d6();
    // println!("Day6 result: part 1 = {d6p1}");
    // println!("Day6 result: part 2 = {d6p2}");

    // let (d7p1, d7p2) = d07::solve_d7();
    // println!("Day7 result: part 1 = {d7p1}");
    // println!("Day7 result: part 2 = {d7p2}");

    // let (d8p1, d8p2) = d08::solve_d8();
    // println!("Day8 result: part 1 = {d8p1}");
    // println!("Day8 result: part 2 = {d8p2}");

    // let (d9p1, d9p2) = d09::solve_d9();
    // println!("Day9 result: part 1 = {d9p1}");
    // println!("Day9 result: part 2 = {d9p2}");

    let (d10p1, d10p2) = d10::solve_d10();
    println!("Day10 result: part 1 = {d10p1}");
    // TODO
    println!("Day10 result: part 2 = {d10p2}");

    let (d11p1, d11p2) = d11::solve_d11();
    println!("Day11 result: part 1 = {d11p1}");
    println!("Day11 result: part 2 = {d11p2}");
}
