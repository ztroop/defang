use clap::{crate_authors, crate_version, App, Arg};

const SUBSTITUTIONS: [(&str, &str); 4] = [
    (".", "[.]"),
    ("@", "[at]"),
    ("ftp", "fxp"),
    ("http", "hxxp"),
];

enum Order {
    Normal,
    Reversed,
}

fn fang(url: &str, order: Order) -> String {
    let mut url = url.trim().to_owned();
    for item in SUBSTITUTIONS.iter() {
        match order {
            Order::Normal => {
                if url.contains(item.0) {
                    url = url.replace(item.0, item.1);
                }
            }
            Order::Reversed => {
                if url.contains(item.1) {
                    url = url.replace(item.1, item.0);
                }
            }
        }
    }
    url
}

fn main() {
    let matches = App::new("defang")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A utility to defang or refang URL strings")
        .arg(
            Arg::new("defang")
                .short('d')
                .long("defang")
                .value_name("URL")
                .about("Convert a URL into a harmless and shareable format")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("refang")
                .short('r')
                .long("refang")
                .value_name("URL")
                .about("Restore a defanged URL to its original format")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    if let Some(i) = matches.value_of("defang") {
        println!("{}", fang(i, Order::Normal))
    };
    if let Some(i) = matches.value_of("refang") {
        println!("{}", fang(i, Order::Reversed))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defang() {
        let defanged_google: String = fang("http://google.com", Order::Normal);
        assert_eq!(defanged_google, "hxxp://google[.]com".to_owned());

        let defanged_debian: String = fang("ftp://ftp.debian.org/debian", Order::Normal);
        assert_eq!(
            defanged_debian,
            "fxp://fxp[.]debian[.]org/debian".to_owned()
        );

        let defanged_email: String = fang("bad.actor@example.com", Order::Normal);
        assert_eq!(defanged_email, "bad[.]actor[at]example[.]com".to_owned())
    }

    #[test]
    fn test_refang() {
        let refanged_google: String = fang("hxxp://google[.]com", Order::Reversed);
        assert_eq!(refanged_google, "http://google.com".to_owned());

        let refanged_debian: String = fang("fxp://fxp[.]debian[.]org/debian", Order::Reversed);
        assert_eq!(refanged_debian, "ftp://ftp.debian.org/debian".to_owned());

        let refanged_email: String = fang("bad[.]actor[at]example[.]com", Order::Reversed);
        assert_eq!(refanged_email, "bad.actor@example.com".to_owned())
    }
}
