fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was undefined");
    let value = arguments.next().expect("Value was undefined");
    let contents = format!("{}/t{}/n", key, value);

    std::fs::write("kv.db", contents);
}
