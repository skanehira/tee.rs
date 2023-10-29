# tee.rs
Rust implementation of tee command.

**NOTE: This is a learning repository, so please do not use it in production.**

## Usage

### Use as a command.

```sh
$ echo "hello\nworld" | ./target/release/tee /tmp/hello.txt  
hello
world
skanehira@gorilla github.com/skanehira/tee.rs 
$ cat /tmp/hello.txt   
hello
world
```

### Use as a crate

```sh
let text = "hello";
let mut r = Cursor::new(text);
let mut w = Cursor::new(vec![]);

let mut tee = TeeReader::new(&mut r, &mut w);

let mut buf = [0; 5];
let readed = tee.read(&mut buf)?;

assert_eq!(readed, 5);
assert_eq!(String::from_utf8(w.into_inner())?, text);
assert_eq!(String::from_utf8(buf.to_vec())?, text);
```

## Author
skanehira
