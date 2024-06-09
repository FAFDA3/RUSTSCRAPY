use reqwest::{Error, header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT, REFERER, CONNECTION, UPGRADE_INSECURE_REQUESTS, ACCEPT_ENCODING}};
use scraper::{Html, Selector};
use tokio::time::{sleep, Duration};
use std::io::{self, Write};
use tokio;
use std::fs;
use std::path::Path;
use flate2::read::GzDecoder;
use std::io::Read;
use regex:: Regex;


fn create_airbnb_url(latitude1: f64, latitude2: f64, longitude1: f64, longitude2: f64) -> String {
    format!("https://it.airbnb.com/s/homes?refinement_paths%5B%5D=%2Fhomes&place_id=ChIJu46S-ZZhLxMROG5lkwZ3D7k&checkin=2024-07-19&checkout=2024-07-31&adults=1&tab_id=home_tab&query=Rome%2C+Italie&flexible_trip_lengths%5B%5D=one_week&monthly_start_date=2024-07-01&monthly_length=3&monthly_end_date=2024-10-01&search_mode=regular_search&price_filter_input_type=0&price_filter_num_nights=12&channel=EXPLORE&ne_lat={}&ne_lng={}&sw_lat={}&sw_lng={}&zoom=12.930721908719006&zoom_level=12.930721908719006&search_by_map=true&search_type=user_map_move", latitude1, longitude1, latitude2, longitude2)
}


async fn fetch_html(url: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"));
    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"));
    headers.insert(REFERER, HeaderValue::from_static("https://www.google.com"));
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br, zstd"));

    let response = client.get(url)
        .headers(headers.clone())
        .send()
        .await?;

    let final_url = response.url().clone();

    let response = client.get(final_url)
        .headers(headers)
        .send()
        .await?;

    let local_response = &response;
    //println!("{}", response);

    let status = response.status();
    let headers = response.headers().clone();
   // let body = response.text().await?;
    let body= response.bytes().await?;
  //  println!("Response Body {}",  body);
    println!("Response Status: {}", status);
    println!("Response Headers:\n{:#?}", headers);

   
    let mut gz = GzDecoder::new(&body[..]);
    let mut s = String::new();
    gz.read_to_string( &mut s);

    Ok(s)
}


fn extract_json (html: &str)->Option<String>{
    let re = Regex::new(r#"data-deferred-state-0="true" type="application/json">([^<]+)</script></body></html>"#).unwrap();
    re.captures(html).and_then(|cap| cap.get(1)).map(|m| m.as_str().to_string())



}

fn save_html(content: &str, folder: &str, filename: &str) -> std::io::Result<()> {
    let path = Path::new(folder);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    let filepath = path.join(filename);
    fs::write(filepath, content)?;
    Ok(())
}

fn extract_data(html: &str) -> String {
    html.to_string()
}

/*async fn run_scraper(lat1: f64, lat2: f64, long1: f64, long2: f64) {
    loop {
        let url = create_airbnb_url(lat1, lat2, long1, long2);
        println!("URL created: {}", url);
        match fetch_html(&url).await {
            Ok(html) => {
                let data = extract_data(&html);
                if let Err(e) = save_html(&data, "HTML", "test.html") {
                    eprintln!("Error saving HTML: {}", e);
                } else {
                    println!("HTML saved successfully.");
                }
            }
            Err(e) => eprintln!("Error fetching HTML: {}", e),
        }
        println!("scarper end");
        sleep(Duration::from_secs(1800)).await;
    }
}*/


async fn run_scraper(lat1: f64, lat2: f64, long1: f64, long2: f64) {
    loop {
        let url = create_airbnb_url(lat1, lat2, long1, long2);
        println!("URL created: {}", url);

    

        


        match fetch_html(&url).await {
            Ok(html) => {


                let data = extract_data(&html);
                if let Err(e) = save_html(&data, "HTML", "test20240608.html") {
                    eprintln!("Error saving HTML: {}", e);
                } else {
                    println!("HTML saved successfully.");
                }
            
              


                if let Some(json_content) = extract_json(&html){
                    if let Err(e) = save_html(&json_content, "HTML", "extracted_data.json"){
                        eprintln!("Error saving JSON: {}", e);
                        }else{
                            println!("JSON saved successfully.");
                        }
                }else {
                    println!("No JSON content found.");
                }



                /*let contextual_content = extract_contextual_content(&html);

                println!("this is the contextual {}", contextual_content);
                if let Err(e) = save_html(&contextual_content, "HTML", "contextual_content.html") {
                    eprintln!("Error saving HTML: {}", e);
                } else {
                    println!("HTML saved successfully.");
                }*/
            }
            Err(e) => eprintln!("Error fetching HTML: {}", e),
        }

        sleep(Duration::from_secs(1800)).await;
    }
}


fn main() {
    println!("Start the scraping");
    let lat1 = get_input("Enter latitude1: ");
    let long1 = get_input("Enter longitude1: ");
    let lat2 = get_input("Enter latitude2: ");
    let long2 = get_input("Enter longitude2: ");

    let lat1: f64 = lat1.trim().parse().expect("Invalid input for latitude1");
    let long1: f64 = long1.trim().parse().expect("Invalid input for longitude1");
    let lat2: f64 = lat2.trim().parse().expect("Invalid input for latitude2");
    let long2: f64 = long2.trim().parse().expect("Invalid input for longitude2");

    
    /*let lat1: f64 = 43.947613;
    let lat2: f64 = 43.8520324685;

    let long1: f64 = 12.5242224779;
    let long2: f64 = 12.412739371;*/




    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        run_scraper(lat1, lat2, long1, long2).await;
    });

    println!("finished");

}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}


fn extract_contextual_content(html: &str) -> String{
    let document = Html::parse_document(html);
    let selector = Selector::parse("[contextualPicturesPageInfo]").unwrap();
    document.select(&selector)
        .map(|element| element.html())
        .collect::<Vec<_>>()
        .join("\n")


}

/*
43.947613
43.8520324685

12.5242224779
12.412739371


*/