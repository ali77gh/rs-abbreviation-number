
const SYMBOLS:[&str;7] = ["","K", "M", "G", "T", "P", "E"];

fn abbreviate_number(num:f64)->String{
    let base:f64=1000.0;
    let mut number=num ;
    let index_of_symbol= num.abs().log(base).floor() as usize;
    let symbol = *SYMBOLS.get(index_of_symbol).unwrap();
    let pw: f64=base.powf(index_of_symbol as f64);
    number /= pw;     
    if ((number*100.0) as i64 % 100)==0{
        println!("first");
        return  format!("{:.0}{}", number, symbol);
    }
    if (number*100.0) as i64 % 10==0{
        return  format!("{:.1}{}", (number*10.0).floor()/10.0, symbol);
    }
    format!("{}{}", (number*100.0).floor()/100.0, symbol)  
}

mod tests{

    use super::*;


    #[test]
    fn abbreviate_number_less_then_thousand_test(){
        assert_eq!("1",abbreviate_number(1.0));
        assert_eq!("0",abbreviate_number(0.0));
        assert_eq!("123",abbreviate_number(123.0));
    }



    #[test]
    fn abbreviate_number_ceil_test(){
        assert_eq!("1K",abbreviate_number(1_000.0));
        assert_eq!("1M",abbreviate_number(1_000_000.0));
        assert_eq!("16G",abbreviate_number(16_000_000_000.0));
        assert_eq!("1T",abbreviate_number(1_000_000_000_000.0));
        assert_eq!("54P",abbreviate_number(54_000_000_000_000_000.0));
        assert_eq!("123E",abbreviate_number(123_000_000_000_000_000_000.0));
    }

    #[test]
    fn abbreviate_number_randoms_test(){
        assert_eq!("5K",abbreviate_number(5_000.0));
        assert_eq!("12M",abbreviate_number(12_000_430.0));
        assert_eq!("1G",abbreviate_number(1_000_345_000.0));
        assert_eq!("133T",abbreviate_number(133_000_000_055_000.0));
        assert_eq!("166P",abbreviate_number(166_000_643_300_000_000.0));
        assert_eq!("999E",abbreviate_number(999_000_123_123_000_000_123.0));
    }

    #[test]
    fn abbreviate_number_randoms_floating_test(){
        assert_eq!("5.1K",abbreviate_number(5_100.0));
        assert_eq!("12.01M",abbreviate_number(12_010_430.0));
        assert_eq!("1.1G",abbreviate_number(1_100_345_000.0));
        assert_eq!("133.09T",abbreviate_number(133_090_000_055_000.0));
        assert_eq!("166.05P",abbreviate_number(166_050_643_300_000_000.0));
        assert_eq!("999.5E",abbreviate_number(999_500_123_123_000_000_123.0));
    }


}