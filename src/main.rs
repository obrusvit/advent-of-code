mod d01;
mod d02;
mod d03;
mod d04;

fn main() {
    println!("Hello Advent of Code 2023!");
    let (d1p1, d1p2) = d01::solve_d1();
    println!("Day1 result: part 1 = {d1p1}");
    println!("Day1 result: part 2 = {d1p2}");

    let (d2p1, d2p2) = d02::solve_d2();
    println!("Day2 result: part 1 = {d2p1}");
    println!("Day2 result: part 2 = {d2p2}");

    let (d3p1, d3p2) = d03::solve_d3();
    println!("Day3 result: part 1 = {d3p1}");
    println!("Day3 result: part 2 = {d3p2}");

    let (d4p1, d4p2) = d04::solve_d4();
    println!("Day4 result: part 1 = {d4p1}");
    println!("Day4 result: part 2 = {d4p2}");

}
