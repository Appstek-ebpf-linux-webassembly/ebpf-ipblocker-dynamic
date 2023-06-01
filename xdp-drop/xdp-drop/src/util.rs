use std::fs::File;
use std::io::Read;
use serde_json::{self, Value};
use std::error::Error;
use std::fs;
//use serde_json;
use std::str::FromStr;
//use serde::de::Error;
use std::convert::TryInto;

use std::env;

pub fn pwd() {
    if let Ok(current_dir) = env::current_dir() {
        println!("Current working directory: {:?}", current_dir);
    } else {
        println!("Failed to get current working directory.");
    }
}


//////////////////////////////////
fn readvector()->Vec<Vec<[u8; 4]>>{
    // Create an empty vector of vector of [u8; 4]
    let mut data: Vec<Vec<[u8; 4]>> = Vec::new();

    // Create a few [u8; 4] arrays
    let arr1: [u8; 4] = [1, 2, 3, 4];
    let arr2: [u8; 4] = [5, 6, 7, 8];
    let arr3: [u8; 4] = [9, 10, 11, 12];

    // Create individual vectors and push the arrays
    let vec1 = vec![arr1, arr2];
    let vec2 = vec![arr3];

    // Push the individual vectors to the main vector
    data.push(vec1);
    data.push(vec2);

    // Print the vector
    println!("{:?}", data);

    data
}
//////////////////////////////////////////

fn read_json_file(file_path: &str) -> Result<Value, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;

    let json_data: Value = serde_json::from_str(&json_string)?;

    Ok(json_data)
}
//////////////////////////////////////////////////////
fn parsejson() {
    let file_path = "./src/input.json";
    match read_json_file(file_path) {
        Ok(json_data) => {
            // Process the JSON data
            println!("JSON data: {:?}", json_data);
        }
        Err(err) => {
            println!("Error reading JSON file: {}", err);
        }
    }
}
/////////////////////////////////////////////////////////


pub fn readjson()->Result<Value, Box<dyn std::error::Error>>  {
    // Read the JSON file as a string
    //let file_path = ".././xdp-drop/policy/input.json";
    let file_path = ".././Policy/output.json";
    //let file_path="/home/a0480/Documents/EBPF Rust Golang/EBPf Final Demo/Policy/output.json";
    pwd();
    let json_string = fs::read_to_string(file_path).expect("Failed to read file");
    //println!("{:?}", json_string);
    // Parse the JSON string into a serde_json::Value
    let json_data: serde_json::Value = serde_json::from_str(&json_string).expect("Failed to parse JSON");

    // Process the JSON data
    // Here, you can access the parsed JSON data using the `json_data` variable
    // and perform operations on it according to your requirements.
    //println!("JSON data: {:?}", json_data);

    for i in json_data.as_array(){
    //    print!("{:?}", i);

    }
    Ok(json_data)
}
