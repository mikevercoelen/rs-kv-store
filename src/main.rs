use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key was undefined");
    let value = arguments.next().expect("Value was undefined");
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();

    let database = Database::new();
}

struct Database {
    map: HashMap<String, String>
}

impl Database {
    fn new () -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;

        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key, value);
        }

        Ok(Database {
            map: HashMap::new()
        })
    }
}
