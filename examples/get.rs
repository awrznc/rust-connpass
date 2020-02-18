use connpass;

fn main() {
    let mut query_params = [("keyword", "Rust")];
    let response = connpass::event::new().query(&mut query_params).get().expect("request error.");
    
    // print result
    println!("{:#?}", response);
}
