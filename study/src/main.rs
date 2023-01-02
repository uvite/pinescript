extern crate nom;
extern crate nom_locate;

use nom::{FindSubstring, Slice};
use nom_locate::LocatedSpan;

fn main() {
    let program = LocatedSpan::new(
        "Hello World!\
    \nThis is a multi-line input\
    \nthat ends after this line.\n");
    let multi = program.find_substring("multi").unwrap();
   // let column=program.slice(multi..).get_line_beginning().clone();
    println!("{:?}-{:?}",multi,program.slice(multi..).get_line_beginning());

    // assert_eq!(
    //     program.slice(multi..).get_line_beginning(),
    //     "This is a multi-line input".as_bytes(),
    // );
}