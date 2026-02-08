#![forbid(unsafe_code)]

mod schema;

use anyhow::Result;
use reqwest::Client;
use schema::{Deductions, OtherDeductions, Rates, Scales};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::ops::Deref;

#[derive(Serialize)]
struct Parameters {
    #[serde(rename(serialize = "TaxYear"))]
    tax_year: u32,
    #[serde(rename(serialize = "TaxGroupID"))]
    tax_group_id: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    fs::create_dir_all("data")?;

    for year in 2010..=2025 {
        let client = Client::new();
        fetch_year(&client, year).await?;
    }

    Ok(())
}

async fn fetch_year(client: &Client, tax_year: u32) -> Result<()> {
    const BASE_URL: &str = "https://swisstaxcalculator.estv.admin.ch/delegate/ost-integration/v1/lg-proxy/operation/c3b67379_ESTV";

    if let Err(e) = fetch_url::<Rates>(
        client,
        &format!("{BASE_URL}/API_exportManySimpleRates"),
        "rates",
        tax_year,
        99,
    )
    .await
    {
        eprintln!("Failed fetching rates for {tax_year}: {e:?}");
    }

    if let Err(e) = fetch_url::<Scales>(
        client,
        &format!("{BASE_URL}/API_exportManyTaxScales"),
        "scales",
        tax_year,
        88,
    )
    .await
    {
        eprintln!("Failed fetching scales for {tax_year}: {e:?}");
    }

    if let Err(e) = fetch_url::<Deductions>(
        client,
        &format!("{BASE_URL}/API_exportManyDeductions"),
        "deductions",
        tax_year,
        88,
    )
    .await
    {
        eprintln!("Failed fetching deductions for {tax_year}: {e:?}");
    }

    if let Err(e) = fetch_url::<OtherDeductions>(
        client,
        &format!("{BASE_URL}/API_exportManyDeductionScales"),
        "other-deductions",
        tax_year,
        88,
    )
    .await
    {
        eprintln!("Failed fetching other deductions for {tax_year}: {e:?}");
    }

    Ok(())
}

async fn fetch_url<T: for<'a> Deserialize<'a>>(
    client: &Client,
    url: &str,
    title: &str,
    tax_year: u32,
    tax_group_id: u32,
) -> Result<()> {
    let path = format!("data/{title}-{tax_year}.json");
    let mut file = File::create_new(&path)?;
    println!("Created new file: {path:?}");

    let res = client
        .post(url)
        .json(&Parameters {
            tax_year,
            tax_group_id,
        })
        .send()
        .await?;
    println!("Status: {:?}", res.status());

    let bytes = res.bytes().await?;
    println!("Received {} bytes", bytes.len());

    file.write_all(bytes.deref())?;

    println!("Validating data against schema");
    let _: T = serde_json::from_slice(bytes.deref())?;

    Ok(())
}
