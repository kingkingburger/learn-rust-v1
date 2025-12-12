pub fn vector_test() {
    let v: Vec<i32> = Vec::new();
    let mut v2 = VEc::new();

    v2.push(3);

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no element"),
    }

    enum SpreadsheetCel {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCel::Int(3),
        SpreadsheetCel::Text(String::from("blue")),
        SpreadsheetCel::Float(10, 12),
    ];
}
