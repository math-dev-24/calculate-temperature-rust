use std::io;

fn parse_int(number: &str) -> Result<Option<i32>, std::num::ParseIntError> {
    match number.trim().parse() {
        Ok(num) => {
            if num <= 7 && num > 0 {
                return Ok(Some(num));
            }else{
                Ok(None)
            }
        },
        Err(err) => Err(err)
    }
}

fn parse_degree(number: &str)-> Result<f64, std::num::ParseFloatError> {
    match number.trim().parse() {
        Ok(num) => Ok(num),
        Err(err) => Err(err)
    }
}

fn request_temperature(unit: &str) -> Option<f64> {
    println!("Entrez la température en {}", unit);
    let mut entree = String::new();
    io::stdin().read_line(&mut entree).expect("Erreur lors de la lecture de l'entrée");
    
    match parse_degree(&entree) {
        Ok(num) => Some(num),
        Err(_) => None
    }
}

fn handle_conversion(unit: &str, conversion_fn: fn(f64) -> f64, target_unit: &str) {
    if let Some(num) = request_temperature(unit) {
        println!("Voici la conversion : {}{}", conversion_fn(num), target_unit);
    } else {
        println!("Nombre invalide !");
    }
}


fn c_to_f(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn c_to_k(celsius: f64) -> f64 {
    celsius + 273.15
}

fn f_to_k(f: f64) -> f64 {
    f
}

fn k_to_c(c: f64) -> f64 {
    c - 273.15
}
fn k_to_f(k: f64) -> f64 {
    k
}

fn main() {
    println!("Bienvenue dans le convertisseur de températures !");
    let choice = ["Celsius → Fahrenheit", "Celsius → Kelvin", "Fahrenheit → Celsius", "Fahrenheit → Kelvin", "Kelvin → Celsius", "Kelvin → Fahrenheit", "Quitter"];
    
    loop {
        println!("\n Choisissez une option :");
        for (index, value) in choice.iter().enumerate() {
            println!("{}. {}", index + 1, value);
        }

        let mut choice: String = String::new();
        io::stdin().read_line(&mut choice).expect("Erreur lors de l'entrée !");


        match parse_int(&choice) {
            Ok(Some(7)) => {
                println!("Au revoir !");
                break;
            },
            Ok(Some(num)) => {
                match num {
                    1 => handle_conversion("°C", c_to_f, "F"),
                    2 => handle_conversion("°C", c_to_k, "K"),
                    3 => handle_conversion("F", f_to_c, "°C"),
                    4 => handle_conversion("F", f_to_k, "K"),
                    5 => handle_conversion("K", k_to_c, "°C"),
                    6 => handle_conversion("K", k_to_f, "F"),
                    _ => unreachable!(),
                }
            },
            Ok(None) => {
                println!("Erreur !")
            }
            Err(err) => {
                println!("{}", err)
            }
        }

    }

}

