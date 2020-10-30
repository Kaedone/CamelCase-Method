// Rust tests
extern crate rand;
use self::rand::Rng;

#[test]
fn sample_test() {
  assert_eq!(camel_case("test case"), "TestCase");
  assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
  assert_eq!(camel_case("say hello "), "SayHello");
  assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
  assert_eq!(camel_case(""), "");
}

fn solution(str: &str) -> String {
    str.split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .map(|&s| [&s[..1].to_uppercase(), &s[1..]].join(""))
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn random_tests() {
    let quotes = ["Sam x abc", "harris yellow black", "patrick z uip", "Feenan evan mac", "Cole P hop", "Favuzzi greg", "david Lendieta cucker", "george sneeze", "Kile clooney make me", "marky bark dark glock", " "];

    for _ in 0..10 {
        let quote = quotes[rand::thread_rng().gen_range(0, quotes.len() - 1)];
        assert_eq!(camel_case(&quote), solution(&quote));
    }
}
