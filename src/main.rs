use std::io;
use rand::Rng;

fn main(){

    // generovani nahodneho cisla
    let mut b = rand::thread_rng(); 
    let rand_pc = b.gen_range(1..=100);

    println!("zadej svuj tip v rozsahu 1 - 100: ");
    let mut a = String::new();

    io::stdin().read_line(&mut a).expect("chyba");
    let mut rand_user:i32 = a.trim().parse().unwrap();

    while rand_pc != rand_user{

        if rand_user < 1 || rand_user > 100{
            println!("zadal jsi cislo mimo rozsah.");
        }
        else {
            println!("spatny tip, zkus to znovu!");

            let rozdil = (rand_user - rand_pc).abs(); // funkce abs vraci absolutni hodnotu

            if rozdil > 20{
                println!("jsi hodne daleko");
            }
            else if rozdil > 5 && rozdil <= 20{
                println!("getting there..");
            }
            else{
                println!("uz jsi skoro tam!");
            }
        }
            // novy tip od uzivatele

        println!("zadej svuj tip v rozsahu 1 - 100: ");
        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("chyba");
        rand_user = c.trim().parse().unwrap();
    }
    println!("uhadl jsi cislo, gratuluji! Tve cislo bylo {}", rand_pc);

    
}