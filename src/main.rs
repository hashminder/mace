use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::{self, BufRead};
use std::{vec};
use colored::*;
use rand::seq::SliceRandom;
use std::fs;
struct Vendor {
    mac : String,
    factory : String,
}

fn normalize (text : String ) -> String {
    text.to_lowercase().trim()
    .chars()
    .filter(|c| c.is_alphanumeric()||c.is_whitespace())
    .collect()
}

fn structer (enter:String) -> Vendor{
    let lengh = enter.len();
    let temp_struct = Vendor {
        mac : enter[0..=5].to_string(),
        factory: enter[6..lengh].to_string(),
    };
    return temp_struct;
}

fn mac_gen () -> Vec<String> { // generate a random mac address
    let mut mac : Vec<u8> = Vec::new();
    let mut _rng = thread_rng();
    while mac.len() != 3 {
        let trying: u8 = thread_rng().gen_range(0..=255);
        mac.push(trying);
    }
    let mut mac_converted: Vec<String> = Vec::new();
    for mac in mac {
        let mac_as_hex = format!("{:X}",mac);
        mac_converted.push(mac_as_hex);
    }
    return mac_converted;
}

fn correction (mut entry : String) -> String{ // to get a string and check if it has exact 6 pairs and is seprated by 2 
    let get_len = entry.len();
    if get_len != 12 {
        entry.push('0');
    }
    let mut temp_vec: Vec<char> = Vec::new();
    for numbers in entry.chars() {
        temp_vec.push(numbers);
    }
    let chunked = temp_vec.chunks_exact(2);
    let chunks_list: Vec<&[char]> = chunked.collect();
    let mut outcome: Vec<&[char]> = Vec::new();
    let sep = vec![':'];
    for parts in chunks_list {
        outcome.push(parts);
        outcome.push(&sep);
    }
    outcome.remove(11);
    let export :String = outcome.iter()
    .flat_map(|arr| arr.iter())
    .collect();
return export;
}

fn main() {
    let mut select = String::new();
    println!(r"
        |\
        | \        /|
        |  \____  / |
       /|__/AMMA\/  |
     /AMMMMMMMMMMM\_|
 ___/AMMMMMMMMMMMMMMA
 \   |MVKMMM/ .\MMMMM\
  \__/MMMMMM\  /MMMMMM---
  |MMMMMMMMMMMMMMMMMM|  /
  |MMMM/. \MM.--MMMMMM\/
  /\MMM\  /MM\  |MMMMMM   ___
 /  |MMMMMMMMM\ |MMMMMM--/   \-.
/___/MMMMMMMMMM\|M.--M/___/_|   \
     \VMM/\MMMMMMM\  |      /\ \/
      \V/  \MMMMMMM\ |     /_  /
        |  /MMMV'   \|    |/ _/
        | /              _/  /
        |/              /| \'
                       /_  /
                       /  /");
    println!("enter your factory name :");
    io::stdin().read_line(&mut select).expect("failed to read the input");
    let mac = mac_gen(); // random generated mac
    // search engin
    // read the text
        // open the file

    let mut macvec: Vec<String> = Vec::new(); // to find and push any mac search engin finds
    let file_path = "valid_mac.txt";
    let file = File::open(file_path)
    .expect("Failed to open file");
    // read it line by line  

    let reader = io::BufReader::new(file);
    for (_index,line) in reader.lines().enumerate(){

        // perform actions ???
        let word = line.expect("failed to read the line");
        let temp = structer(word);
        let search_for = normalize(temp.factory);
        let find_me = select.as_str();
        if search_for.contains(find_me.trim()) {
            macvec.push(temp.mac);
        }
    }
    let outery: Vec<_> = macvec.choose_multiple(&mut thread_rng(), 1).collect(); // to select 1 macs from macvec 
    let mut final_res: Vec<String> = Vec::new();
    for macs in outery {
        final_res.push(macs.to_string());
    }
    for macs in mac {
        final_res.push(macs.to_string());
    }
    let final_res1 = final_res.join("");
    let final_output = correction(final_res1);
    println!("here is your mac generated {} and written in mac_generated.txt" ,final_output.yellow().bold());
    fs::write("mac_generated.txt", final_output).expect("failed to write the generated mac");
}