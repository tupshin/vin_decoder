pub mod constants;

use pyo3::{pymodule,wrap_pyfunction,pyfunction, Python, PyResult};
use pyo3::types::PyModule;
use crate::constants::{country_from_range, manufacturer_from_code};
use crate::constants::YEAR_CHARS;
use ascii::AsAsciiStr;
use ascii::AsciiChar;
use ascii::AsciiString;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct VIN(pub AsciiString);

impl VIN {
    fn is_less_500(&self) -> bool {
        if self.0[2..3].as_ascii_str().unwrap().to_ascii_string() == "9" {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct DecodedVIN {
    source_vin: VIN,
    year: AsciiChar,
    serial_number: AsciiString,
    country_code: AsciiString,
    vds_code: AsciiString,
    wmi_code: AsciiString,
    plant_code: AsciiString,
    manufacturer_code: AsciiString,

}

impl Into<VIN> for &str {
    fn into(self) -> VIN {
        VIN (self.to_ascii_uppercase().parse().unwrap())
    }
}

impl Into<DecodedVIN> for VIN {
    fn into(self) -> DecodedVIN {
        let value = self.0.to_ascii_uppercase();
        let value = value.as_slice();

        DecodedVIN {
            source_vin: self.clone(),
            country_code: value[0..2].into(),
            wmi_code: value[0..3].into(),
            vds_code: value[3..8].into(),
            year: *value.get(9).unwrap(),
            plant_code: value[10..11].into(),
            manufacturer_code: if self.is_less_500() {
                value[11..13].into()
            } else {
                value[0..3].into()
            },
            serial_number: if self.is_less_500() {
                value[14..17].into()
            } else {
                value[12..17].into()
            },
        }
    }
}

//This is the only code that actually uses external data/lookup tables
#[allow(unused)]
impl Display for DecodedVIN {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "VIN: {}", self.source_vin.0);
        writeln!(f,"YEAR: {}", &YEAR_CHARS.chars().position(|r| &r == &self.year).unwrap() + 1980 + 30); //FIXME
        writeln!(f,"COUNTRY: {}",country_from_range(self.country_code.as_str()));
        writeln!(f,"MANUFACTURER: {}",manufacturer_from_code(self.manufacturer_code.as_str()));
        writeln!(f,"SERIAL: {}",self.serial_number);
        //writeln!(f,"PLANT: {}",self.plant_code);
        Ok(())
    }
}

#[pymodule]
fn vin_extract(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_vin, m)?)?;

    Ok(())
}

#[pyfunction]
fn render_vin(vin:&str) {
    let vin:VIN =  vin.into();
    let decoded_vin: DecodedVIN = vin.into();
    println!("{:?}", decoded_vin);
    println!("{}", decoded_vin);
}
