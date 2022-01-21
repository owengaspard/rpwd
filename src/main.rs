use std::env;

fn main() {
    pwd();
}

fn pwd() -> std::io::Result<()>{
    let pwd = env::current_dir()?;
    println!("pwd: {}", pwd.display());
    Ok(())
}