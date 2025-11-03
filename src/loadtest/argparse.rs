use anyhow::{{Context, Result}};

struct TestProfileArguments {
    pub config_path: String,
    pub client_n: usize,
    pub output_csv_path: Option<String>,
}

impl TestProfileArguments {
    pub fn new(args: Vec<String>) -> Result<Self> {
       let config_path = args
        .get(1)
        .context(format!("Usage: {} <config-path> [num-clients]", args[0]))?
        .to_string();

    let client_n = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or_else(|| 1);

    Ok(Self { 
        config_path,
        client_n,
        output_csv_path: None,
    })
    }
}
