
fn main() {
    for index in 1..13 {
        write(index);
        println!();
    }
}

fn write(n: usize) {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let strings = [
        "Twelve lords a leaping",
        "Eleven ladies dancing",
        "Ten pipers piping",
        "Nine drummers drumming",
        "Eight maids a milking",
        "Seven swans a swimming",
        "Six geese a laying",
        "Five gold rings",
        "Four colley birds",
        "Three French hens",
        "Two turtle doves, and"
    ];

    println!("The {} day of Christmas,", days[n - 1]);
    println!("My true love sent to me");

    for index in (12 - n)..11 {
        println!("{}", strings[index]);
    }

    println!("A partridge in a pear tree.");
}
