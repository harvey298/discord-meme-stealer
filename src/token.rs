pub mod token_handle {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;

    pub fn no_token(_token_file: &str) -> std::io::Result<()> {
        let mut _file = File::create(_token_file)?;
        println!("\nPlease input your bots token:");
        let mut _token = String::new();
        io::stdin()
            .read_line(&mut _token)
            .expect("Failed to read line");
        _file.write_all(_token[..].as_bytes())?;
        Ok(())
    }
}
