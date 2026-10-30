use std::env;
fn main(){
    let arguments : Vec<String> = env::args().collect();
    let command : &str = &arguments[1];

    match command{
        "add" => {
            println!("The operation to be performed is Addition")
        }

        "list" => {
            println!("The operation to be performed is List")
        }

        "delete" => {
            println!("The operation to be performed is Delete")
        }

        _ => {
            println!("This is a wild card entry ")
        }
    }
}
