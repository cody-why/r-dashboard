/*
 * @Author: plucky
 * @Date: 2022-10-10 11:51:28
 * @LastEditTime: 2022-10-15 13:43:11
 * @Description: 
 */

use std::process::Command;

fn main(){
    // println!("hello, build.rs");
    // tailwindcss -i index.css -o style.css
    Command::new("tailwindcss").args(&["-i", "index.css", "-o", "dist/.stage/style.css"])
        .status().unwrap();
    // cp static/favicon.ico ... dist/.stage
    Command::new("cp").args(&["static/favicon.ico", "...", "dist/.stage/"])
    .status().unwrap();
}