use clap::{crate_authors, crate_version, App, Arg};

const SUBS: [(&str, &str); 3] = [(".", "[.]"), ("http", "hxxp"), ("ftp", "fxp")];

fn reverse_tuple<'a>(pair: (&'a str, &'a str)) -> (&'a str, &'a str) {
    let (first, second) = pair;
    (second, first)
}

fn replace(mut url: String, s: [(&str, &str); 3]) -> String {
    for item in s.iter() {
        if url.contains(item.0) {
            url = url.replace(item.0, item.1);
        }
    }
    url
}

fn fang(url: &str, reversed: bool) -> String {
    let url = String::from(url.trim());
    if reversed {
        replace(url, SUBS.clone().map(|x| reverse_tuple(x)))
    } else {
        replace(url, SUBS)
    }
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

    match matches.value_of("defang") {
        Some(i) => println!("{}", fang(i, false)),
        None => (),
    };

    match matches.value_of("refang") {
        Some(i) => println!("{}", fang(i, true)),
        None => (),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defang() {
        let defanged_google: String = fang("http://google.com", false);
        assert_eq!(defanged_google, String::from("hxxp://google[.]com"));

        let defanged_debian: String = fang("ftp://ftp.debian.org/debian", false);
        assert_eq!(
            defanged_debian,
            String::from("fxp://fxp[.]debian[.]org/debian")
        )
    }

    #[test]
    fn test_refang() {
        let defanged_google: String = fang("hxxp://google[.]com", true);
        assert_eq!(defanged_google, String::from("http://google.com"));

        let defanged_debian: String = fang("fxp://fxp[.]debian[.]org/debian", true);
        assert_eq!(defanged_debian, String::from("ftp://ftp.debian.org/debian"))
    }
}
