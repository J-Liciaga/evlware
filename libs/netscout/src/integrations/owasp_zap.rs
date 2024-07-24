use std::process::Command;
use std::error::Error;
use std::{thread, time};
use zap_api::ZapApiError;
use zap_api::ZapService;

pub async fn run_owasp_zap(
    target: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Running OWASP ZAP scan on {}", target);

    let zap_url = "http://localhost:8090".to_string();
    let zap_api_key = "KEY".to_string();
    let target_url = "http://localhost:8080".to_string();

    let service = ZapService {
        url: zap_url,
        api_key: zap_api_key,
    };

    // get the ZAP version
    let res = zap_api::core::version(&service);
    let zap_version;

    match res {
        Ok(v) => zap_version = v["version"].to_string(),
        Err(e) => return Err(e),
    }

    println!("ZAP Version: {}", zap_version);

    println!("Startin the std spider");

    let res = zap_api::spider::scan(
        &service,
        target_url,
        "-1".to_string(),       // max children,
        "true".to_string(),     // recurse
        "".to_string(),         // context name
        "false".to_string(),    // subtree only
    );

    let scan_id;

    match res {
        Ok(v) => {
            let res_status = &v["status"].as_str().unwrap();
            status = res_status.parse::<i32>().unwrap();
        }
        Err(e) => return Err(e),
    }

    println!("Scan status: {}", status);

    // TODO: run active scanner
    // TODO: display alerts

    Ok(())
}
