use slug::slugify;
use std::env;
use std::io;

fn vypis_po_znaku(input: &str) {
    for c in input.chars() {
        println!("{}", c);
    }
}

fn vypis_delku(input: &str) {
    let delka = input.len();
    if delka < 2 {
        println!("Délka řetězce je {} znak", delka);
    } else if (delka > 2 && delka < 5) {
        println!("Délka řetězce jsou {} znaky", delka);
    } else {
        println!("Délka řetězce je {} znaků", delka);
    }
}

fn vypis_pozpatku(input: &str) {
    let po_zpatku = input.chars().rev().collect::<String>();
    println!("Pozpátku: {}", po_zpatku);
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    println!("Zadej text: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // vytvoření proměné slug na slugify
    let for_slug = input.clone();
    let slug = slugify(for_slug);

    //pushnutí inputu do Vec
    args.push(input.trim().to_string());

    // z nějakého důvodu mi to tam furt cpe: C:\Users\PechD\target\debug\ukol_dve.exe, když zavolám pozici 0, proto volám 1
    println!("{}", args[1]);

    //Menu
    println!(
        "
        Vyber úpravu ... (vyber z uvozovek)
        ------------------------------------------
        'lowercase' = převede text na lowercase,
        'uppercase' = převede text na uppercase,
        'no-spaces' = odebere mezery, 
        'slugify' = konvertuje text do slug,
        'znaky' = vypíše text do znaků,
        'delka' = vypíše délku řetězce,
        'zpetne' = vypíše pozpátku"
    );

    // volba a zpracování volby
    let mut volba = String::new();
    io::stdin()
        .read_line(&mut volba)
        .expect("Failed to read line");

    let choice = volba.trim();

    // match a.k.a. switch
    match choice {
        "lowercase" => println!("Do lowercase: {}", args[1].to_lowercase()),
        "uppercase" => println!("Do uppercase: {}", args[1].to_uppercase()),
        "no-spaces" => println!("Bez mezer: {}", args[1].replace(" ", "")),
        "slugify" => println!("Do slug: {}", slugify(slug)),
        "znaky" => println!("{:#?}", vypis_po_znaku(&args[1])),
        "delka" => println!("{:#?}", vypis_delku(&args[1])),
        "zpetne" => println!("{:#?}", vypis_pozpatku(&args[1])),
        _ => println!("Neplatná volba!"),
    }
}
