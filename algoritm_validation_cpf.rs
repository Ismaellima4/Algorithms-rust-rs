fn validation_cpf(cpf_number: &str) -> bool {
    let mut cpf = parse_u32(remove_char_not_digit(cpf_number));
    match cpf.len() {
        len if len != 11 => false,
        _ => {
            let mut cpf_cloned = cpf.clone();
            let sum_primary_number = validation_last_number(&mut cpf, 10, 2);
            let sum_second_number = validation_last_number(&mut cpf_cloned, 11, 1);

            match sum_primary_number {
                n if n == cpf[9] => match sum_second_number {
                    n2 if n2 == cpf[10] => return true,
                    _ => return false,
                },
                _ => return false,
            };
        }
    }
}

fn validation_last_number(cpf: &mut Vec<u32>, mut iter_n: u32, minor_n: usize) -> u32 {
    let mut sum = 0;
    for i in 0..cpf.len() - minor_n {
        cpf[i] *= iter_n;
        sum += cpf[i];
        iter_n -= 1;
    }
    sum = sum * 10 % 11;
    sum
}

fn parse_u32(cpf: Vec<char>) -> Vec<u32> {
    let cpf_parser_u32 = cpf
        .iter()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    cpf_parser_u32
}

fn remove_char_not_digit(cpf: &str) -> Vec<char> {
    let cpf = cpf
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<Vec<char>>();
    cpf
}

fn main() {
    let is_valid_cpf = validation_cpf("529.982.247-25");
    println!("{is_valid_cpf}");
}

#[test]
fn test_validation_cpf() {
    assert!(validation_cpf("529.982.247-25"));
}

#[test]
fn test_no_validation_cpf() {
    assert!(!validation_cpf("123.456.789-12"));
    assert!(!validation_cpf("123.456.789"));
    assert!(!validation_cpf("123.456.78"));
    assert!(!validation_cpf("123.456.7"));
    assert!(!validation_cpf("123.456"));
    assert!(!validation_cpf("123.45"));
    assert!(!validation_cpf("123.4"));
}
