use ascii::AsciiString;
use std::ops::{Range, RangeInclusive};



pub static YEAR_CHARS: &str = "ABCDEFGHJKLMNPRSTVWXY1234567890";
pub static COUNTRY_CODES: [(RangeInclusive<&str>, &str); 110] = [
    ("AA"..="AH", "South Africa"),
    ("AJ"..="AN", "Ivory Coast"),
    ("AP"..="A0", "not assigned"),
    ("BA"..="BE", "Angola"),
    ("BF"..="BK", "Kenya"),
    ("BL"..="BR", "Tanzania"),
    ("BS"..="B0", "not assigned"),
    ("CA"..="CE", "Benin"),
    ("CF"..="CK", "Malagasy"),
    ("CL"..="CR", "Tunisia"),
    ("CS"..="C0", "not assigned"),
    ("DA"..="DE", "Egypt"),
    ("DF"..="DK", "Morocco"),
    ("DL"..="DR", "Zambia"),
    ("DS"..="D0", "not assigned"),
    ("EA"..="EE", "Ethiopia"),
    ("EF"..="EK", "Mozambique"),
    ("EL"..="E0", "not assigned"),
    ("FA"..="FE", "Ghana"),
    ("FF"..="FK", "Nigeria"),
    ("FF"..="FK", "Madagascar"),
    ("FL"..="F0", "not assigned"),
    ("GA"..="G0", "not assigned"),
    ("HA"..="H0", "not assigned"),
    ("JA"..="J0", "Japan"),
    ("KA"..="KE", "Sri Lanka"),
    ("KF"..="KK", "Israel"),
    ("KL"..="KR", "Korea (South)"),
    ("KS"..="K0", "not assigned"),
    ("LA"..="L0", "China"),
    ("MA"..="ME", "India"),
    ("MF"..="MK", "Indonesia"),
    ("ML"..="MR", "Thailand"),
    ("MS"..="M0", "not assigned"),
    ("NF"..="NK", "Pakistan"),
    ("NL"..="NR", "Turkey"),
    ("NS"..="N0", "not assigned"),
    ("PA"..="PE", "Philipines"),
    ("PF"..="PK", "Singapore"),
    ("PL"..="PR", "Malaysia"),
    ("PS"..="P0", "not assigned"),
    ("RA"..="RE", "United Arab Emirates"),
    ("RF"..="RK", "Taiwan"),
    ("RL"..="RR", "Vietnam"),
    ("RS"..="R0", "not assigned"),
    ("SA"..="SM", "Great Britain"),
    ("SN"..="ST", "Germany"),
    ("SU"..="SZ", "Poland"),
    ("S1"..="S0", "not assigned"),
    ("TA"..="TH", "Switzerland"),
    ("TJ"..="TP", "Czechoslovakia"),
    ("TR"..="TV", "Hungary"),
    ("TW"..="T1", "Portugal"),
    ("T2"..="T0", "not assigned"),
    ("UA"..="UG", "not assigned"),
    ("UH"..="UM", "Denmark"),
    ("UN"..="UT", "Ireland"),
    ("UU"..="UZ", "Romania"),
    ("U1"..="U4", "not assigned"),
    ("U5"..="U7", "Slovakia"),
    ("U8"..="U0", "not assigned"),
    ("VA"..="VE", "Austria"),
    ("VF"..="VR", "France"),
    ("VS"..="VW", "Spain"),
    ("VX"..="V2", "Yugoslavia"),
    ("V3"..="V5", "Croatia"),
    ("V6"..="V0", "Estonia"),
    ("WA"..="WZ", "Germany"),
    ("XA"..="XE", "Bulgaria"),
    ("XF"..="XK", "Greece"),
    ("XL"..="XR", "Netherlands"),
    ("XS"..="XW", "U.S.S.R."),
    ("XX"..="X2", "Luxembourg"),
    ("X3"..="X0", "Russia"),
    ("YA"..="YE", "Belgium"),
    ("YF"..="YK", "Finland"),
    ("YL"..="YR", "Malta"),
    ("YS"..="YW", "Sweden"),
    ("YX"..="Y2", "Norway"),
    ("Y3"..="Y5", "Belarus"),
    ("Y6"..="Y0", "Ukraine"),
    ("ZA"..="ZR", "Italy"),
    ("ZS"..="ZW", "not assigned"),
    ("ZX"..="Z2", "Slovenia"),
    ("Z3"..="Z5", "Lithuania"),
    ("Z6"..="Z0", "not assigned"),
    ("10"..="1Z", "United States"),
    ("2A"..="20", "Canada"),
    ("3A"..="3W", "Mexico"),
    ("3X"..="37", "Costa Rica"),
    ("38"..="30", "not assigned"),
    ("4A"..="40", "United States"),
    ("50"..="5Z", "United States"),
    ("6A"..="6W", "Australia"),
    ("6X"..="60", "not assigned"),
    ("7A"..="7E", "New Zealand"),
    ("7F"..="70", "not assigned"),
    ("8A"..="8E", "Argentina"),
    ("8F"..="8K", "Chile"),
    ("8L"..="8R", "Ecuador"),
    ("8S"..="8W", "Peru"),
    ("8X"..="82", "Venezuela"),
    ("83"..="80", "not assigned"),
    ("9A"..="9E", "Brazil"),
    ("9F"..="9K", "Colombia"),
    ("9L"..="9R", "Paraguay"),
    ("9S"..="9W", "Uruguay"),
    ("9X"..="92", "Trinidad & Tobago"),
    ("93"..="99", "Brazil"),
    ("90"..="90", "not assigned"),
];

