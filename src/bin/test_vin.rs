use ascii::{AsAsciiStr, AsciiString};
use vin_extract::{DecodedVIN, VIN};

fn main() {
    let vins: Vec<VIN> = vec![
        "1J4FF68S5XL503754".into(), //wraps a string into a VIN struct
        "1J4FF68S5XL503754".into(),
        "5YJSA1E62NFO16329".into(),
        "WBA8B9G39HNU54468".into(),
        "SCCFE33C9VHF65358".into(),
    ];

    for vin in vins {
        //converts a VIN into a DecodedVIN struct using only extraction
        let decoded_vin: DecodedVIN = vin.into();

        //output the raw VIN debug form
        println!("{:?}", decoded_vin);

        //output a formatted version that generates user
        // friendly representation on the fly, using lookup facts
        println!("{}", decoded_vin);
    }
}
