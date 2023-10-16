pub fn hashMap() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrites 10 to 25

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    println!("---------------- exercises");
    let mut numbers = [37, 225, 8, 0, 9, 25, 168, 500, 15, 98, 100, 32, 32];

    fn median(numbers: &mut [i32]) -> i32 {
        numbers.sort();
        let median = numbers.len() / 2;
        numbers[median]
    }

    fn mode(numbers: &[i32]) -> i32 {
        let mut occurrences = HashMap::new();

        for &value in numbers {
            *occurrences.entry(value).or_insert(0) += 1;
        }

        occurrences
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(val, _)| val)
            .expect("cannot compute mode of zero numbers")
    }

    println!("{}", mode(&numbers));
    println!("{}", median(&mut numbers));
}
