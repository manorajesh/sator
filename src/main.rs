fn main() {
    let wordlist = std::fs::read_to_string("words_alpha.txt").unwrap();
    let words: Vec<&str> = wordlist.lines().collect();

    let mut square: Vec<String> = Vec::new();
    let mut idx: usize = 1;

    'main: for master_word in &words {
        let rev_word: String = master_word.chars().rev().collect();
        if master_word.len() < 5 || !words.contains(&rev_word.as_str()) {
            continue;
        }
        
        square.push(master_word.to_string());
        for child_word in &words {
            if child_word.len() != master_word.len() {
                continue;
            }

            if master_word.chars().nth(idx).expect(format!("idx: {}, mword: {}, cword: {}", idx, master_word, child_word).as_str()) != child_word.chars().nth(0).unwrap() {
                continue;
            }

            if master_word.chars().nth(master_word.len()-idx-1).expect(format!("idx: {}, mword: {}, cword: {}", idx, master_word, child_word).as_str()) != child_word.chars().nth(child_word.len()-1).unwrap() {
                continue;
            }

            square.push(child_word.to_string());
            idx += 1;
            if square.len() == master_word.len()/2 {
                break 'main;
            }
        }
        square.clear();
        idx = 1;
    }

    let mut idx = 1;
    while square.len() < square[0].len() {
        let rev_word = square[idx].chars().rev().collect::<String>();
        square.push(rev_word);
        idx += 1;
    }
    print_square(&square);
}

fn print_square(square: &Vec<String>) {
    for word in square {
        for c in word.chars() {
            print!("{} ", c);
        }
        println!();
    }
    println!();
}