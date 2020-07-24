fn main() {
    let x = 1;
    // int, floating, bool, characters

    let y = 2.0;
    // cute error messages if there are no semicolons

    let math = 8+10;
    let myArray = [1,2,3,4,5];
    let myTuple = (5, 5.0, "chris");

    let (val1, val2, val3) = myTuple;

    println!("hello world");
    println!("value of x is {}", x);
    println!("value of array index is {}", myArray[4]);

    hello_world("Chris");
}

// rust is a more systems level language

// without the ampersand it'll complain it doesn't know the length of the value
fn hello_world(name: str) {
    println!("hello your mom {}", name);
}
