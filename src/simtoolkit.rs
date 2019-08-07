
const MCCMNC_CODES_HAVING_3DIGITS_MNC: &'static [&'static str] = &[
    "405025", "405026", "405027", "405028", "405029", "405030", "405031", "405032",
    "405033", "405034", "405035", "405036", "405037", "405038", "405039", "405040",
    "405041", "405042", "405043", "405044", "405045", "405046", "405047", "405750",
    "405751", "405752", "405753", "405754", "405755", "405756", "405799", "405800",
    "405801", "405802", "405803", "405804", "405805", "405806", "405807", "405808",
    "405809", "405810", "405811", "405812", "405813", "405814", "405815", "405816",
    "405817", "405818", "405819", "405820", "405821", "405822", "405823", "405824",
    "405825", "405826", "405827", "405828", "405829", "405830", "405831", "405832",
    "405833", "405834", "405835", "405836", "405837", "405838", "405839", "405840",
    "405841", "405842", "405843", "405844", "405845", "405846", "405847", "405848",
    "405849", "405850", "405851", "405852", "405853", "405875", "405876", "405877",
    "405878", "405879", "405880", "405881", "405882", "405883", "405884", "405885",
    "405886", "405908", "405909", "405910", "405911", "405912", "405913", "405914",
    "405915", "405916", "405917", "405918", "405919", "405920", "405921", "405922",
    "405923", "405924", "405925", "405926", "405927", "405928", "405929", "405930",
    "405931", "405932"
];

