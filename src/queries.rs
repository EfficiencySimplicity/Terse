pub fn run_query_app(query: Vec<String>) {
    if query.is_empty() {
        // NOTE: you could just load the help page
        println!("No search query given! Try typing ts --help for info on how to use ts");
        return
    }

    // https://users.rust-lang.org/t/converting-a-vec-to-a-string-of-comma-separated-values/62853
    println!{"Sending query (not really): {}", query.join(" ")}
}