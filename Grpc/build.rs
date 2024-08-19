fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("generate proto");
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/hello.proto"], &["proto/"])?;
    Ok(())
}
