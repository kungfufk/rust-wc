use std::io::Read;
use std::fs::File;

fn main() {
    let v: Vec<String> = std::env::args().collect();

    if v.len() < 2 {
        println!("Usage: <File>");
        return;
    }

    for i in 1..v.len() {
        let file = File::open(&v[i]);

        let mut words = 0;
        let mut chars = 0;     
        let mut lines = 0;
        let mut largest =0;
        let mut s=0;
        let mut d=0;

        match file {
            Ok(mut file) => {
                let mut content = String::new();

                match file.read_to_string(&mut content) {
                    Ok(_) => {
                        lines = content.lines().count();

                        for k in content.lines() {
    s += 1;

    let word_count = k.split_whitespace().count();

    words += word_count;

    if word_count > largest {
        largest = word_count;
        d = s;
    }
}

                        chars = content.chars().count();
                    }

                    Err(err) => eprintln!("{}", err),
                }
            }

            Err(err) => eprintln!("{}", err),
        }

        println!("{}\n{}\n{}\n{}\n{}\n", v[i], lines, words, chars,largest);
    }
}
