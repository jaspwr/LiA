use julia::api::Julia;

pub fn execute(code: String) -> Result<String, String> {
    let mut jl = match Julia::new() {
        Ok(jl) => jl,
        Err(e) => {
            return Err(
                format!("Failed to initialize Julia. Ensure you have Julia installed before using a `jl` statement. {}", e)
            );
        }
    };
    let api: String = String::from_utf8_lossy(include_bytes!("../../../LiaAPI.jl")).to_string();
    if let Err(e) = jl.eval_string(api) {
        println!("{}", e.to_string());
        panic!("Error loading LiaAPI.jl.");
    }

    let val = match jl.eval_string(&code) {
        Ok(v) => { Ok(v) },
        Err(e) => {
            Err(
                format!("Julia error: {}", e.to_string())
            )
        }
    }?;

    Ok(val.to_string())
}