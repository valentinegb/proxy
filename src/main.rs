use anyhow::anyhow;
use async_trait::async_trait;
use fast_socks5::{
    SocksError,
    server::{self, AuthSucceeded, Authentication, DenyAuthentication, Socks5Server},
};
use sha2::{Digest, Sha256, digest::generic_array::functional::FunctionalSequence};
use tokio_stream::StreamExt as _;
use tracing::{debug, error, info};

struct HashedPasswordAuthentication {
    username: String,
    /// A SHA-256 encrypted password hash.
    password_hash: String,
}

#[async_trait]
impl Authentication for HashedPasswordAuthentication {
    type Item = AuthSucceeded;

    async fn authenticate(&self, credentials: Option<(String, String)>) -> Option<Self::Item> {
        credentials.and_then(|(username, password)| {
            let username_matches = username == self.username;
            let password_hash = Sha256::digest(password.as_bytes())
                .map(|byte| format!("{byte:x}"))
                .join("");
            let password_matches = password_hash == self.password_hash;

            debug!(?username, ?self.username, ?username_matches, ?password_matches);

            (username_matches && password_matches).then_some(AuthSucceeded { username })
        })
    }
}

async fn try_main() -> anyhow::Result<()> {
    let server = Socks5Server::<DenyAuthentication>::bind("0.0.0.0:1080")
        .await?
        .with_config(
            server::Config::<DenyAuthentication>::default().with_authentication(
                HashedPasswordAuthentication {
                    username: "valentinegb".to_string(),
                    password_hash:
                        "bad001813383a2fbc884ba78e25ba61b6224bfa902cd27b614b295a906d146d2"
                            .to_string(),
                },
            ),
        );
    let mut incoming = server.incoming();

    info!("Listening for incoming connections");

    while let Some(socket) = incoming.next().await {
        tokio::spawn(async move {
            info!("Received connection");

            let res: Result<(), SocksError> = async {
                if let Some(target_addr) = socket?.upgrade_to_socks5().await?.target_addr() {
                    debug!(?target_addr);
                }

                Ok(())
            }
            .await;

            if let Err(err) = res {
                error!("{:#}", anyhow!(err));
            }
        });
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    match try_main().await {
        Ok(_) => info!("Shutting down"),
        Err(err) => error!("{err:#}"),
    }
}