const MCCTABLE: &'static [(&'static str, usize, &'static str)] = &[
    ("202", 2, "gr"),       // Greece
    ("204", 2, "nl_nl"),    // Netherlands (Kingdom of the)
    ("206", 2, "be"),       // Belgium
    ("208", 2, "fr_fr"),    // France
    ("212", 2, "mc"),       // Monaco (Principality of)
    ("213", 2, "ad"),       // Andorra (Principality of)
    ("214", 2, "es_es"),    // Spain
    ("216", 2, "hu"),       // Hungary (Republic of)
    ("218", 2, "ba"),       // Bosnia and Herzegovina
    ("219", 2, "hr"),       // Croatia (Republic of)
    ("220", 2, "rs"),       // Serbia and Montenegro
    ("222", 2, "it_it"),    // Italy
    ("225", 2, "va_it"),    // Vatican City State
    ("226", 2, "ro"),       // Romania
    ("228", 2, "ch_de"),    // Switzerland (Confederation of)
    ("230", 2, "cz_cs"),    // Czech Republic
    ("231", 2, "sk"),       // Slovak Republic
    ("232", 2, "at_de"),    // Austria
    ("234", 2, "gb_en"),    // United Kingdom of Great Britain and Northern Ireland
    ("235", 2, "gb_en"),    // United Kingdom of Great Britain and Northern Ireland
    ("238", 2, "dk"),       // Denmark
    ("240", 2, "se"),       // Sweden
    ("242", 2, "no"),       // Norway
    ("244", 2, "fi"),       // Finland
    ("246", 2, "lt"),       // Lithuania (Republic of)
    ("247", 2, "lv"),       // Latvia (Republic of)
    ("248", 2, "ee"),       // Estonia (Republic of)
    ("250", 2, "ru"),       // Russian Federation
    ("255", 2, "ua"),       // Ukraine
    ("257", 2, "by"),       // Belarus (Republic of)
    ("259", 2, "md"),       // Moldova (Republic of)
    ("260", 2, "pl"),       // Poland (Republic of)
    ("262", 2, "de_de"),    // Germany (Federal Republic of)
    ("266", 2, "gi"),       // Gibraltar
    ("268", 2, "pt"),       // Portugal
    ("270", 2, "lu"),       // Luxembourg
    ("272", 2, "ie_en"),    // Ireland
    ("274", 2, "is"),       // Iceland
    ("276", 2, "al"),       // Albania (Republic of)
    ("278", 2, "mt"),       // Malta
    ("280", 2, "cy"),       // Cyprus (Republic of)
    ("282", 2, "ge"),       // Georgia
    ("283", 2, "am"),       // Armenia (Republic of)
    ("284", 2, "bg"),       // Bulgaria (Republic of)
    ("286", 2, "tr"),       // Turkey
    ("288", 2, "fo"),       // Faroe Islands
    ("289", 2, "ge"),       // Abkhazia (Georgia)
    ("290", 2, "gl"),       // Greenland (Denmark)
    ("292", 2, "sm"),       // San Marino (Republic of)
    ("293", 2, "si"),       // Slovenia (Republic of)
    ("294", 2, "mk"),       // The Former Yugoslav Republic of Macedonia
    ("295", 2, "li"),       // Liechtenstein (Principality of)
    ("297", 2, "me"),       // Montenegro (Republic of)
    ("302", 3, "ca"),       // Canada
    ("308", 2, "pm"),       // Saint Pierre and Miquelon (Collectivit territoriale de la Rpublique franaise)
    ("310", 3, "us_en"),    // United States of America
    ("311", 3, "us_en"),    // United States of America
    ("312", 3, "us_en"),    // United States of America
    ("313", 3, "us_en"),    // United States of America
    ("314", 3, "us_en"),    // United States of America
    ("315", 3, "us_en"),    // United States of America
    ("316", 3, "us_en"),    // United States of America
    ("330", 2, "pr"),       // Puerto Rico
    ("332", 2, "vi"),       // United States Virgin Islands
    ("334", 3, "mx"),       // Mexico
    ("338", 3, "jm"),       // Jamaica
    ("340", 2, "gp"),       // Guadeloupe (French Department of)
    ("342", 3, "bb"),       // Barbados
    ("344", 3, "ag"),       // Antigua and Barbuda
    ("346", 3, "ky"),       // Cayman Islands
    ("348", 3, "vg"),       // British Virgin Islands
    ("350", 2, "bm"),       // Bermuda
    ("352", 2, "gd"),       // Grenada
    ("354", 2, "ms"),       // Montserrat
    ("356", 2, "kn"),       // Saint Kitts and Nevis
    ("358", 2, "lc"),       // Saint Lucia
    ("360", 2, "vc"),       // Saint Vincent and the Grenadines
    ("362", 2, "ai"),       // Netherlands Antilles
    ("363", 2, "aw"),       // Aruba
    ("364", 2, "bs"),       // Bahamas (Commonwealth of the)
    ("365", 3, "ai"),       // Anguilla
    ("366", 2, "dm"),       // Dominica (Commonwealth of)
    ("368", 2, "cu"),       // Cuba
    ("370", 2, "do"),       // Dominican Republic
    ("372", 2, "ht"),       // Haiti (Republic of)
    ("374", 2, "tt"),       // Trinidad and Tobago
    ("376", 2, "tc"),       // Turks and Caicos Islands
    ("400", 2, "az"),       // Azerbaijani Republic
    ("401", 2, "kz"),       // Kazakhstan (Republic of)
    ("402", 2, "bt"),       // Bhutan (Kingdom of)
    ("404", 2, "in"),       // India (Republic of)
    ("405", 2, "in"),       // India (Republic of)
    ("410", 2, "pk"),       // Pakistan (Islamic Republic of)
    ("412", 2, "af"),       // Afghanistan
    ("413", 2, "lk"),       // Sri Lanka (Democratic Socialist Republic of)
    ("414", 2, "mm"),       // Myanmar (Union of)
    ("415", 2, "lb"),       // Lebanon
    ("416", 2, "jo"),       // Jordan (Hashemite Kingdom of)
    ("417", 2, "sy"),       // Syrian Arab Republic
    ("418", 2, "iq"),       // Iraq (Republic of)
    ("419", 2, "kw"),       // Kuwait (State of)
    ("420", 2, "sa"),       // Saudi Arabia (Kingdom of)
    ("421", 2, "ye"),       // Yemen (Republic of)
    ("422", 2, "om"),       // Oman (Sultanate of)
    ("423", 2, "ps"),       // Palestine
    ("424", 2, "ae"),       // United Arab Emirates
    ("425", 2, "il"),       // Israel (State of)
    ("426", 2, "bh"),       // Bahrain (Kingdom of)
    ("427", 2, "qa"),       // Qatar (State of)
    ("428", 2, "mn"),       // Mongolia
    ("429", 2, "np"),       // Nepal
    ("430", 2, "ae"),       // United Arab Emirates
    ("431", 2, "ae"),       // United Arab Emirates
    ("432", 2, "ir"),       // Iran (Islamic Republic of)
    ("434", 2, "uz"),       // Uzbekistan (Republic of)
    ("436", 2, "tj"),       // Tajikistan (Republic of)
    ("437", 2, "kg"),       // Kyrgyz Republic
    ("438", 2, "tm"),       // Turkmenistan
    ("440", 2, "jp_ja"),    // Japan
    ("441", 2, "jp_ja"),    // Japan
    ("450", 2, "kr_ko"),    // Korea (Republic of)
    ("452", 2, "vn"),       // Viet Nam (Socialist Republic of)
    ("454", 2, "hk"),       // "Hong Kong, China"
    ("455", 2, "mo"),       // "Macao, China"
    ("456", 2, "kh"),       // Cambodia (Kingdom of)
    ("457", 2, "la"),       // Lao People's Democratic Republic
    ("460", 2, "cn_zh"),    // China (People's Republic of)
    ("461", 2, "cn_zh"),    // China (People's Republic of)
    ("466", 2, "tw"),       // "Taiwan, China"
    ("467", 2, "kp"),       // Democratic People's Republic of Korea
    ("470", 2, "bd"),       // Bangladesh (People's Republic of)
    ("472", 2, "mv"),       // Maldives (Republic of)
    ("502", 2, "my"),       // Malaysia
    ("505", 2, "au_en"),    // Australia
    ("510", 2, "id"),       // Indonesia (Republic of)
    ("514", 2, "tl"),       // Democratic Republic of Timor-Leste
    ("515", 2, "ph"),       // Philippines (Republic of the)
    ("520", 2, "th"),       // Thailand
    ("525", 2, "sg_en"),    // Singapore (Republic of)
    ("528", 2, "bn"),       // Brunei Darussalam
    ("530", 2, "nz_en"),    // New Zealand
    ("534", 2, "mp"),       // Northern Mariana Islands (Commonwealth of the)
    ("535", 2, "gu"),       // Guam
    ("536", 2, "nr"),       // Nauru (Republic of)
    ("537", 2, "pg"),       // Papua New Guinea
    ("539", 2, "to"),       // Tonga (Kingdom of)
    ("540", 2, "sb"),       // Solomon Islands
    ("541", 2, "vu"),       // Vanuatu (Republic of)
    ("542", 2, "fj"),       // Fiji (Republic of)
    ("543", 2, "wf"),       // Wallis and Futuna (Territoire franais d'outre-mer)
    ("544", 2, "as"),       // American Samoa
    ("545", 2, "ki"),       // Kiribati (Republic of)
    ("546", 2, "nc"),       // New Caledonia (Territoire franais d'outre-mer)
    ("547", 2, "pf"),       // French Polynesia (Territoire franais d'outre-mer)
    ("548", 2, "ck"),       // Cook Islands
    ("549", 2, "ws"),       // Samoa (Independent State of)
    ("550", 2, "fm"),       // Micronesia (Federated States of)
    ("551", 2, "mh"),       // Marshall Islands (Republic of the)
    ("552", 2, "pw"),       // Palau (Republic of)
    ("602", 2, "eg"),       // Egypt (Arab Republic of)
    ("603", 2, "dz"),       // Algeria (People's Democratic Republic of)
    ("604", 2, "ma"),       // Morocco (Kingdom of)
    ("605", 2, "tn"),       // Tunisia
    ("606", 2, "ly"),       // Libya (Socialist People's Libyan Arab Jamahiriya)
    ("607", 2, "gm"),       // Gambia (Republic of the)
    ("608", 2, "sn"),       // Senegal (Republic of)
    ("609", 2, "mr"),       // Mauritania (Islamic Republic of)
    ("610", 2, "ml"),       // Mali (Republic of)
    ("611", 2, "gn"),       // Guinea (Republic of)
    ("612", 2, "ci"),       // Cte d'Ivoire (Republic of)
    ("613", 2, "bf"),       // Burkina Faso
    ("614", 2, "ne"),       // Niger (Republic of the)
    ("615", 2, "tg"),       // Togolese Republic
    ("616", 2, "bj"),       // Benin (Republic of)
    ("617", 2, "mu"),       // Mauritius (Republic of)
    ("618", 2, "lr"),       // Liberia (Republic of)
    ("619", 2, "sl"),       // Sierra Leone
    ("620", 2, "gh"),       // Ghana
    ("621", 2, "ng"),       // Nigeria (Federal Republic of)
    ("622", 2, "td"),       // Chad (Republic of)
    ("623", 2, "cf"),       // Central African Republic
    ("624", 2, "cm"),       // Cameroon (Republic of)
    ("625", 2, "cv"),       // Cape Verde (Republic of)
    ("626", 2, "st"),       // Sao Tome and Principe (Democratic Republic of)
    ("627", 2, "gq"),       // Equatorial Guinea (Republic of)
    ("628", 2, "ga"),       // Gabonese Republic
    ("629", 2, "cg"),       // Congo (Republic of the)
    ("630", 2, "cg"),       // Democratic Republic of the Congo
    ("631", 2, "ao"),       // Angola (Republic of)
    ("632", 2, "gw"),       // Guinea-Bissau (Republic of)
    ("633", 2, "sc"),       // Seychelles (Republic of)
    ("634", 2, "sd"),       // Sudan (Republic of the)
    ("635", 2, "rw"),       // Rwanda (Republic of)
    ("636", 2, "et"),       // Ethiopia (Federal Democratic Republic of)
    ("637", 2, "so"),       // Somali Democratic Republic
    ("638", 2, "dj"),       // Djibouti (Republic of)
    ("639", 2, "ke"),       // Kenya (Republic of)
    ("640", 2, "tz"),       // Tanzania (United Republic of)
    ("641", 2, "ug"),       // Uganda (Republic of)
    ("642", 2, "bi"),       // Burundi (Republic of)
    ("643", 2, "mz"),       // Mozambique (Republic of)
    ("645", 2, "zm"),       // Zambia (Republic of)
    ("646", 2, "mg"),       // Madagascar (Republic of)
    ("647", 2, "re"),       // Reunion (French Department of)
    ("648", 2, "zw"),       // Zimbabwe (Republic of)
    ("649", 2, "na"),       // Namibia (Republic of)
    ("650", 2, "mw"),       // Malawi
    ("651", 2, "ls"),       // Lesotho (Kingdom of)
    ("652", 2, "bw"),       // Botswana (Republic of)
    ("653", 2, "sz"),       // Swaziland (Kingdom of)
    ("654", 2, "km"),       // Comoros (Union of the)
    ("655", 2, "za_en"),    // South Africa (Republic of)
    ("657", 2, "er"),       // Eritrea
    ("702", 2, "bz"),       // Belize
    ("704", 2, "gt"),       // Guatemala (Republic of)
    ("706", 2, "sv"),       // El Salvador (Republic of)
    ("708", 3, "hn"),       // Honduras (Republic of)
    ("710", 2, "ni"),       // Nicaragua
    ("712", 2, "cr"),       // Costa Rica
    ("714", 2, "pa"),       // Panama (Republic of)
    ("716", 2, "pe"),       // Peru
    ("722", 3, "ar"),       // Argentine Republic
    ("724", 2, "br"),       // Brazil (Federative Republic of)
    ("730", 2, "cl"),       // Chile
    ("732", 3, "co"),       // Colombia (Republic of)
    ("734", 2, "ve"),       // Venezuela (Bolivarian Republic of)
    ("736", 2, "bo"),       // Bolivia (Republic of)
    ("738", 2, "gy"),       // Guyana
    ("740", 2, "ec"),       // Ecuador
    ("742", 2, "gf"),       // French Guiana (French Department of)
    ("744", 2, "py"),       // Paraguay (Republic of)
    ("746", 2, "sr"),       // Suriname (Republic of)
    ("748", 2, "uy"),       // Uruguay (Eastern Republic of)
    ("750", 2, "fk"),       // Falkland Islands (Malvinas)
];

