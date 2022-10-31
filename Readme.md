# dataport job crawler ⱒ
a crawler to parse a url f.g. 'https://karriere.dataport.de/stellenangebote.html' for a specific search pattern. crawler follows hrefs and notifies if search_pattern is found: depth 1 . 
# usage 
`./dataport_rust_crawler {search_pattern} url_1 url_2`
```shell
./dataport_rust_crawler "<strong>Bremen</strong>" "https://karriere.dataport.de/stellenangebote.html" "https://karriere.dataport.de/stellenangebote.html?start=100"
```