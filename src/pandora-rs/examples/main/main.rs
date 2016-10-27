extern crate pandora;
extern crate serde_json;

use pandora::*;

fn main() {
    let res = login(DEFAULT_ENDPOINT, "john.doe@gmail.com", "johndoe");

    match res {
        Ok(res) => println!("Ok: {:?}", res),
        Err(e) => println!("Err: {:?}", e),
    }
}
