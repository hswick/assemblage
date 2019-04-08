mod assemblage;

fn main() {
    
    let a = assemblage::null();
    let b = assemblage::null();
    let c = assemblage::and(a, b, "Primitive Assemblage", 42.0);
    println!("{}", c);

    // Define Cartesian space with assemblages
    let origin = assemblage::null();
    let x = assemblage::and(origin.clone(), origin.clone(), "X", 2.0);
    let y = assemblage::and(origin.clone(), origin.clone(), "Y", 4.0);
    let point2 = assemblage::and(x, y, "2D point", 0.0);
    println!("{}", point2);

    let z = assemblage::and(origin.clone(), origin.clone(), "Z", 42.0);
    let point3 = assemblage::and(point2, z, "3D Point", 0.0);
    println!("{}", point3);

    // Builder syntax
    let allison = assemblage::null().name("Allison");
    let harley = assemblage::null().name("Harley");
    let love = assemblage::and(harley, allison, "Love of Harley and Allison", 1.0);
    println!("{}", love);
   
}