pub struct IMSI<'a> {
    pub mcc: &'a str,
    pub mnc: &'a str,
    pub msin: &'a str,
    pub region: &'a str
}

pub struct IccHelper {

}

impl IccHelper {
    pub fn parse_imsi(imsi: &str) -> IMSI {
        if (imsi.len() < 5) {
            panic!("Invalid IMSI");
        }

        let mcc: &str = &imsi[0..3];
        let mut mnc_len: usize = 2;
        let mut region: &str = "";

        // Currently MCCMNC_CODES_HAVING_3DIGITS_MNC only includes 405 codes,
        // this if statement needs to be removed (or modified) when this changes!
        if (mcc == "405") {
            for mccmnc in MCCMNC_CODES_HAVING_3DIGITS_MNC {
                if (&&imsi[0..6] == mccmnc) {
                    mnc_len = 3;
                    break;
                }
            }
        }

        if (mnc_len != 3) {
            for entry in MCCTABLE {
                if (mcc == entry.0) {
                    mnc_len = entry.1;
                    region = entry.2;
                    break;
                }
            }
        }

        if (imsi.len() < (3 + mnc_len)) {
            panic!("Possibly invalid IMSI, expected minimum length of {}", 3 + mnc_len);
        }

        let mnc: &str = &imsi[3..(3 + mnc_len)];
        let msin: &str = &imsi[(3 + mnc_len)..];

        return IMSI { mcc, mnc, msin, region };
    }
}