pub fn country_from_range(value: &str) -> String {
    for rng in &COUNTRY_CODES {
        //println!("{} {:?}", value, rng);
        if rng.0.contains(&value) {
            return (rng.1.to_string());
        }
    }
    unimplemented!()
}

pub static MANUFACTURER_CODES: [(&str, &str); 538] = [
    ("AAV", "Volkswagen South Africa"),
    ("AC5", "Hyundai South Africa"),
    ("ADD", "Hyundai South Africa"),
    ("AFA", "Ford South Africa"),
    ("AHT", "Toyota South Africa"),
    ("JA3", "Mitsubishi"),
    ("JA4", "Mitsubishi"),
    ("JA", "Isuzu"),
    ("JD", "Daihatsu"),
    ("JF", "Fuji Heavy Industries (Subaru)"),
    ("JHA", "Hino"),
    ("JHB", "Hino"),
    ("JHC", "Hino"),
    ("JHD", "Hino"),
    ("JHE", "Hino"),
    ("JHF", "Honda"),
    ("JHG", "Honda"),
    ("JHL", "Honda"),
    ("JHM", "Honda"),
    ("JHN", "Honda"),
    ("JHZ", "Honda"),
    ("JH1", "Honda"),
    ("JH2", "Honda"),
    ("JH3", "Honda"),
    ("JH4", "Honda"),
    ("JH5", "Honda"),
    ("JK", "Kawasaki (motorcycles)"),
    ("JL5", "Mitsubishi Fuso"),
    ("JM1", "Mazda"),
    ("JMB", "Mitsubishi Motors"),
    ("JMY", "Mitsubishi Motors"),
    ("JMZ", "Mazda"),
    ("JN", "Nissan"),
    ("JS", "Suzuki"),
    ("JT", "Toyota"),
    ("JY", "Yamaha (motorcycles)"),
    ("KL", "Daewoo General Motors South Korea"),
    ("KM", "Hyundai"),
    ("KMY", "Daelim (motorcycles)"),
    ("KM1", "Hyosung (motorcycles)"),
    ("KN", "Kia"),
    ("KNM", "Renault Samsung"),
    ("KPA", "SsangYong"),
    ("KPT", "SsangYong"),
    ("LAE", "Jinan Qingqi Motorcycle"),
    ("LAL", "Sundiro Honda Motorcycle"),
    ("LAN", "Changzhou Yamasaki Motorcycle"),
    ("LBB", "Zhejiang Qianjiang Motorcycle (Keeway/Generic)"),
    ("LBE", "Beijing Hyundai"),
    ("LBM", "Zongshen Piaggio"),
    ("LBP", "Chongqing Jainshe Yamaha (motorcycles)"),
    ("LB2", "Geely Motorcycles"),
    ("LCE", "Hangzhou Chunfeng Motorcycles (CFMOTO)"),
    ("LDC", "Dong Feng Peugeot Citroen (DPCA), China"),
    ("LDD", "Dandong Huanghai Automobile"),
    ("LDF", "Dezhou Fulu Vehicle (motorcycles)"),
    ("LDN", "SouEast Motor"),
    ("LDY", "Zhongtong Coach, China"),
    ("LET", "Jiangling-Isuzu Motors, China"),
    ("LE4", "Beijing Benz, China"),
    ("LFB", "FAW, China (busses)"),
    ("LFG", "Taizhou Chuanl Motorcycle Manufacturing"),
    ("LFP", "FAW, China (passenger vehicles)"),
    ("LFT", "FAW, China (trailers)"),
    ("LFV", "FAW-Volkswagen, China"),
    ("LFW", "FAW JieFang, China"),
    ("LFY", "Changshu Light Motorcycle Factory"),
    ("LGB", "Dong Feng (DFM), China"),
    ("LGH", "Qoros (formerly Dong Feng (DFM)), China"),
    ("LGX", "BYD Auto, China"),
    ("LHB", "Beijing Automotive Industry Holding"),
    ("LH1", "FAW-Haima, China"),
    ("LJC", "JAC, China"),
    ("LJ1", "JAC, China"),
    ("LKL", "Suzhou King Long, China"),
    ("LL6", "Hunan Changfeng Manufacture Joint-Stock"),
    ("LL8", "Linhai (ATV)"),
    ("LMC", "Suzuki Hong Kong (motorcycles)"),
    ("LPR", "Yamaha Hong Kong (motorcycles)"),
    ("LPS", "Polestar (Volvo) (Sweden)"),
    ("LRW", "Tesla, Inc. (Gigafactory Shanghai)"),
    ("LSG", "Shanghai General Motors, China"),
    ("LSJ", "MG Motor UK Limited - SAIC Motor, Shanghai, China"),
    ("LSV", "Shanghai Volkswagen, China"),
    ("LSY", "Brilliance Zhonghua"),
    ("LTP", "National Electric Vehicle Sweden AB (NEVS)"),
    ("LTV", "Toyota Tian Jin"),
    ("LUC", "Guangqi Honda, China"),
    ("LVS", "Ford Chang An"),
    ("LVV", "Chery, China"),
    ("LVZ", "Dong Feng Sokon Motor Company (DFSK)"),
    ("LV3", "National Electric Vehicle Sweden AB (NEVS)"),
    ("LZM", "MAN China"),
    ("LZE", "Isuzu Guangzhou, China"),
    ("LZG", "Shaanxi Automobile Group, China"),
    ("LZP", "Zhongshan Guochi Motorcycle (Baotian)"),
    ("LZY", "Yutong Zhengzhou, China"),
    ("LZZ", "Chongqing Shuangzing Mech & Elec (Howo)"),
    ("L4B", "Xingyue Group (motorcycles)"),
    ("L5C", "KangDi (ATV)"),
    ("L5K", "Zhejiang Yongkang Easy Vehicle"),
    ("L5N", "Zhejiang Taotao, China (ATV & motorcycles)"),
    ("L5Y", "Merato Motorcycle Taizhou Zhongneng"),
    ("L85", "Zhejiang Yongkang Huabao Electric Appliance"),
    ("L8X", "Zhejiang Summit Huawin Motorcycle"),
    ("MAB", "Mahindra & Mahindra"),
    ("MAC", "Mahindra & Mahindra"),
    ("MAJ", "Ford India"),
    ("MAK", "Honda Siel Cars India"),
    ("MAL", "Hyundai India"),
    ("MAT", "Tata Motors"),
    ("MA1", "Mahindra & Mahindra"),
    ("MA3", "Suzuki India (Maruti)"),
    ("MA6", "GM India"),
    ("MA7", "Mitsubishi India (formerly Honda)"),
    ("MB8", "Suzuki India Motorcycles"),
    ("MBH", "Suzuki India (Maruti)"),
    ("MBJ", "Toyota India"),
    ("MBR", "Mercedes-Benz India"),
    ("MB1", "Ashok Leyland"),
    ("MCA", "Fiat India"),
    ("MCB", "GM India"),
    ("MC2", "Volvo Eicher commercial vehicles limited."),
    ("MDH", "Nissan India"),
    ("MD2", "Bajaj Auto"),
    ("MD9", "Shuttle Cars India"),
    ("MEC", "Daimler India Commercial Vehicles"),
    ("MEE", "Renault India"),
    ("MEX", "Volkswagen India"),
    ("MHF", "Toyota Indonesia"),
    ("MHR", "Honda Indonesia"),
    ("MLC", "Suzuki Thailand"),
    ("NAA", "Iran Khodro (Peugeot Iran)"),
    ("NAP", "Pars Khodro"),
    ("MLH", "Honda Thailand"),
    ("MMA", "Mitsubishi Thailand"),
    ("MMB", "Mitsubishi Thailand"),
    ("MMC", "Mitsubishi Thailand"),
    ("MMM", "Chevrolet Thailand"),
    ("MMS", "Suzuki Thailand"),
    ("MMT", "Mitsubishi Thailand"),
    ("MMU", "Holden Thailand"),
    ("MM8", "Mazda Thailand"),
    ("MNB", "Ford Thailand"),
    ("MNT", "Nissan Thailand"),
    ("MPA", "Isuzu Thailand"),
    ("MP1", "Isuzu Thailand"),
    ("MRH", "Honda Thailand"),
    ("MR0", "Toyota Thailand"),
    ("MS0", "SSS MOTORS Myanmar"),
    ("MS3", "Suzuki Myanmar Motor Co.,Ltd."),
    ("NLA", "Honda Türkiye"),
    ("NLE", "Mercedes-Benz Türk Truck"),
    ("NLH", "Hyundai Assan"),
    ("NLN", "Karsan"),
    ("NLR", "OTOKAR"),
    ("NLT", "TEMSA"),
    ("NMB", "Mercedes-Benz Türk Buses"),
    ("NMC", "BMC"),
    ("NM0", "Ford Turkey"),
    ("NM4", "Tofaş Türk"),
    ("NMT", "Toyota Türkiye"),
    ("NNA", "Isuzu Turkey"),
    ("PE1", "Ford Philippines"),
    ("PE3", "Mazda Philippines"),
    ("PL1", "Proton, Malaysia"),
    ("PNA", "NAZA, Malaysia (Peugeot)"),
    ("R2P", "Evoke Electric Motorcycles HK"),
    ("RA1", "Steyr Trucks International FZE, UAE"),
    ("RFB", "Kymco, Taiwan"),
    ("RFG", "Sanyang SYM, Taiwan"),
    ("RFL", "Adly, Taiwan"),
    ("RFT", "CPI, Taiwan"),
    ("RF3", "Aeon Motor, Taiwan"),
    ("SAB", "Optare"),
    ("SAD", "Jaguar (F-Pace, I-Pace)"),
    ("SAL", "Land Rover"),
    ("SAJ", "Jaguar"),
    ("SAR", "Rover"),
    ("SAX", "Austin-Rover"),
    ("SA9", "OX Global"),
    ("SB1", "Toyota UK"),
    ("SBM", "McLaren"),
    ("SCA", "Rolls Royce"),
    ("SCB", "Bentley"),
    ("SCC", "Lotus Cars"),
    ("SCE", "DeLorean Motor Cars N. Ireland (UK)"),
    ("SCF", "Aston"),
    ("SCK", "iFor Williams"),
    ("SDB", "Peugeot UK (formerly Talbot)"),
    ("SED", "General Motors Luton Plant"),
    ("SEY", "LDV"),
    ("SFA", "Ford UK"),
    ("SFD", "Alexander Dennis UK"),
    ("SHH", "Honda UK"),
    ("SHS", "Honda UK"),
    ("SJN", "Nissan UK"),
    ("SKF", "Vauxhall"),
    ("SLP", "JCB Research UK"),
    ("SMT", "Triumph Motorcycles"),
    ("SUF", "Fiat Auto Poland"),
    ("SUL", "FSC (Poland)"),
    ("SUP", "FSO-Daewoo (Poland)"),
    ("SU9", "Solaris Bus & Coach (Poland)"),
    ("SUU", "Solaris Bus & Coach (Poland)"),
    ("SWV", "TA-NO (Poland)"),
    ("TCC", "Micro Compact Car AG (smart 1998-1999)"),
    ("TDM", "QUANTYA Swiss Electric Movement (Switzerland)"),
    ("TK9", "SOR buses (Czech Republic)"),
    ("TMA", "Hyundai Motor Manufacturing Czech"),
    ("TMB", "Škoda (Czech Republic)"),
    ("TMK", "Karosa (Czech Republic)"),
    ("TMP", "Škoda trolleybuses (Czech Republic)"),
    ("TMT", "Tatra (Czech Republic)"),
    ("TM9", "Škoda trolleybuses (Czech Republic)"),
    ("TNE", "TAZ"),
    ("TN9", "Karosa (Czech Republic)"),
    ("TRA", "Ikarus Bus"),
    ("TRU", "Audi Hungary"),
    ("TSB", "Ikarus Bus"),
    ("TSE", "Ikarus Egyedi Autobuszgyar, (Hungary)"),
    ("TSM", "Suzuki Hungary"),
    ("TW1", "Toyota Caetano Portugal"),
    ("TYA", "Mitsubishi Trucks Portugal"),
    ("TYB", "Mitsubishi Trucks Portugal"),
    ("UU1", "Renault Dacia, (Romania)"),
    ("UU2", "Oltcit"),
    ("UU3", "ARO"),
    ("UU4", "Roman SA"),
    ("UU5", "Rocar"),
    ("UU6", "Daewoo Romania"),
    ("UU7", "Euro Bus Diamond"),
    ("UU9", "Astra Bus"),
    ("UV9", "ATP Bus"),
    ("UZT", "UTB (Uzina de Tractoare Brașov)"),
    ("U5Y", "Kia Motors Slovakia"),
    ("U6Y", "Kia Motors Slovakia"),
    ("VAG", "Magna Steyr Puch"),
    ("VAN", "MAN Austria"),
    ("VBK", "KTM (Motorcycles)"),
    ("VF1", "Renault"),
    ("VF2", "Renault"),
    ("VF3", "Peugeot"),
    ("VF4", "Talbot"),
    ("VF6", "Renault (Trucks & Buses)"),
    ("VF7", "Citroën"),
    ("VF8", "Matra"),
    ("VF9", "Bugatti"),
    ("VG5", "MBK (motorcycles)"),
    ("VLU", "Scania France"),
    ("VN1", "SOVAB (France)"),
    ("VNE", "Irisbus (France)"),
    ("VNK", "Toyota France"),
    ("VNV", "Renault-Nissan"),
    ("VSA", "Mercedes-Benz Spain"),
    ("VSE", "Suzuki Spain (Santana Motors)"),
    ("VSK", "Nissan Spain"),
    ("VSS", "SEAT"),
    ("VSX", "Opel Spain"),
    ("VS6", "Ford Spain"),
    ("VS7", "Citroën Spain"),
    ("VS9", "Carrocerias Ayats (Spain)"),
    ("VTH", "Derbi (motorcycles)"),
    ("VTL", "Yamaha Spain (motorcycles)"),
    ("VTT", "Suzuki Spain (motorcycles)"),
    ("VV9", "TAURO Spain"),
    ("VWA", "Nissan Spain"),
    ("VWV", "Volkswagen Spain"),
    ("VX1", "Zastava / Yugo Serbia"),
    ("WAG", "Neoplan"),
    ("WAU", "Audi"),
    ("WA1", "Audi SUV"),
    ("WBA", "BMW"),
    ("WBS", "BMW M"),
    ("WBW", "BMW"),
    ("WBY", "BMW"),
    ("WB1", "BMW Motorrad of North America"),
    ("WDA", "Daimler"),
    ("WDB", "Mercedes-Benz"),
    ("WDC", "DaimlerChrysler"),
    ("WDD", "Mercedes-Benz"),
    ("WDF", "Mercedes-Benz (commercial vehicles)"),
    ("WEB", "Evobus GmbH (Mercedes-Bus)"),
    ("WJM", "Iveco Magirus"),
    ("WF0", "Ford Germany"),
    ("WKE", "Fahrzeugwerk Bernard Krone (truck trailers)"),
    ("WKK", "Kässbohrer/Setra"),
    ("WMA", "MAN Germany"),
    ("WME", "smart"),
    ("WMW", "MINI"),
    ("WMX", "Mercedes-AMG"),
    ("WMZ", "MINI"),
    ("WP0", "Porsche"),
    ("WP1", "Porsche SUV"),
    ("WSM", "Schmitz-Cargobull (truck trailers)"),
    ("W09", "RUF"),
    ("W0L", "Opel"),
    ("W0V", "Opel (since 2017)"),
    ("WUA", "Audi Sport GmbH (formerly quattro GmbH)"),
    ("WVG", "Volkswagen MPV/SUV"),
    ("WVW", "Volkswagen"),
    ("WV1", "Volkswagen Commercial Vehicles"),
    ("WV2", "Volkswagen Bus/Van"),
    ("WV3", "Volkswagen Trucks"),
    ("XLB", "Volvo (NedCar)"),
    ("XLE", "Scania Netherlands"),
    ("XLR", "DAF (trucks)"),
    ("XL4", "Lightyear"),
    ("XL9", "Spyker"),
    ("XMC", "Mitsubishi (NedCar)"),
    ("XMG", "VDL Bus & Coach"),
    ("XTA", "Lada/AvtoVAZ (Russia)"),
    ("XTC", "KAMAZ (Russia)"),
    ("XTH", "GAZ (Russia)"),
    ("XTT", "UAZ/Sollers (Russia)"),
    ("XTU", "Trolza (Russia)"),
    ("XTY", "LiAZ (Russia)"),
    ("XUF", "General Motors Russia"),
    ("XUU", "AvtoTor (Russia, General Motors SKD)"),
    ("XW8", "Volkswagen Group Russia"),
    ("XWB", "UZ-Daewoo (Uzbekistan)"),
    ("XWE", "AvtoTor (Russia, Hyundai-Kia SKD)"),
    ("X1M", "PAZ (Russia)"),
    ("X4X", "AvtoTor (Russia, BMW SKD)"),
    ("X7L", "Renault AvtoFramos (Russia)"),
    ("X7M", "Hyundai TagAZ (Russia)"),
    ("YBW", "Volkswagen Belgium"),
    ("YB1", "Volvo Trucks Belgium"),
    ("YCM", "Mazda Belgium"),
    ("YE2", "Van Hool (buses)"),
    ("YH2", "BRP Finland (Lynx snowmobiles)"),
    ("YK1", "Saab-Valmet Finland"),
    ("YSC", "Cadillac (Saab)"),
    ("YS2", "Scania AB"),
    ("YS3", "Saab"),
    ("YS4", "Scania Bus"),
    ("YTN", "Saab NEVS"),
    // YT9/("007", "Koenigsegg"),
    // YT9/("034", "Carvia"),
    ("YU7", "Husaberg (motorcycles)"),
    ("YVV", "Polestar (Volvo) (Sweden)"),
    ("YV1", "Volvo Cars"),
    ("YV4", "Volvo Cars"),
    ("YV2", "Volvo Trucks"),
    ("YV3", "Volvo Buses"),
    ("Y3M", "MAZ (Belarus)"),
    ("Y6D", "Zaporozhets/AvtoZAZ (Ukraine)"),
    ("ZAA", "Autobianchi"),
    ("ZAM", "Maserati"),
    ("ZAP", "Piaggio/Vespa/Gilera"),
    ("ZAR", "Alfa Romeo"),
    ("ZBN", "Benelli"),
    ("ZCG", "Cagiva SpA / MV Agusta"),
    ("ZCF", "Iveco"),
    ("ZDC", "Honda Italia Industriale SpA"),
    ("ZDM", "Ducati Motor Holdings SpA"),
    ("ZDF", "Ferrari Dino"),
    ("ZD0", "Yamaha Italy"),
    ("ZD3", "Beta Motor"),
    ("ZD4", "Aprilia"),
    ("ZFA", "Fiat"),
    ("ZFC", "Fiat V.I."),
    ("ZFF", "Ferrari"),
    ("ZGU", "Moto Guzzi"),
    ("ZHW", "Lamborghini"),
    ("ZJM", "Malaguti"),
    ("ZJN", "Innocenti"),
    ("ZKH", "Husqvarna Motorcycles Italy"),
    ("ZLA", "Lancia"),
    ("Z8M", "Marussia (Russia)"),
    ("1B3", "Dodge"),
    ("1C3", "Chrysler"),
    ("1C4", "Chrysler"),
    ("1C6", "Chrysler"),
    ("1D3", "Dodge"),
    ("1FA", "Ford Motor Company"),
    ("1FB", "Ford Motor Company"),
    ("1FC", "Ford Motor Company"),
    ("1FD", "Ford Motor Company"),
    ("1FM", "Ford Motor Company"),
    ("1FT", "Ford Motor Company"),
    ("1FU", "Freightliner"),
    ("1FV", "Freightliner"),
    ("1F9", "FWD Corp."),
    ("1G", "General Motors USA"),
    ("1GC", "Chevrolet Truck USA"),
    ("1GT", "GMC Truck USA"),
    ("1G1", "Chevrolet USA"),
    ("1G2", "Pontiac USA"),
    ("1G3", "Oldsmobile USA"),
    ("1G4", "Buick USA"),
    ("1G6", "Cadillac USA"),
    ("1G8", "Saturn USA"),
    ("1GM", "Pontiac USA"),
    ("1GN", "Chevrolet SUV USA"),
    ("1GY", "Cadillac USA"),
    ("1H", "Honda USA"),
    ("1HD", "Harley-Davidson"),
    ("1HT", "International Truck and Engine Corp. USA"),
    ("1J4", "Jeep"),
    ("1J8", "Jeep"),
    ("1L", "Lincoln USA"),
    ("1ME", "Mercury USA"),
    ("1M1", "Mack Truck USA"),
    ("1M2", "Mack Truck USA"),
    ("1M3", "Mack Truck USA"),
    ("1M4", "Mack Truck USA"),
    ("1M9", "Mynatt Truck & Equipment"),
    ("1N", "Nissan USA"),
    ("1NX", "NUMMI USA"),
    ("1P3", "Plymouth USA"),
    ("1PY", "John Deere USA"),
    ("1R9", "Roadrunner Hay Squeeze USA"),
    ("1VW", "Volkswagen USA"),
    ("1XK", "Kenworth USA"),
    ("1XP", "Peterbilt USA"),
    ("1YV", "Mazda USA (AutoAlliance International)"),
    ("1ZV", "Ford (AutoAlliance International)"),
    ("2A4", "Chrysler Canada"),
    ("2BP", "Bombardier Recreational Products"),
    ("2B3", "Dodge Canada"),
    ("2B7", "Dodge Canada"),
    ("2C3", "Chrysler Canada"),
    ("2CN", "CAMI"),
    ("2D3", "Dodge Canada"),
    ("2FA", "Ford Motor Company Canada"),
    ("2FB", "Ford Motor Company Canada"),
    ("2FC", "Ford Motor Company Canada"),
    ("2FM", "Ford Motor Company Canada"),
    ("2FT", "Ford Motor Company Canada"),
    ("2FU", "Freightliner"),
    ("2FV", "Freightliner"),
    ("2FZ", "Sterling"),
    ("2Gx", "General Motors Canada"),
    ("2G1", "Chevrolet Canada"),
    ("2G2", "Pontiac Canada"),
    ("2G3", "Oldsmobile Canada"),
    ("2G4", "Buick Canada"),
    ("2G9", "mfr. of less than 1000/ yr. Canada"),
    ("2HG", "Honda Canada"),
    ("2HK", "Honda Canada"),
    ("2HJ", "Honda Canada"),
    ("2HM", "Hyundai Canada"),
    ("2M", "Mercury"),
    ("2NV", "Nova Bus Canada"),
    ("2P3", "Plymouth Canada"),
    ("2T", "Toyota Canada"),
    ("2TP", "Triple E Canada LTD"),
    ("2V4", "Volkswagen Canada"),
    ("2V8", "Volkswagen Canada"),
    ("2WK", "Western Star"),
    ("2WL", "Western Star"),
    ("2WM", "Western Star"),
    ("363", "Spyker"),
    ("3C4", "Chrysler Mexico"),
    ("3C6", "RAM Mexico"),
    ("3D3", "Dodge Mexico"),
    ("3D4", "Dodge Mexico"),
    ("3FA", "Ford Motor Company Mexico"),
    ("3FE", "Ford Motor Company Mexico"),
    ("3G", "General Motors Mexico"),
    ("3H", "Honda Mexico"),
    ("3JB", "BRP Mexico (all-terrain vehicles)"),
    ("3MD", "Mazda Mexico"),
    ("3MZ", "Mazda Mexico"),
    ("3N", "Nissan Mexico"),
    ("3NS", "Polaris Industries USA"),
    ("3NE", "Polaris Industries USA"),
    ("3P3", "Plymouth Mexico"),
    ("3VW", "Volkswagen Mexico"),
    ("46J", "Federal Motors Inc. USA"),
    ("4EN", "Emergency One USA"),
    ("4F", "Mazda USA"),
    ("4JG", "Mercedes-Benz USA"),
    ("4M", "Mercury"),
    ("4P1", "Pierce Manufacturing Inc. USA"),
    ("4RK", "Nova Bus USA"),
    ("4S", "Subaru-Isuzu Automotive"),
    ("4T", "Toyota"),
    ("4T9", "Lumen Motors"),
    ("4UF", "Arctic Cat Inc."),
    ("4US", "BMW USA"),
    ("4UZ", "Frt-Thomas Bus"),
    ("4V1", "Volvo"),
    ("4V2", "Volvo"),
    ("4V3", "Volvo"),
    ("4V4", "Volvo"),
    ("4V5", "Volvo"),
    ("4V6", "Volvo"),
    ("4VL", "Volvo"),
    ("4VM", "Volvo"),
    ("4VZ", "Volvo"),
    ("538", "Zero Motorcycles (USA)"),
    ("5F", "Honda USA-Alabama"),
    ("5J", "Honda USA-Ohio"),
    ("5L", "Lincoln"),
    ("5N1", "Nissan USA"),
    ("5NP", "Hyundai USA"),
    ("5T", "Toyota USA - trucks"),
    ("5YJ", "Tesla, Inc."),
    ("56K", "Indian Motorcycle USA"),
    ("6AB", "MAN Australia"),
    ("6F4", "Nissan Motor Company Australia"),
    ("6F5", "Kenworth Australia"),
    ("6FP", "Ford Motor Company Australia"),
    ("6G1", "General Motors-Holden (post Nov 2002)"),
    ("6G2", "Pontiac Australia (GTO & G8)"),
    ("6H8", "General Motors-Holden (pre Nov 2002)"),
    ("6MM", "Mitsubishi Motors Australia"),
    ("6T1", "Toyota Motor Corporation Australia"),
    ("6U9", "Privately Imported car in Australia"),
    ("795", "Bugatti"),
    ("8AD", "Peugeot Argentina"),
    ("8AF", "Ford Motor Company Argentina"),
    ("8AG", "Chevrolet Argentina"),
    ("8AJ", "Toyota Argentina"),
    ("8AK", "Suzuki Argentina"),
    ("8AP", "Fiat Argentina"),
    ("8AW", "Volkswagen Argentina"),
    ("8A1", "Renault Argentina"),
    ("8GD", "Peugeot Chile"),
    ("8GG", "Chevrolet Chile"),
    ("8LD", "Chevrolet Ecuador"),
    ("935", "Citroën Brazil"),
    ("936", "Peugeot Brazil"),
    ("93H", "Honda Brazil"),
    ("93R", "Toyota Brazil"),
    ("93U", "Audi Brazil"),
    ("93V", "Audi Brazil"),
    ("93X", "Mitsubishi Motors Brazil"),
    ("93Y", "Renault Brazil"),
    ("94D", "Nissan Brazil"),
    ("9BF", "Ford Motor Company Brazil"),
    ("9BG", "Chevrolet Brazil"),
    ("9BM", "Mercedes-Benz Brazil"),
    ("9BR", "Toyota Brazil"),
    ("9BS", "Scania Brazil"),
    ("9BW", "Volkswagen Brazil"),
    ("9FB", "Renault Colombia"),
    ("9GA", "Chevrolet Colombia "),
];

pub fn manufacturer_from_code(value: &str) -> String {
        let code = 94;
        for code in &MANUFACTURER_CODES {
                if code.0 == value  {
                        return (code.1.to_string());
                }
        }
        unimplemented!()
}
