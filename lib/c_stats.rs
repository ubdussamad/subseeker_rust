// ||||||||||||||||||| Comparison Statics
// Simple Statistical methods for comparing
// strings (Non Case Sensitive) for best hit selection.
// Authors: Mohammed S. Haque , Steve Aron Jr. , Hemant Arora (Co-author)
// ----------Required Declaration-------------
// This file contains comparison methods
// which have minor traces of code that 
// is derived or inspired from the
// Hidden Analysis toolkit's source code.
// All the derived techniques are DECLASSIFIED
// for public usage and are FREE to distribute
// given the user adds this documentaion atop the media.
// Documentation and usage terms are subject to change.
// Read adyybnhc1kge2b.onion for more details.
extern crate regex;
use regex::Regex;

fn main () {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let foo :i32 = 12;
    let bar :i32 = 12;
    println!("The sum of foo and bar is: {}.",sum(foo,bar));
    println!("Found date: {}" , assert!(re.is_match("2014-01-01")));   
}

fn sum ( x : i32 , y : i32 ) -> i32  {
    
    return x + y
}
