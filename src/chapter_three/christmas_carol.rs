pub fn print_christmas_carol(day: i32) {
    println!("One the {day} of Christmas my true love sent to me");
    let mut days_left = day;
    while days_left >= 0 {
        if days_left == 0 {
            if day == 0 {
                println!("A partridge in a pear tree");
            } else {
                println!("And a partridge in a pear tree.");
            }
        } else if days_left == 1 {
            println!("Two turtle doves,");
        } else if days_left == 2 {
            println!("Three French hens,");
        } else if days_left == 3 {
            println!("Four calling birds,")
        }
        days_left -= 1
    }
}