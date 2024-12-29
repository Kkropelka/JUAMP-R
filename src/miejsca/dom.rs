use std::io;

pub fn w_dom(imie: &str) -> &'static str {
    println!("\nJesteś w domu!");
    println!("1 - Wyjdź na zewnątrz");
    println!("2 - Odpocznij");
    println!("3 - Wyjdź z gry");
    println!("4 - Zjedz cos");

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
        "4" => {
            println!("Zjadłeś coś smacznego. Czujesz się pełen energii!");
            "dom"
        }
        _ => {
            println!("Nieprawidłowy wybór!");
            "dom"
        }
    }
}
