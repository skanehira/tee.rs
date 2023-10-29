use std::io::prelude::*;

pub struct TeeReader<'t> {
    r: &'t mut dyn std::io::Read,
    w: &'t mut dyn std::io::Write,
}

impl<'t> TeeReader<'t> {
    pub fn new(r: &'t mut impl Read, w: &'t mut impl Write) -> Self {
        Self { r, w }
    }
}

impl<'t> Read for TeeReader<'t> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.r.read(buf)?;
        self.w.write_all(&buf[..n])?;
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use crate::TeeReader;
    use std::{
        fs::OpenOptions,
        io::{prelude::*, Cursor, SeekFrom},
    };

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_buffer() -> Result<()> {
        let text = "hello";
        let mut r = Cursor::new(text);
        let mut w = Cursor::new(vec![]);
        let mut tee = TeeReader::new(&mut r, &mut w);
        let mut buf = [0; 5];
        let readed = tee.read(&mut buf)?;
        assert_eq!(readed, 5);
        assert_eq!(String::from_utf8(w.into_inner())?, text);
        assert_eq!(String::from_utf8(buf.to_vec())?, text);
        Ok(())
    }

    #[test]
    fn test_file() -> Result<()> {
        let text = "hello world\n";
        let mut r = Cursor::new(text);
        let tmpfile = std::path::Path::new(&std::env::temp_dir()).join("test.txt");
        let mut w = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(tmpfile)?;
        let mut tee = TeeReader::new(&mut r, &mut w);

        let mut buf = [0; 12];
        let readed = tee.read(&mut buf)?;
        assert_eq!(readed, 12);
        assert_eq!(String::from_utf8(buf.to_vec())?, text);

        w.seek(SeekFrom::Start(0))?;
        let mut output = String::new();
        w.read_to_string(&mut output)?;
        assert_eq!(output, text);
        Ok(())
    }
}
