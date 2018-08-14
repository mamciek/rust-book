fn main() {
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-layin'",
        "seven swans a-swimmin'",
        "eight maids milkin'",
        "nine lords a-leapin'",
        "ten ladies dancin'",
        "eleven pipers pipin'",
        "twelve drummers drummin'",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut acc = String::new();
    for (index, (gift, day)) in gifts.iter().zip(days.iter()).enumerate() {
        let conj = if index == 0 {
            ""
        } else if index == 1 {
            " and "
        } else {
            ", "
        };
        acc = format!("{}{}{}", gift, conj, acc);

        println!(
            "On the {} day of Christmas, my true love gave to me \n{}\n\n",
            day, acc
        );
    }
}
