use juamp_r::miejsca::*;
use std::io;

fn main() {
    println!("=== JUAMP-R ===");
    println!("Aktualna wersja: 0.1");

    // Gracz wybiera swoje imię i miasto
    println!("\nWpisz swoje imię:");
    let mut imie = String::new();
    io::stdin().read_line(&mut imie).expect("Błąd odczytu");
    let imie = imie.trim();

    let miasto = "Zalewowice";
    println!("\nTwoje miasto: {miasto}.");
    println!("{miasto} to spokojne miasteczko położone w górach, gdzie wszyscy się znają.");

    // Menu główne
    let mut lokacja = "dom"; // Gracz zaczyna w domu
    loop {
        match lokacja {
            "dom" => lokacja = dom::w_dom(imie),
            "ulica" => lokacja = ulica::na_ulicy(imie),
            _ => {
                panic!("Nieoczekiwany błąd, gra się zakończyła.");
            }
        }
    }
}
