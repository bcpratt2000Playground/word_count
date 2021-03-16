use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut words_vec: Vec<(String, i32)> = Vec::new();
    {
        let mut words = HashMap::new();
        let str= remove_punctuation(fs::read_to_string(&args[1]).unwrap().to_lowercase());
        for i in str.split_whitespace(){
            if words.contains_key(i){
                *words.get_mut(i).unwrap()=words.get(i).unwrap()+1;

            } else{
                words.insert(i, 1);
            }
        }
        for (k, v) in words.iter(){
            words_vec.push(((**k).to_string(), *v));
        }
    }
    words_vec.sort_by(|b, a| a.1.cmp(&b.1));

    for (k, v) in words_vec.iter(){
        // if *v > 1{
            println!("{}, {}", k, v);
        // }
    }

}

fn remove_punctuation(input: String) -> String{
    let mut output = String::with_capacity(input.len());
    for &i in input.as_bytes().iter(){
        if (i <=90 && i >=65) || (i == ' ' as u8) || (i == '\'' as u8) || (i >=97 && i <=122){
            output.push(i as char);
        } else {
            output.push(' ');
        }
    }
    // println!("{}", output);
    output.shrink_to_fit();
    output
}