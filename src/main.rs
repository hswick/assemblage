mod assemblage;
use assemblage::Assemblage;

fn main() {
    let a = Assemblage::new();
    let b = Assemblage::new();
    let c = assemblage::add(a, b);

    println!("{}", c);
}
