use clap::{crate_authors, crate_version, App, Arg};

const SUBSTITUTIONS: [[&str; 2]; 4] = [
    [".", "[.]"],
    ["@", "[at]"],
    ["ftp", "fxp"],
    ["http", "hxxp"],
];

enum Order {
    Normal,
    Reversed,
}

fn replace(url: &mut String, x: usize, y: usize) {
    for item in SUBSTITUTIONS.iter() {
        if url.contains(item[x]) {
            *url = url.replace(item[x], item[y]);
        }
    }
}

fn fang(url: &str, order: Order) -> String {
    let mut url = url.trim().to_owned();
    match order {
        Order::Normal => replace(&mut url, 0_usize, 1_usize),
        Order::Reversed => replace(&mut url, 1_usize, 0_usize),
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
