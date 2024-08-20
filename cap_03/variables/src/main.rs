fn main() {
    // 1. inmutables a no ser que se declare
    let mut x = 5;
    println!("The value of x is: {x}");
    // No compila xq x es inmutable, distinto de que pongamos mut
    x = 6;
    println!("The value of x is: {x}");
    
    // 2. Constantes:
    #[allow(dead_code)]
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");
    // 3. Shadow
     let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
