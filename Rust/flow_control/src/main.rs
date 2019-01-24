fn main() {
    // match
    let x = 2u8;
    match x
    {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        // _ is the placeholder equivalent to default for switch-case
        // () is the unit type, it means to nothing
        _ => (),
    }

    // if let
    // a way to manage matching when just a few arms
    let x = 2u8;
    if let 2 = x
    {
        println!("two");
    }
    else if let 3 = x
    {
        println!("three");
    }

    let x = Some(4);
    if let Some(i) = x
    {
        println!("{}", i);
    }
}
