use tokio::process::Command;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Invoking cmd...");

    let cmd = Command::new("ls").arg("-al").output().await?;

    if !cmd.status.success() {
        anyhow::bail!("cmd failed to execute")
    }

    let result = String::from_utf8(cmd.stdout)?;
    println!("{}", result);
    Ok(())
}
