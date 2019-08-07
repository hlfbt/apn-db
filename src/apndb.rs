use std::string::String;
use std::vec::Vec;
use std::collections::HashMap;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Apn {
    pub carrier_id: u32,
    pub carrier: String,
    pub carrier_enabled: bool,
    pub user_visible: bool,
    pub mcc: String,
    pub mnc: String,
    pub apn: String,
    pub user: String,
    pub password: String,
    pub port: u32,
    pub proxy: String,
    pub type_: Vec<String>
}

impl Apn {
    // Ordering::Less is "better"
    // Ordering::Greater is "worse"
    // This way, when sorted normally, the natural order is preserved on equality
    fn cmp_type(&self, other: &Self, type_: &str) -> Ordering {
        let typ: String = String::from(type_);
        let equals_type = |s: &Apn, o: &Apn| s.type_ == [typ.clone()] && o.type_ != [typ.clone()];
        let contains_type = |s: &Apn, o: &Apn| s.type_.contains(&typ) && !o.type_.contains(&typ);

        if (equals_type(self, other)) {
            return Ordering::Less;
        } else if (equals_type(other, self)) {
            return Ordering::Greater;
        }

        if (contains_type(self, other)) {
            return Ordering::Less;
        } else if (contains_type(other, self)) {
            return Ordering::Greater;
        }

        return Ordering::Equal;
    }

    fn cmp_misc_predicates(&self, other: &Self) -> Ordering {
        let mut predicates: Vec<fn(&Self, &Self) -> bool> = Vec::new();
        predicates.push(|s, o| s.carrier_enabled && o.carrier_enabled);
        predicates.push(|s, o| s.user.is_empty() && s.password.is_empty() && !(o.user.is_empty() && o.password.is_empty()));
        predicates.push(|s, o| s.proxy.is_empty() && !o.proxy.is_empty());

        for predicate in predicates {
            if (predicate(self, other)) {
                return Ordering::Less;
            } else if (predicate(other, self)) {
                return Ordering::Greater;
            }
        }

        return Ordering::Equal;
    }

    pub fn cmp_for_default(&self, other: &Self) -> Ordering {
        return match self.cmp_type(other, "default") {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.cmp_misc_predicates(other)
        };
    }

    pub fn cmp_for_lte(&self, other: &Self) -> Ordering {
        return match self.cmp_type(other, "ia") {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.cmp_for_default(other)
        };
    }
}

impl Clone for Apn {
    fn clone(&self) -> Self {
        return Apn {
            carrier_id: self.carrier_id.clone(),
            carrier: self.carrier.clone(),
            carrier_enabled: self.carrier_enabled.clone(),
            user_visible: self.user_visible.clone(),
            mcc: self.mcc.clone(),
            mnc: self.mnc.clone(),
            apn: self.apn.clone(),
            user: self.user.clone(),
            password: self.password.clone(),
            port: self.port.clone(),
            proxy: self.proxy.clone(),
            type_: self.type_.clone()
        }
    }
}

impl PartialEq for Apn {
    fn eq(&self, other: &Self) -> bool {
        return (self.mcc == other.mcc && self.mnc == other.mnc);
    }
}
impl Eq for Apn {}

impl PartialOrd for Apn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}
impl Ord for Apn {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.eq(other)) {
            return Ordering::Equal;
        } else {
            if (self.mcc > other.mcc) {
                return Ordering::Greater;
            } else {
                if (self.mnc > other.mnc) {
                    return Ordering::Greater;
                } else if (self.mnc < other.mnc) {
                    return Ordering::Less;
                } else {
                    return Ordering::Equal;
                }
            }
        }
    }
}

