use std::io::{self, Write};
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use chrono::naive::Days;

type Comment = String;
type Value   = u64;
#[derive(Copy, Clone, Debug)]
enum Effect { Additive, Subtractive }
struct Record(Comment, Value, Effect, NaiveDateTime);
struct Ledger(Comment, Vec<Record>);

impl Effect {
    fn to_str(self) -> &'static str {
        use Effect::*;
        match self {
            Additive    => "+",
            Subtractive => "-",
        }
    }
}

impl Record {
    fn render(&self, out: &mut impl Write) -> io::Result<()> {
        let mut val = format!("{:>16}", self.1);
        let ind = val.len() - 2;
        val.insert(ind, '.');
        writeln!(out, "\t£{} {} : {} : {}",
                 val, self.2.to_str(),
                 self.3.format("%d/%m/%Y %H:%M:%S"),
                 self.0)?;
        Ok(())
    }
}
impl Ledger {
    fn render(&self, out: &mut impl Write) -> io::Result<()> {
        writeln!(out, "{}:", self.0)?;
        for record in &self.1 {
            record.render(out)?;
        }
        Ok(())
    }

    fn total(&self) -> i128 {
        let mut tot = 0;
        for record in &self.1 {
            match record.2 {
                Effect::Additive    => tot += record.1 as i128,
                Effect::Subtractive => tot -= record.1 as i128,
            }
        }
        return tot;
    }
}

macro_rules! magic {
    [$x: expr] => {
        NaiveDateTime::parse_from_str($x, "%d/%m/%Y %H:%M:%S").unwrap()
    }
}

fn main() -> io::Result<()> {
    let today    = NaiveDate::from_ymd_opt(2023, 6, 1).unwrap();
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    let other    = NaiveTime::from_hms_opt(17, 42, 30).unwrap();
    let test     = Ledger(String::from("my money"), vec![
            Record(String::from("initial payment"),  4802, Effect::Additive,    magic!["29/06/2023 00:00:00"]),
            Record(String::from("XBOX GAMES"),        750, Effect::Subtractive, magic!["30/05/2023 14:43:50"]),
            Record(String::from("balls and beer"),    460, Effect::Subtractive, magic!["30/05/2023 15:28:42"]),
            Record(String::from("costco hotdog"),     150, Effect::Subtractive, magic!["31/05/2023 17:00:00"]),
            Record(String::from("bottle of rum :)"), 2350, Effect::Subtractive, magic!["01/06/2023 17:42:00"]),
    ]);
    test.render(&mut io::stdout())?;
    println!("total:: {}", test.total());
    Ok(())
}
