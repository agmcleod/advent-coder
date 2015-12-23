fn main() {
    let letters: Vec<u8> = (b'a'..b'{').collect();
    let bad_letters: Vec<u8> = vec![b'l', b'o', b'i'];

    let input: Vec<u8> = "hxbxwxba".split("").filter(|s| *s != "").map(|s| s.as_bytes()[0]).collect();
    println!("{:?}", input.iter().map(|c| *c as char)::Vec<char>::collect());
}
