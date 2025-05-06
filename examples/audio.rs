fn main() {
    let audify_rs = audify_rs::core::Audify::new("audio.wav");
    
    audify_rs.synthesize("hey man").unwrap();
    println!("done");
}
