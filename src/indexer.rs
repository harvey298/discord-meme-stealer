pub mod index_handle {
    extern crate fs_extra;
    use fs_extra::file::{CopyOptions, move_file};

    use error_chain::error_chain;
    use std::borrow::Borrow;
    use std::io::{stdout, Write, copy, BufRead, BufReader};

    use std::fs::{File, OpenOptions};
    use std::fs;   

    use std::env;
    use std::path::Path;
    use tempfile::Builder;
    use checksums::{hash_file, Algorithm};

    use curl::easy::Easy;

    use rand::Rng;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            HttpRequest(reqwest::Error);
        }
   }

    pub async fn download(target: &str, raw_filename: &str) -> Result<u8> { // https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html
        println!("Downloading!");

        let processing_dir: String = "processing".to_string();
        let processing_dir2: &str = &processing_dir.clone()[..];
        let output_dir: &str = "memes";
        let _hash_data: &Path = Path::new("hashes.txt");
        let dupe_dir: &str = "dupes";
        let unknown_dir: &str = "unknown";

        // file ext grap
        //let raw_filename: &str = filename.clone();
        
        //let target: &str = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
        let mut rng = rand::thread_rng();

        let tmp_id: i32 = (rng.gen_range(1..9)+rng.gen_range(1..9)+rng.gen_range(1..9)+rng.gen_range(1..9));
        let temp_id_str: String = tmp_id.clone().to_string();

        let tmp_id2: i32 = (rng.gen_range(1..9)+rng.gen_range(1..9)+rng.gen_range(1..9)+rng.gen_range(1..9));
        let temp_id_str2: String = tmp_id2.clone().to_string();

        let tmp_id_3: String = temp_id_str+&temp_id_str2;

        //while tmp_id_3.len() < 4 {
        //    let tmp_id_3 = &tmp_id_3[1..tmp_id_3.len()];
        //    if tmp_id_3.len() == 4 {
                //return tmp_id_3;
        //    }
        //}

        let filename: String = processing_dir+&"/".to_string()+&tmp_id_3+&"tmpFile.process".to_string();

        if Path::new(processing_dir2).exists() == false {
            println!("Found no Processing directory! making one");
            fs::create_dir_all(processing_dir2)?;
        }
        if Path::new(output_dir).exists() == false {
            println!("Found no output directory! making one");
            fs::create_dir_all(output_dir)?;
        }

        if Path::new(&filename).exists() == true {
            println!("File already found! Changing file name!");
        }

        if _hash_data.exists() == false {
            File::create(_hash_data);
        }

        let filename2 = filename.clone();
        let filename3 = filename.clone();

        let filename_path = Path::new(&filename2);

        let mut _file = File::create(filename)?;
    
        let mut easy = Easy::new();
        easy.url(target).unwrap();
        easy.write_function(move |data| {
            _file.write_all(data);
            Ok(data.len())
        }).unwrap();
        easy.perform().unwrap();

        let found_file_hash: String = hash_file(filename_path, Algorithm::MD5);

        let __hash_file = File::open(_hash_data).unwrap();
        let hash_file_reader = BufReader::new(__hash_file);

        let __hash_file2 = File::open(_hash_data).unwrap();
        let hash_file_reader2 = BufReader::new(__hash_file2);
        let current_count: usize = hash_file_reader2.lines().count();

        for (index, line) in hash_file_reader.lines().enumerate() {
            let line = line.unwrap(); // Ignore errors.
            // Show the line and its number.
            //println!("{}. {}", index + 1, line);
            if line == found_file_hash {
                let options = CopyOptions::new();
                move_file(&filename3, dupe_dir, &options);
            }
        }

        if Path::new(&filename3).exists() == true {
            let mut hashfile = OpenOptions::new().append(true).open(_hash_data).expect(
                "cannot open hash file");
            let ready_to_Write: String = found_file_hash+"\n";
            hashfile.write_all(ready_to_Write.as_bytes()).expect("write failed");
            let options = CopyOptions::new();

            //let files = glob(output_dir).expect("Failed to read glob pattern").enumerate();
            
            if raw_filename.contains(".mp4") {
                let output_dir = output_dir.to_string()+"/"+&current_count.to_string()+".mp4";
                move_file(&filename3, output_dir, &options);

            } else if raw_filename.contains(".mp3") {
                let output_dir = output_dir.to_string()+"/"+&current_count.to_string()+".mp3";
                move_file(&filename3, output_dir, &options);

            } else if raw_filename.contains(".gif") {
                let output_dir = output_dir.to_string()+"/"+&current_count.to_string()+".gif";
                move_file(&filename3, output_dir, &options);
                
            } else if raw_filename.contains(".png") {
                let output_dir = output_dir.to_string()+"/"+&current_count.to_string()+".png";
                move_file(&filename3, output_dir, &options);
                
            } else if raw_filename.contains(".jpg") {
                let output_dir = output_dir.to_string()+"/"+&current_count.to_string()+".jpg";
                move_file(&filename3, output_dir, &options);
                
            } else if raw_filename.contains(".jpeg") {
                let output_dir = output_dir.to_string()+"/"+&current_count.to_string()+".jpeg";
                move_file(&filename3, output_dir, &options);
                
            } else if raw_filename.contains(".webm") {
                let output_dir = output_dir.to_string()+"/"+&current_count.to_string()+".webm";
                move_file(&filename3, output_dir, &options);
            } else {
                println!("Found Unknown file type! {}",raw_filename);
                let output_dir = unknown_dir.to_string()+"/"+&current_count.to_string()+"."+unknown_dir;
                move_file(&filename3, output_dir, &options);
            }

        }

        println!("Done!");
        Ok(54)
    }
}