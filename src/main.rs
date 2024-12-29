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
            "dom" => lokacja = w_dom(imie),
            "ulica" => lokacja = na_ulicy(imie),
            _ => {
                panic!("Nieoczekiwany błąd, gra się zakończyła.");
            }
        }
    }
}

fn w_dom(imie: &str) -> &'static str {
    println!("\nJesteś w domu!");
    println!("1 - Wyjdź na zewnątrz");
    println!("2 - Odpocznij");
    println!("3 - Wyjdź z gry");

    let mut wybor = String::new();
    io::stdin().read_line(&mut wybor).expect("Błąd odczytu");
    match wybor.trim() {
        "1" => "ulica",
        "2" => {
            println!("Odpoczywasz przez chwilę. Czujesz się lepiej!");
            "dom"
        }
        "3" => {
            println!("Żegnaj, {}!", imie);
            std::process::exit(0);
        }
        _ => {
            println!("Nieprawidłowy wybór!");
            "dom"
        }
    }
}

fn na_ulicy(imie: &str) -> &'static str {
    println!("\nJesteś na ulicy!");
    println!("1 - Porozmawiaj z mieszkańcem");
    println!("2 - Wróć do domu");

    let mut wybor = String::new();
    io::stdin().read_line(&mut wybor).expect("Błąd odczytu");
    match wybor.trim() {
        "1" => {
            println!(
                "Spotykasz starszą osobę. Mówi ci: 'Dzień dobry, {}! Miło cię widzieć!'",
                imie
            );
            "ulica"
        }
        "2" => "dom",
        _ => {
            println!("Nieprawidłowy wybór!");
            "ulica"
        }
    }
}
