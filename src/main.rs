use ftp::FtpStream;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").await?;
    ftp_stream.login("user", "password").await?;
    println!("Connexion réussie!");
    ftp_stream.quit().await?;

    Ok(())
}
