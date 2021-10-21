pub mod data_get {
    use std::{future::Future, ops::Index, process::Output};

    use crate::indexer::index_handle;

    use tokio;
    
    //#[tokio::main]
    pub fn get_file_name(data: String) {
        let cull: usize = 38;
        println!("\nFound File");
        let url: &str = &data.clone()[..];
        let data = data.replace("https://cdn.discordapp.com/attachments/","");
        // https://stackoverflow.com/questions/65976432/how-to-remove-first-and-last-character-of-a-string-in-rust
        let data: &str = &data[cull..data.len()];
        if data.contains("MemeFeedBot") { // this is for meme bots, this is the only one that adds theses strange numbers to the end // ?NzE4MTAzMTA1ODQxMTM1NjUw = 25
            let bot_cull: usize = 25;
            let data: &str = &data[0..data.len() - bot_cull];
            //println!("{}",data);

            let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
            let s = runtime.block_on(index_handle::download(url, data));

        } else {
            let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
            let s = runtime.block_on(index_handle::download(url, data));
        }
    }
}