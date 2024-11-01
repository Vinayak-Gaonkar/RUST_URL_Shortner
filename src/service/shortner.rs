use random_string::generate;


pub fn generate_short_code()-> String{
    let charset = "abcdefghijklmnopqrstuvwxyz";

    let random_string = generate(6, charset);
    println!("[{}]", random_string);
    random_string
}
