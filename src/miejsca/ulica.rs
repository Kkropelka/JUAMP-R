use rand::{seq::IteratorRandom, thread_rng};
use std::{collections::HashMap, io};

pub fn na_ulicy(imie: &str) -> &'static str {
    println!("\nJesteś na ulicy!");
    println!("1 - Porozmawiaj z mieszkańcem");
    println!("2 - Wróć do domu");

    let mut wybor = String::new();
    io::stdin().read_line(&mut wybor).expect("Błąd odczytu");
    match wybor.trim() {
        "1" => {
            println!("{}", get_dialog(imie));
            "ulica"
        }
        "2" => "dom",
        _ => {
            println!("Nieprawidłowy wybór!");
            "ulica"
        }
    }
}

fn get_dialog(imie: &str) -> String {
    let mut dialogi: HashMap<i8, String> = HashMap::new();
    dialogi.insert(
        1,
        format!(
            "Spotykasz starszą osobę. Mówi ci: 'Dzień dobry, {}! Miło cię widzieć!'",
            imie
        ),
    );
    dialogi.insert(
        2,
        format!("Spotykasz rówieśnika. Mówi ci: 'Siemka, {}!'", imie),
    );

    let mut rng = thread_rng();
    let key = dialogi.keys().choose(&mut rng).unwrap();

    dialogi.get(&key).cloned().unwrap()
}
