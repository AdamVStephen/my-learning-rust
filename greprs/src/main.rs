use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

const VERSION :f32 =0.0;

// TODO: find standard usage and args patterns from a pro rust program such as ripgrep
//
fn usage() {
    println!("greps pattern [filename]");
}

const TEST_POEM : &str = "
Here is a multiline test poem string.
The second line herein;
";

//fn assert_file_exists(filename : &str) -> std::io::Result<(), std::io::Error> {
fn assert_file_exists(filename : &str) -> std::io::Result<()> {
    let res:bool = std::path::Path::new(filename).exists();
    match res {
        true => { return Ok(()); },
        false => { return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "already exists")); },
    }
}

fn create_poem_file(filename : &str) -> std::io::Result<()> {
    println!("{}", TEST_POEM);
    // let filename = "test_poem.txt";
    // TODO: find the expected way to deal with this.
    // The following is a mess needing more error study
    /*
       let mut fh = match File::open(filename) {
       Ok(f) => {return Err("file exists");},
       Err(_) => File::create(filename)?;
       }
    //fh.write_all(TEST_POEM)?;
    */
    let mut res:bool = true;
    res = Path::new(filename).exists();
    match res {
        true => {return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, format!(" {} already exists", filename))); },
        false => {dbp(&format!("{} not yet in existence : we will make a new one", filename));},
    }

    let mut fh = File::create(filename)?;
    fh.write_all(TEST_POEM.as_bytes())?;

    Ok(())
}

// TODO: determine idiom for trace macros in rust
//

// TODO: find out how to have mutable global variables.
//const DEBUG_LEVEL :i32 = 451;
const DEBUG_LEVEL :i32 = 0;

//let debug_level :i32 = 0;

// TODO: find out how to write macros in rust

fn dbp(debug_message : &str) {
    if DEBUG_LEVEL  == 0  { return; }
    //if debug_level == 0 return;
    println!("{}",debug_message);
}

// TODO: how to rewrite dbp to take varargs a la dbpp(arrsomehow) {&format!(arrsomehow)}
fn dbpp(debug_message : &str) {
    if DEBUG_LEVEL  == 0  { return; }
    //if debug_level == 0 return;
    println!("{}",debug_message);
}
fn parse_args(args: &Vec<String> ) -> (&str, &str) {
    println!("{:?}", args);
    let cmd = &args[0];
    let n_args = args.len();
    dbp(&format!("Executing: {} with {} arguments", cmd, n_args));

    // TODO: find the idiom, but here is a nice rust construct to show progress
    //
    // Challenge : how to bind the variable name in the data structure to 
    // an actual variable subsequently (think eval in python pattern)

    let defaults = [
        ("pattern" , "default_pattern"), 
        ("filename" , "default_filename"), 
    ];

    //for (def_tup, index) in defaults.iter().enumerate() {
    for (index, def_tup) in defaults.iter().enumerate() {
        dbp(&format!("Challenge to find {} in args[{}] and set to {}", def_tup.0, index, def_tup.1));
    }

    let mut pattern : &str = { if n_args >= 2 { &args[1] } else {"default_pattern"} };
    let mut filename : &str = { if n_args >= 3 { &args[2] } else {"default_filename"} };

    let pattern : &str = { if n_args >= 2 { &args[1] } else {"default_pattern"} };
    let filename : &str = { if n_args >= 3 { &args[2] } else {"default_filename"} };
    /*
       match args.len() {
       1 => {pattern = &args[1]},
       _ => {},
       }
       */

    //let pattern = &args[1];
    //let filename = &args[2];

    dbp(&format!("Search for {} in {} ", pattern, filename));

    (pattern, filename)
}

fn main() {
    println!("grep : version {}", VERSION);
    dbp("debug is on");
    let args : Vec<String> = env::args().collect();
    let (pattern, filename) = parse_args(&args);
    dbp(&format!("Search for {} in {} ", pattern, filename));
    //    test_data();
    let poem_filename:&str = &"poem.txt";

    let test = match assert_file_exists(poem_filename) {
        Ok(v) => {println!("Test {} returned OK", poem_filename); 0},
        Err(e) => {println!("Test {} returned Err", poem_filename); 1},
    };

    create_poem_file(poem_filename);
}
