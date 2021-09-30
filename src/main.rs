use std::io;
use std::process::Command;
use std::fs;
extern crate json;

fn main() {
    let jsonfile = fs::read_to_string("submodules.json").expect("Error in reading file: submodules.json");
    let parsed = json::parse(&jsonfile).unwrap();
    let mut input = String::new();
    println!("Enter type of boilerplate: ");
    match io::stdin().read_line(&mut input){
        Ok(_) => (),
        Err(e) => println!("Error in input: {}", e)
    };
    input.truncate(input.len()-1);
    
    for i in 0..parsed["submodules"].len() {
        println!("{}", parsed["submodules"][i]["tags"].contains(&*input));
        if parsed["submodules"][i]["tags"].contains(&*input) {
            let url = &parsed["submodules"][i]["url"];
            println!("{}",url);
            let output = Command::new("git")
                .arg("clone")
                .arg(url.to_string())
                .output()
                .expect("failed to execute process");
                if output.status.success() {
                    println!("Cloned {} successfully",parsed["submodules"][i]["name"]);
                }
                else{
                    println!("Error Encountered while cloning");
                }
        }
    }
}