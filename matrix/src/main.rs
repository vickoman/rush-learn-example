use std::io;
fn main() {
    println!("Esta es tu última oportunidad. Después de esto, no hay vuelta atrás. Toma la pastilla azul y la historia termina, te despiertas en tu cama y crees lo que quieras creer. Toma la pastilla roja y te quedas en el País de las Maravillas, y yo te mostraré cuán profundo es el agujero de conejo");

    println!("Recuerda, lo único que te ofrezco es la verdad. Nada más");

    let red_pill: &str = "🔴";
    let blue_pill: &str = "🔵";

    println!("¿Cuál eliges? 1.{} o 2.{}, escribe red o blue...", red_pill, blue_pill);

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .unwrap();

    if  choice.trim() == "red" {
        println!("Has elegido la pastilla roja, bienvenido al País de las Maravillas, adentremonos al agujero de conejo");
    } else if choice.trim() == "blue" {
        println!("Has elegido la pastilla azul, te despiertas en tu cama y crees lo que quieras creer");
    } else {
        println!("No has elegido ninguna de las dos pastillas, así que te quedas en el limbo");
    }
}
