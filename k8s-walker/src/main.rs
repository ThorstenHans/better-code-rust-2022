use clap::{Arg, Command};
use k8s_openapi::api::core::v1::Namespace;
use kube::{api::ListParams, Api, Client};

const DEFAULT_LIMIT: u32 = 2;
const UNKNOWN: &str = "🤷‍♂️";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = build_cli().get_matches();
    let limit = app.get_one("limit").unwrap_or(&DEFAULT_LIMIT);

    // live demo grab some data from kubernetes cluster
    let client = Client::try_default().await?;
    let namespaces = Api::<Namespace>::all(client);
    let lp = ListParams::default().limit(*limit);
    println!(
        "Here are some (up to {}) of your namespaces in Kubernetes:\n",
        limit
    );
    namespaces.list(&lp).await?.items.iter().for_each(|ns| {
        println!(
            "{} [{}]",
            ns.metadata.name.as_ref().unwrap_or(&UNKNOWN.to_string()),
            ns.status.as_ref().unwrap().phase.as_ref().unwrap()
        )
    });
    Ok(())
}

fn build_cli() -> Command {
    Command::new("k8s-watcher")
        .about("Demonstrates how to use kube-rs")
        .arg(
            Arg::new("limit")
                .short('l')
                .long("limit")
                .value_name("LIMIT")
                .help("Determines how many items will be listed")
                .value_parser(clap::value_parser!(u32)),
        )
}
