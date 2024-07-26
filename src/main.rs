use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::fs::File;
use std::io::{self, Read, BufRead};
use std::sync::Mutex;
use std::time::Instant;
use indicatif::ProgressBar;

const MAXS: usize = 100000;
const MAXC: usize = 128;
const BIT_WIDTH: usize = std::mem::size_of::<usize>() * 8;

lazy_static! {
    static ref CHAR_TO_INDEX: HashMap<char, usize> = {
        let mut m = HashMap::new();
        for i in 0..MAXC {
            m.insert(i as u8 as char, i);
        }
        m
    };
    static ref OUT: Mutex<HashMap<usize, Vec<usize>>> = Mutex::new(HashMap::new());
    static ref F: Mutex<Vec<isize>> = Mutex::new(vec![-1; MAXS]);
    static ref G: Mutex<Vec<Vec<isize>>> = Mutex::new(vec![vec![-1; MAXC]; MAXS]);
}

fn get_file_size(path: &str) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

fn read_all_bytes(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let size = get_file_size(path)? as usize;
    let mut buffer = vec![0; size];
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}

fn build_matching_machine(arr: &[String], k: usize) {
    let mut states = 1;

    let mut out = OUT.lock().unwrap();
    let mut f = F.lock().unwrap();
    let mut g = G.lock().unwrap();

    for (word_idx, word) in arr.iter().enumerate().take(k) {
        let mut current_state = 0;

        for c in word.chars() {
            let ch = *CHAR_TO_INDEX.get(&c).unwrap_or(&0);

            if g[current_state][ch] == -1 {
                if states >= MAXS {
                    panic!("Durum sayısı MAXS sınırını aştı");
                }
                g[current_state][ch] = states as isize;
                states += 1;
            }

            current_state = g[current_state][ch] as usize;
        }

        let entry = out.entry(current_state).or_insert_with(|| vec![0; (k + BIT_WIDTH - 1) / BIT_WIDTH]);
        if word_idx >= entry.len() * BIT_WIDTH {
            panic!("word_idx dizininin boyutunu aştı");
        }
        entry[word_idx / BIT_WIDTH] |= 1 << (word_idx % BIT_WIDTH);
    }

    for ch in 0..MAXC {
        if g[0][ch] == -1 {
            g[0][ch] = 0;
        }
    }

    let mut q = VecDeque::new();

    for ch in 0..MAXC {
        if g[0][ch] != 0 {
            f[g[0][ch] as usize] = 0;
            q.push_back(g[0][ch] as usize);
        }
    }

    while let Some(state) = q.pop_front() {
        for ch in 0..MAXC {
            if g[state][ch] != -1 {
                let mut failure = f[state] as usize;
                while g[failure][ch] == -1 {
                    if failure == 0 {
                        break;  // Eğer failure zaten 0 ise, daha fazla azaltma yapamayız.
                    }
                    failure = f[failure] as usize;
                }

                failure = g[failure][ch] as usize;
                f[g[state][ch] as usize] = failure as isize;

                let failure_out = out.entry(failure).or_insert_with(|| vec![0; (k + BIT_WIDTH - 1) / BIT_WIDTH]).clone();
                let state_out = out.entry(g[state][ch] as usize).or_insert_with(|| vec![0; (k + BIT_WIDTH - 1) / BIT_WIDTH]);

                for word_idx in 0..(k + BIT_WIDTH - 1) / BIT_WIDTH {
                    if word_idx >= state_out.len() || word_idx >= failure_out.len() {
                        panic!("word_idx dizininin boyutunu aştı");
                    }
                    state_out[word_idx] |= failure_out[word_idx];
                }

                q.push_back(g[state][ch] as usize);
            }
        }
    }
}

fn find_next_state(current_state: usize, next_input: char) -> usize {
    let ch = *CHAR_TO_INDEX.get(&next_input).unwrap_or(&0);

    let f = F.lock().unwrap();
    let g = G.lock().unwrap();

    let mut state = current_state;
    while g[state][ch] == -1 {
        if state == 0 {
            break;  // Eğer state zaten 0 ise, daha fazla azaltma yapamayız.
        }
        state = f[state] as usize;
    }
    g[state][ch] as usize
}

fn search_words(arr: &[String], k: usize, file_path: &str, pb: &ProgressBar) -> io::Result<()> {
    let text = read_all_bytes(file_path)?;
    build_matching_machine(&arr, k);

    let mut current_state = 0;

    let out = OUT.lock().unwrap();

    for (i, &byte) in text.iter().enumerate() {
        let c = byte as char;
        current_state = find_next_state(current_state, c);

        if let Some(state_out) = out.get(&current_state) {
            for word_idx in 0..(k + BIT_WIDTH - 1) / BIT_WIDTH {
                if state_out[word_idx] == 0 {
                    continue;
                }

                for j in 0..BIT_WIDTH {
                    if state_out[word_idx] & (1 << j) != 0 {
                        let keyword_idx = word_idx * BIT_WIDTH + j;
                        if keyword_idx < k {
                            let start_index = if i >= arr[keyword_idx].len() - 1 {
                                i - arr[keyword_idx].len() + 1
                            } else {
                                0
                            };
                            println!(
                                "Kelime '{}' {} ile {} arasında geçiyor",
                                arr[keyword_idx],
                                start_index,
                                i
                            );
                        }
                    }
                }
            }
        }
    }

    pb.inc(1); // Progress bar'ı artır

    Ok(())
}

fn sub_dir_list_files(path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();

    let entries = fs::read_dir(path).map_err(|e| {
        io::Error::new(io::ErrorKind::NotFound, format!("Dizin okunamadı: {}", e))
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Giriş okunamadı: {}", e))
        })?;
        
        if entry.file_type().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Dosya türü alınamadı: {}", e))
        })?.is_file() {
            let path = entry.path().display().to_string();
            println!("{}", path);
            files.push(path);
        }
    }

    println!(
        "Toplam {} dosya bulundu\nOkuma Başarılı\n\n---------------------------------\n\n",
        files.len()
    );

    Ok(files)
}

fn main() -> io::Result<()> {
    let mut csv_path = String::new();
    println!("CSV dosyasının yolunu girin: ");
    io::stdin().read_line(&mut csv_path)?;
    let csv_path = csv_path.trim();

    let mut dir_path = String::new();
    println!("Klasörün yolunu girin: ");
    io::stdin().read_line(&mut dir_path)?;
    let dir_path = dir_path.trim();

    let mut keywords = Vec::new();
    let file = File::open(csv_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        keywords.push(line?);
    }

    let k = keywords.len();

    let directories = sub_dir_list_files(dir_path)?;

    let pb = ProgressBar::new(directories.len() as u64);

    let start = Instant::now();

    for entry in directories {
        search_words(&keywords, k, &entry, &pb)?;
    }

    pb.finish_with_message("Tamamlandı!");

    let duration = start.elapsed();
    println!("Toplam süre: {:.2?} saniye", duration);

    Ok(())
}
