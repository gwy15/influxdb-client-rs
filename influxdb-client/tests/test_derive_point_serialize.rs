
use influxdb_client::{Timestamp, PointSerialize};
use influxdb_client::derives::PointSerialize;


#[test]
fn test_derive_serialize() {
    #[derive(PointSerialize)]
    #[point(measurement = "test")]
    struct Test {
        #[point(tag = "notTicker")]
        ticker: String,
        #[point(tag = "notTicker2")]
        ticker2: String,
        #[point(field = "notPrice")]
        price: f32,
        #[point(field)]
        price2: String,
        #[point(timestamp)]
        data: Timestamp,
    }

    let result = Test {
        ticker: "GME".to_string(),
        ticker2: "!GME".to_string(),
        price: 0.32,
        price2: "Hello world".to_string(),
        data: Timestamp::from("321321321"),
    }
    .serialize();

    assert_eq!(
        "test,notTicker=GME,notTicker2=!GME notPrice=0.32,price2=\"Hello world\"".to_string(),
        result
    );
}

#[test]
fn test_derive_serialize_with_timestamp() {
    #[derive(PointSerialize)]
    #[point(measurement = "test")]
    struct Test {
        #[point(tag = "notTicker")]
        ticker: String,
        #[point(tag = "notTicker2")]
        ticker2: String,
        #[point(field = "notPrice")]
        price: f32,
        #[point(field)]
        price2: String,
        #[point(timestamp)]
        data: Timestamp,
    }

    let data = Test {
        ticker: "GME".to_string(),
        ticker2: "!GME".to_string(),
        price: 0.32,
        price2: "Hello world".to_string(),
        data: Timestamp::from("321321321"),
    };
    let result = data.serialize_with_timestamp(None);
    assert_eq!(
        "test,notTicker=GME,notTicker2=!GME notPrice=0.32,price2=\"Hello world\" 321321321"
            .to_string(),
        result
    );

    let result_2 = data.serialize_with_timestamp(Some(Timestamp::from(420)));
    assert_eq!(
        "test,notTicker=GME,notTicker2=!GME notPrice=0.32,price2=\"Hello world\" 420"
            .to_string(),
        result_2
    );
}