use std::io;

pub fn na_ulicy(imie: &str) -> &'static str {
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
