use anyhow::anyhow;
use fast_socks5::{
    SocksError,
    server::{self, DenyAuthentication, SimpleUserPassword, Socks5Server},
};
use tokio_stream::StreamExt as _;
use tracing::{debug, error, info};

async fn try_main() -> anyhow::Result<()> {
    let server = Socks5Server::<DenyAuthentication>::bind("127.0.0.1:1080")
        .await?
        .with_config(
            server::Config::<DenyAuthentication>::default().with_authentication(
                SimpleUserPassword {
                    username: "valentinegb".to_string(),
                    password: "xuhtez-7gixsy-Hiwcyc".to_string(),
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