macro_rules! get_or {
    ($m:ident, $k:expr, $d:expr, $t:ty) => {
        $m.get($k).map(|a|a.clone()).map(|a|a.parse::<$t>().unwrap_or_else(|_|$d as $t)).unwrap_or_else(||$d as $t)
    };
    ($m:ident, $k:expr, $d:expr, !$t:ty) => {
        $m.get($k).map(|a|a.clone()).map(|a|a.parse::<$t>().unwrap()).unwrap_or_else(||$d as $t)
    };
    ($m:ident, $k:expr, $t:ty) => {
        $m.get($k).map(|a|a.clone()).map(|a|a.parse::<$t>().unwrap_or_default()).unwrap_or_default()
    };
    ($m:ident, $k:expr, !$t:ty) => {
        $m.get($k).map(|a|a.clone()).map(|a|a.parse::<$t>().unwrap()).unwrap_or_default()
    };
    ($m:ident, $k:expr, $d:expr) => {
        $m.get($k).map(|a|a.clone()).unwrap_or_else(||String::from($d))
    };
    ($m:ident, $k:expr) => {
        get_or!($m, $k, "")
    };
}

#[derive(Debug)]
pub struct ApnDb {
    pub apns: HashMap<String, Vec<Apn>>
}

impl ApnDb {
    pub fn new() -> ApnDb {
        return ApnDb {
            apns: HashMap::new()
        };
    }

    pub fn parse(&mut self, xml: &str) {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        self.apns.clear();
        let mut buf = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    if (e.name() == b"apn") {
                        let (key, apn): (String, Apn) = ApnDb::parse_apn_element(&e);
                        (match self.apns.get_mut(&key) {
                            Some(vec) => vec,
                            None => {
                                self.apns.insert(key.clone(), Vec::new());
                                self.apns.get_mut(&key).unwrap()
                            }
                        }).push(apn);
                    }
                },
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => ()
            };

            buf.clear();
        };
    }

    fn parse_apn_element(element: &BytesStart) -> (String, Apn) {
        if (element.name() != b"apn") {
            panic!("element is not an apn: {:?}", element);
        }

        let attrs: HashMap<&str, String> = element.attributes().map(|a| {
            let a_ = a.unwrap();
            return (std::str::from_utf8(a_.key).unwrap(), String::from(std::str::from_utf8(&a_.value).unwrap()));
        }).collect::<HashMap<_, _>>();

        let mcc: String = get_or!(attrs, "mcc");
        let mnc: String = get_or!(attrs, "mnc");

        return (format!("{}{}", mcc, mnc) , Apn {
            carrier_id: get_or!(attrs, "carrier_id", u32),
            carrier: get_or!(attrs, "carrier"),
            carrier_enabled: get_or!(attrs, "carrier_enabled", true, bool),
            user_visible: get_or!(attrs, "user_visible", true, bool),
            mcc: mcc,
            mnc: mnc,
            apn: get_or!(attrs, "apn"),
            user: get_or!(attrs, "user"),
            password: get_or!(attrs, "password"),
            port: get_or!(attrs, "port", u32),
            proxy: get_or!(attrs, "proxy"),
            type_: get_or!(attrs, "type").split(",").map(|s|String::from(s)).collect()
        });
    }

    pub fn query(&self, mcc: &str, mnc: &str) -> Option<&Vec<Apn>> {
        return self.apns.get(&format!("{}{}", mcc, mnc));
    }

    fn query_and_sort(&self, mcc: &str, mnc: &str, f: fn(&Apn, &Apn) -> Ordering) -> Option<Vec<Apn>> {
        match self.query(mcc, mnc) {
            Some(a) => {
                let mut apns = a.to_vec();
                apns.sort_by(f);

                return Some(apns);
            },
            None => return None
        };
    }

    pub fn query_default(&self, mcc: &str, mnc: &str) -> Option<Apn> {
        return match self.query_and_sort(mcc, mnc, |s, o| s.cmp_for_default(o)) {
            Some(apns) => apns.get(0).map(|a| a.clone()),
            None => None
        };
    }

    pub fn query_lte(&self, mcc: &str, mnc: &str) -> Option<Apn> {
        return match self.query_and_sort(mcc, mnc, |s, o| s.cmp_for_lte(o)) {
            Some(apns) => apns.get(0).map(|a| a.clone()),
            None => None
        };
    }
}
