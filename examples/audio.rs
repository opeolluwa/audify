fn main() {
    let audify_rs = audify_rs::core::Audify::new();

    audify_rs.synthesize("test2.pdf", "audio2.wav").unwrap();
    println!("done");
}
