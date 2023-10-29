use std::io::prelude::*;
use tee::TeeReader;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        eprintln!(
            r#"Usage:
    echo "Hello World" | {} hello.txt
"#,
            args.get(0).unwrap()
        );
        std::process::exit(1);
    }

    let file_path = args.get(1).unwrap();
    let mut output = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(file_path)?;

    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let mut tee = TeeReader::new(&mut stdin, &mut stdout);

    loop {
        let mut buf = [0; 1024];
        let n = tee.read(&mut buf)?;
        if n == 0 {
            break;
        }
        output.write_all(&buf[0..n])?;
    }

    Ok(())
}
