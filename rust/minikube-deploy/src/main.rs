use argh::FromArgs;
use k8s_openapi::api::core::v1::Pod;
use kube::api::PostParams;
use kube::config::AuthInfo;
use kube::{Api, Client, Config, ResourceExt as _};

#[derive(FromArgs, Debug)]
/// A CLI tool to deploy minikube pods
///
/// the tool always deploy two alpine containers for a pod
struct CliArgs {
    /// minikube ip; if not provided it will be fetched using `minikube ip`
    #[argh(option)]
    address: Option<String>,
    /// pod namespace
    #[argh(option, default = "\"default\".to_string()")]
    namespace: String,
    /// minikube port
    #[argh(option, default = "8443")]
    port: u16,
    /// pod name
    #[argh(option)]
    pod_name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let CliArgs {
        address,
        port,
        namespace,
        pod_name,
    } = argh::from_env::<CliArgs>();

    let address = match address {
        Some(address) => address,
        None => minikube_ip()?,
    };

    let home = dirs::home_dir().unwrap().to_string_lossy().to_string();
    let auth_info = AuthInfo {
        username: Some("minikube".to_string()),
        client_certificate: Some(format!("{home}/.minikube/profiles/minikube/client.crt")),
        client_key: Some(format!("{home}/.minikube/profiles/minikube/client.key")),
        ..Default::default()
    };
    let config = Config {
        cluster_url: format!("https://{address}:{port}").parse().unwrap(),
        default_namespace: namespace.clone(),
        read_timeout: None,
        root_cert: None,
        connect_timeout: None,
        write_timeout: None,
        accept_invalid_certs: true,
        auth_info,
        proxy_url: None,
        tls_server_name: None,
    };

    let client = Client::try_from(config)?;
    let api: Api<Pod> = Api::default_namespaced(client);

    let pod: Pod = serde_json::from_value(serde_json::json!({
      "apiVersion": "v1",
      "kind": "Pod",
      "metadata": { "name": pod_name },
      "spec": {
        "containers": [
          {
            "name": "container-a",
            "image": "alpine",
            "command": ["tail", "-f", "/dev/null"]
          },
          {
            "name": "container-b",
            "image": "alpine",
            "command": ["tail", "-f", "/dev/null"]
          }
        ]
      }
    }
    ))?;

    let post_params = PostParams::default();
    match api.create(&post_params, &pod).await {
        Ok(o) => {
            let name = o.name_any();
            assert_eq!(pod.name_any(), name);
            println!("Created pod {name}");
        }
        Err(kube::Error::Api(ae)) => assert_eq!(ae.code, 409), // if you skipped delete, for instance
        Err(e) => panic!("failed to create: {e}"),             // any other case is probably bad
    }

    let establish = kube::runtime::wait::await_condition(
        api.clone(),
        &pod_name,
        kube::runtime::conditions::is_pod_running(),
    );

    println!("Waiting for pod to be running...");
    let _ = tokio::time::timeout(std::time::Duration::from_secs(30), establish)
        .await
        .expect("pod timeout");

    println!("Pod is running");

    Ok(())
}

/// run command `minikube ip` and get output
fn minikube_ip() -> anyhow::Result<String> {
    let output = std::process::Command::new("minikube")
        .arg("ip")
        .output()?
        .stdout;

    let address = String::from_utf8(output)?.trim().to_string();

    Ok(address)
}
