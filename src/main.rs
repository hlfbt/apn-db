#![allow(unused_parens)]
//#![feature(alloc_system)]

#[macro_use]
extern crate clap;

mod cache;
mod apndb;
mod simtoolkit;

use crate::apndb::Apn;

const APN_FILE_URL: &str = "https://android.googlesource.com/device/sample/+/master/etc/apns-full-conf.xml?format=TEXT";
const DB_CACHE_FILE: &str = "~/.cache/apn-db.xml";

fn main() {
    match run() {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(1)
    };
}

fn run() -> Result<(), ()> {
    let matches = clap_app!(apndb =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Fetch and query a database of Mobile Network Access Point Names")
        (@arg mcc: --mcc +takes_value +required value_name("MCC") conflicts_with("imsi") requires("mnc")
            "Mobile Carrier Code to query (3 digits)\nRequires MNC\nMust not be specified when querying by IMSI")
        (@arg mnc: --mnc +takes_value +required value_name("MNC") conflicts_with("imsi") requires("mcc")
            "Mobile Network Code to query (2-3 digits)\nRequires MCC\nMust not be specified when querying by IMSI")
        (@arg imsi: --imsi +takes_value +required value_name("IMSI") conflicts_with_all(&["mcc", "mnc"])
            "International Mobile Subscriber Identity to query (15 digits)\nMust not be specified when querying by MCC and MNC")
        (@arg cache: -c --cache value_name("FILE") +takes_value !required default_value(DB_CACHE_FILE)
            "Cache file location")
        (@arg update: -u --update
            "Force update cache. By default, the cache file is only updated when not present.")
        (@group print =>
            (@arg all: --all
                "Print all matching APNs")
            (@arg lte: --lte
                "Print the APN best matching LTE requirements")
            (@arg default: --default
                "Print the best matching default APN (the APN most likely to be working to connect to the internet)")
        )
    ).get_matches();

    let db_cache_file: &str = matches.value_of("cache").unwrap_or(DB_CACHE_FILE);
    let force_update: bool = matches.is_present("update");

    let cache: cache::ApnFile = cache::ApnFile::new(db_cache_file, APN_FILE_URL);
    let mut db: apndb::ApnDb = apndb::ApnDb::new();

    db.parse(&cache.read(force_update)[..]);

    let mcc: &str;
    let mnc: &str;
    if (matches.is_present("imsi")) {
        let imsi = simtoolkit::IccHelper::parse_imsi(matches.value_of("imsi").unwrap());
        println!("IMSI:       {} {} {}", imsi.0, imsi.1, imsi.2);
        mcc = imsi.0;
        mnc = imsi.1;
    } else {
        mcc = matches.value_of("mcc").unwrap();
        mnc = matches.value_of("mnc").unwrap();
    }

    if (matches.is_present("all")) {
        return print_apns(db.query(mcc, mnc));
    } else if (matches.is_present("lte")) {
        match db.query_lte(mcc, mnc) {
            Some(apn) => {
                print_apn(&apn);
                return Ok(());
            },
            None => return Err(())
        };
    }

    match db.query_default(mcc, mnc) {
        Some(apn) => {
            print_apn(&apn);
            return Ok(());
        },
        None => return Err(())
    };
}

fn print_apns(apns: Option<&Vec<Apn>>) -> Result<(), ()> {
    match apns {
        Some(apns) => {
            if (apns.is_empty()) {
                return Err(());
            }

            for apn in apns {
                print_apn(apn);
                println!("----------");
            }

            return Ok(());
        },
        None => return Err(())
    };
}

fn print_apn(apn: &Apn) {
    println!("CARRIER_ID: {}", apn.carrier_id);
    println!("CARRIER:    {}", apn.carrier);
    println!("MCC:        {}", apn.mcc);
    println!("MNC:        {}", apn.mnc);
    println!("APN:        {}", apn.apn);
    println!("USER:       {}", apn.user);
    println!("PASSWORD:   {}", apn.password);
    println!("PORT:       {}", apn.port);
    println!("PROXY:      {}", apn.proxy);
    println!("ENABLED:    {}", apn.carrier_enabled);
    println!("VISIBLE:    {}", apn.user_visible);
    println!("TYPE:       {}", apn.type_.join(", "));
}
