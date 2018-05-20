extern crate protobuf;
extern crate grpc;
extern crate tls_api;

use shorty_grpc::ShortyService;

fn main() {

    let mut count: u32 = 0;

    loop {
        count += 1;

        let mut shorten_request = shorty::ShortenRequest::new();
        let mut campaign = shorty::GoogleAnalyticsCampaign::new();

        campaign.set_utm_campaign("test-campaign".parse().unwrap());

        shorten_request.set_url("http://google.com".parse().unwrap());
        shorten_request.set_campaign(campaign);

        println!("{:#?}", shorten_request);

        let client_conf = grpc::ClientConf::new();

        // try create a client
        let client = grpc::Client::new_plain(
            "localhost",
            8080,
            client_conf
        ).unwrap();

        let shorty_client = &shorty_grpc::ShortyServiceClient::with_client(client.clone());

        let resp = shorty_client.shorten(grpc::RequestOptions::new(), shorten_request);

        let contents = resp.wait();

        match contents {
            Err(_) => {
                println!("Error with request {}", count);
                continue;
            },
            Ok(c) => {
                println!("{:?}", c.1);
            },
        }

        if count == 1000 {
            println!("Done looping");

            break;
        }
    }


}
mod shorty;

mod shorty_grpc;

//use shorty_grpc;
