mod assemblage;

fn main() {
    let a = assemblage::identity();
    let b = assemblage::identity();
    let c = assemblage::add(a, b);

    println!("{}", c);
}
