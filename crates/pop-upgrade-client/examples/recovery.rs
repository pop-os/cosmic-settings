#[tokio::main(flavor = "current_thread")]
async fn main() {
    let connection = zbus::Connection::system().await.unwrap();
    let client = pop_upgrade_client::ClientProxy::new(&connection)
        .await
        .unwrap();

    let (version, build) = client.recovery_version().await.unwrap();
    eprintln!("recovery version {version}: build {build}");

    let (current, next, build, _urgent, is_lts) = client.release_check(true).await.unwrap();
    eprintln!("release for {current} to {next} with build {build} is_lts={is_lts}");
}
