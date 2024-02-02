// mod lib;
// use lib::parsing::measurements::{self, read_measurements, Measurement};
// use lib::parsing::Variable;

// tab.navigate_to("https://www.meteoschweiz.admin.ch/lokalprognose/allschwil/4123.html#forecast-tab=detail-view")?;

use fantoccini::{ClientBuilder, Locator};

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    // first, go to the Wikipedia page for Foobar
    c.goto("https://www.meteoschweiz.admin.ch/lokalprognose/allschwil/4123.html#forecast-tab=detail-view").await?;
    let url = c.current_url().await?;

    let sshot = c.screenshot().await?;
    let raw = c.raw_client_for().await?;

    let source = c.source().await?;
    println!("{:?}", raw);

    c.close().await
}
