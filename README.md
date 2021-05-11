# Build Status

Using Travis CI to automatically build the service against Rust Nightly. 

[![Build Status](https://travis-ci.com/sean-halpin/betfair_stream_api_rust.svg?branch=master)](https://travis-ci.com/sean-halpin/betfair_stream_api_rust)

# Overview

This service is capable of recording Snapshots(`IMG`) and changes(`Deltas`) from the [Betfair Sports Trading Exchange](https://www.betfair.com/exchange/plus/).

Traditionally betting has occurred between a customer and a bookmaker where the customer 'backs' (bets that an outcome will occur) and the bookmaker 'lays' (bets that the outcome will not occur). Betting exchanges offer the opportunity for anyone to both back and lay. This is functionally similar to trading stocks. 

You might want to record the exchange in order to build algorithmic trading models or as a live feed of the current prices/odds for the market. 

## Technologies Used

- The Exchange Recorder is written in Rust.
  - The connection to the Betfair exchange is a TLS secured TCP Stream
  - We send a custom Auth msg & a market Subscription msg
  - The exchange then starts sending CRLF delimited JSON payloads back to us
  - These payloads contain snapshots `IMG` of the whole market & then Deltas from then on.
    - The deltas can be aggregated to form a material view of the current market
  - The service then places these into a Mongo DB Collection in order to record the feed. 
- Containerised with Docker. 
- Orchestrated with Docker-Compose.
- Storage with MongoDB.

## Create Exchange Keys & Export Environment Variables

Sign in at `https://www.betfair.com/exchange/plus/`
 - Navigate to a Horse Race or Soccer Event 
 - Click on a Market
   - Grab the market ID from the URL `https://www.betfair.com/exchange/plus/football/market/1.178839284` 
     - Export the market ID to your environment `export MARKET_ID=1.178839284`
  
Navigate to `https://docs.developer.betfair.com/visualisers/api-ng-account-operations/`
 - Grab your `SSOID` token from the UI
 - Use the API visualiser to `createDeveloperAppKeys`, then grab the `Application Key` using `getDeveloperAppKeys`
 - Export the `SSOID` and `APP_KEY` to your environment. 

## Run the Exchange Recorder Service Locally

Example;
```
export SSOID=XXXXXXXXXXXXXXXXXXXXXXX
export APP_KEY=XXXXXXXXXX
export MARKET_ID=1.178839712
cargo build
RUST_BACKTRACE=1 cargo run
```

## Run MongoDB & Mongo Express UI & Exchange Recorder


```
export SSOID=XXXXXXXXXXXXXXXXXXXXXXX
export APP_KEY=XXXXXXXXXX
export MARKET_ID=1.178839712
docker-compose build
docker-compose up 
```

## Prometheus Metrics

The app reveals metrics via `http://localhost:8000/metrics`, ready for prometheus scraping and display in grafana dashboards. 

## Mongo Express - Database UI

The `docker-compose.yaml` starts a mongodb instance as well as a Mongo Express UI, handy for debugging locally.
Available at `http://localhost:9081/`

## TODO

- Add Logger with configurable levels, WARN/INFO/EROR
- Add Tests
- Helmify for Kubernetes deployment
- Configure Mongo Replica Set for Redundancy & access to the `Change Stream`
- Forward the change stream events to Kafka as a messaging layer to enable multiple consumers.
