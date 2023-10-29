# tee.rs
Rust implementation of tee command.

**NOTE: This is a learning repository, so please do not use it in production.**

## Usage

```sh
$ echo "hello\nworld" | ./target/release/tee /tmp/hello.txt  
hello
world
skanehira@gorilla github.com/skanehira/tee.rs 
$ cat /tmp/hello.txt   
hello
world
```

## Author
skanehira
