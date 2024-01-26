mod pokemon {

    struct PokemonProduct {
        url: Option<String>,
        image: Option<String>,
        name: Option<String>,
        price: Option<String>,
    }

    fn scrape_url(url: &str) -> String {
        // download the target HTML document
        let response = reqwest::blocking::get(url);
        // get the HTML content from the request response
        // and print it
        return response.unwrap().text().unwrap();
    }

    fn parse_document(html_content: &String) -> Vec<PokemonProduct> {
        // generate stuff
        let document = scraper::Html::parse_document(&html_content);

        let html_product_selector = scraper::Selector::parse("li.product").unwrap();

        let html_products = document.select(&html_product_selector);

        let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

        for html_product in html_products {
            // retrieve the info from the fields of interest
            // println!("{html_product}")
            let url = html_product
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(str::to_owned);

            let image = html_product
                .select(&scraper::Selector::parse("img").unwrap())
                .next()
                .and_then(|img| img.value().attr("src"))
                .map(str::to_owned);

            let name = html_product
                .select(&scraper::Selector::parse("h2").unwrap())
                .next()
                .map(|h2| h2.text().collect::<String>());

            let price = html_product
                .select(&scraper::Selector::parse(".price").unwrap())
                .next()
                .map(|price| price.text().collect::<String>());

            let pokemon_product = PokemonProduct {
                url,
                image,
                name,
                price,
            };
            pokemon_products.push(pokemon_product);
        }

        return pokemon_products;
    }

    fn serialize2csv(pokemon_products: Vec<PokemonProduct>, filename: &str) {
        let path = std::path::Path::new(filename);
        let mut writer = csv::Writer::from_path(path).unwrap();

        writer
            .write_record(&["url", "image", "name", "price"])
            .unwrap();

        for product in pokemon_products {
            let url = product.url.unwrap();
            let image = product.image.unwrap();
            let name = product.name.unwrap();
            let price = product.price.unwrap();
            writer.write_record(&[url, image, name, price]).unwrap();
        }

        writer.flush().unwrap();
    }
}
