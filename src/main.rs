mod term;
mod snow;

fn main() -> std::io::Result<()> {
    term::run()?;
    Ok(())
}