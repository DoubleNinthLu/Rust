use std::string::FromUtf8Error;

fn str_upper_match(str: Vec<u8>) -> Result<String, FromUtf8Error>{
    // let ret = match String::from_utf8(str) {
    //     Ok(str) => str.to_uppercase(),
    //     Err(err) => return Err(err),
    // };
    let ret = String::from_utf8(str).map(|s| s.to_uppercase())?;
    
    println!("Conversion succeeded: {}", ret);
    Ok(ret)
}

fn main() {
    let invalid_str = str_upper_match(vec![111, 112]);
    println!("{:?}", invalid_str);
}
