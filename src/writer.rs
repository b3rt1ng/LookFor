use std::io::{self, Write};

pub struct MultiWriter {
    writers: Vec<Box<dyn Write>>,
}

impl MultiWriter {
    pub fn new() -> Self {
        Self {
            writers: Vec::new(),
        }
    }

    pub fn add<W: Write + 'static>(&mut self, writer: W) {
        self.writers.push(Box::new(writer));
    }
}

impl Write for MultiWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for writer in &mut self.writers {
            writer.write_all(buf)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        for writer in &mut self.writers {
            writer.flush()?;
        }
        Ok(())
    }
}
