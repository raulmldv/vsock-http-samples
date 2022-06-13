use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    for _i in 1..5 {
        let resp = reqwest::blocking::get("http://0.0.0.0:8000/api/v1/find")?.text()?;
        println!("{:#?}", resp);
    }

    Ok(())
}