extern crate unidecode;
use std::fs;
use std::io;
use std::io::BufRead;
use unidecode::unidecode;

struct Dict {
    data: Vec<String>,
}

impl Dict {
    fn find_word(&self, word: &str) -> bool {
        self.data.iter().find(|&x| x == word).is_some()
    }

    fn find_word_from_letters(&self, letters: &str) -> Vec<String> {
        let mut data: Vec<_> = self.data.iter().map(|x| (x.clone(), x.clone())).collect();
        for x in letters.chars() {
            for (word, rest) in data.iter_mut() {
                *rest = rest.replacen(x, "", 1);
            }
        }

        data.into_iter()
            .filter(|(_, x)| x.len() == 0)
            .map(|(x, _)| x)
            .collect()
    }
}

fn load(filename: &str) -> io::Result<Dict> {
    let data: Vec<_> = io::BufReader::new(fs::File::open(filename)?)
        .lines()
        .filter_map(|x| x.ok().map(|x| unidecode(x.as_ref()).to_lowercase()))
        .collect();

    Ok(Dict { data })
}

#[test]
fn test_read_dict() {
    let dict = load("liste_francais.txt").unwrap();
    assert!(dict.data.len() > 0);
}

#[test]
fn test_find_word() {
    let dict = load("liste_francais.txt").unwrap();
    assert!(dict.find_word("abdomen"));
    assert!(!dict.find_word("zzz"));
    assert!(
        dict.find_word("abandonnee"),
        format!("{:?}", dict.data.iter().take(15).collect::<Vec<_>>())
    );
}

#[test]
fn test_find_word_from_letters() {
    let dict = load("liste_francais.txt").unwrap();
    let empty: Vec<String> = Vec::new();

    assert_ne!(dict.find_word_from_letters("abdomne").len(), 0);
    assert_ne!(dict.find_word_from_letters("ebdomna").len(), 0);
    assert_eq!(dict.find_word_from_letters("zzz"), empty);
}

fn main() {
    let dict = load("liste_francais.txt").unwrap();

    println!(
        "{:?}",
        dict.find_word_from_letters("qosareaveirk")
            .iter()
            .filter(|x| x.len() == 8)
            .collect::<Vec<_>>()
    );
    println!(
        "{:?}",
        dict.find_word_from_letters("emcxwdiadjfi")
            .iter()
            .filter(|x| x.len() == 5)
            .collect::<Vec<_>>()
    );
    println!(
        "{:?}",
        dict.find_word_from_letters("huuwborarqgt")
            .iter()
            .filter(|x| x.len() == 7)
            .collect::<Vec<_>>()
    );
    println!(
        "{:?}",
        dict.find_word_from_letters("ahnbqiresecm")
            .iter()
            .filter(|x| x.len() == 7)
            .collect::<Vec<_>>()
    );
}
