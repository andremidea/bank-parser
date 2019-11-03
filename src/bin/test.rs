use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = File::create("/tmp/bla.ast")?;
    buffer.write_all(b"Blablabla")?;
    buffer.flush();


    let mut buffer2 = tempfile::tempfile_in("/tmp")?;
    buffer2.write_all(b"fooo bar");
    print!("{:?}", buffer2);
    buffer2.flush();

    Ok(())
}
