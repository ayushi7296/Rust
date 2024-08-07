// enums in rust is a type that represents data that is one of the several possible variants.
// Each variant in the enum can have data associated with it.



enum Snacks {
    Cookie,
    Chips,
    Chocolate
}

fn supper(item:Snacks) {
    match item{
    Snacks::Cookie => {
        println!("You want Cookies?");
    }

    Snacks::Chips => {
        println!("You want Chips?");
    }

    Snacks::Chocolate => {
        println!("You want Chocolates?");
    }

}
}

fn main() {
    supper(Snacks::Cookie);
    supper(Snacks::Chips);
    supper(Snacks::Chocolate);

}
