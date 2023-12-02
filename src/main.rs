mod d01;
mod d02;

fn main() {
    println!("Hello Advent of Code 2023!");
    let (d1p1, d1p2) = d01::solve_d1();
    println!("Day1 result: part 1 = {d1p1}");
    println!("Day1 result: part 2 = {d1p2}");

    let (d2p1, d2p2) = d02::solve_d2p1();
    println!("Day2 result: part 1 = {d2p1}");
    println!("Day2 result: part 2 = {d2p2}");


}
