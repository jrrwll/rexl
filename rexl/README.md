# A common library for Rust

## argument parser
```rust
fn test_arg_parser() {
    let args = vec![
        "aux", "-n3", "-o", "yaml", "-P6379", "--rm=false", "-owide",
        "-H", "x-opts=gzip", "-Ffilename=awesome.rb", "-Ffilemode=777",
        "-R", "svc", "ep", "ds", "-n5",
        "-H", "Accept: */*", "-H", "User-Agent: curl/7.54.0",
        "--", "nowarn", "noredirect"].iter().map(|s| s.to_string()).collect();
    println!("{:?}", &args);

    let mut parser = get_parser();
    match parser.parse_bsd(args) {
        Ok(_) => {
            println!("integer P:\t{:?}", parser.get_integer("P"));
            println!("bool rm:\t{:?}", parser.get_bool("rm"));
            println!("bool a:\t{:?}", parser.get_bool("a"));
            println!("bool u:\t{:?}", parser.get_bool("u"));
            println!("bool x:\t{:?}", parser.get_bool("x"));
            println!("strings o:\t{:?}", parser.get_strings("o"));
            println!("strings H:\t{:?}", parser.get_strings("H"));
            println!("strings R:\t{:?}", parser.get_strings("R"));
            println!("properties F:\t{:?}", parser.get_properties("F"));
            println!("extra values:\t{:?}", parser.get_extra_values());

            assert_eq!(Some(&6379), parser.get_integer("P"));
            assert_eq!(false, parser.get_bool("rm"));
            assert_eq!(true, parser.get_bool("a"));
            assert_eq!(true, parser.get_bool("u"));
            assert_eq!(true, parser.get_bool("x"));
            assert_eq!(&vec!["yaml".to_string()], parser.get_strings("o").unwrap());
            assert_eq!(&vec!["x-opts=gzip".to_string(), "Accept: */*".to_string(), "User-Agent: curl/7.54.0".to_string()],
                       parser.get_strings("H").unwrap());
            assert_eq!(&vec!["svc".to_string(), "ep".to_string(), "ds".to_string()],
                       parser.get_strings("R").unwrap());

            let props = parser.get_properties("F").unwrap();
            assert_eq!("awesome.rb", props["filename"]);
            assert_eq!("777", props["filemode"]);

            assert_eq!(&vec!["nowarn".to_string(), "noredirect".to_string()],
                       parser.get_extra_values());
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
}
```

## interpolations for dollar $ and brace {}
```rust
fn test_dollar() {
    let mut context = HashMap::new();
    context.insert("Eric".to_string(), "Clapton".to_string());
    context.insert("John".to_string(), "Lennon".to_string());
    context.insert("Bob".to_string(), "Dylan".to_string());
    context.insert(":\\:".to_string(), "colon_backslash_colon".to_string());
    let template = "$$ :: $Eric is a friend of $John, but not a friend of $Bob! ::$$";
    show(template, &context, "$$ :: Clapton is a friend of Lennon, but not a friend of Dylan! ::$$");
}

fn show(template: &str, context: &HashMap<String, String>, expect: &str) {
    println!("{}", template);
    match dollar_named(template, &context, Some("NULL")) {
        Ok(formatted) => {
            println!("{}\n", formatted);
            assert_eq!(formatted, expect)
        }
        Err(err) => {
            eprintln!("{:?}\n", err);
        }
    }
}
```

## time it (just like `timeit` in Python)
```rust
fn test_time_it() {
    let ts = Timeit::new()
        .add_unary_action(|| {1}, |_|{
            // println!("invoke 1");
            let _ = vec![0; 100_000]; // must be 0 if you wanna effective
        } )
        .add_action(|| {
            // println!("invoke 2");
            let _: Vec<i32> = Vec::with_capacity(100_000);
        })
        .add_action(|| {
            // println!("invoke 3");
            for _ in 0..100_000 {
            }
        })
        .add_unary_action(|| {1}, |_|{
            // println!("invoke 4");
            let _ = vec![1; 100_000];
        } )
        .add_unary_action(|| {1}, |_|{
            // println!("invoke 4");
            let _ = vec![0u64; 100_000];
        } )
        .repeat(10).count(20).skip(5)
        .run_and_format_us("\t");
    // output like: 85.131us	7.026us	30633.678us	33903.761us	164.901us
    println!("{}", ts);
}
```