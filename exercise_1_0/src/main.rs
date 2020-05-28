
#[derive(Debug, PartialEq)]
pub struct Time {
    ms: u64,
}

impl Time {
    fn new(ms: u64) -> Self {
        Time { ms }
    }
}

fn in_hours(t: Time) -> u64 {
    t.ms / 1000 / 60 / 60
}

fn greater(t1: Time, t2: Time) -> Time {
    if t1.ms > t2.ms { t1 } else { t2 }
}

// implement without cloning
fn greatest(v: Vec<Time>) -> Time {
    let mut res: Time = Time::new(0);
    for time in v {
        if time.ms > res.ms {
            res = Time::new(time.ms);
        }
    }
    res
}

// references
fn time_diff_in_ms(t1: &Time, t2: &Time) -> u64 {
    if t1.ms > t2.ms { t1.ms - t2.ms } else { t2.ms - t1.ms } 
}

fn greatest_ref(v: &Vec<Time>) -> &Time {
    if v.len() == 0 {
        panic!("Cannot work with empty vector");
    }
    let mut res: &Time = &v[0];
    for time in v {
        if time.ms > res.ms {
            res = &time;
        }
    }
    res
}

fn greatest_ref_safe(v: &Vec<Time>) -> Option<&Time> {
    if v.len() == 0 {
        return None;
    }
    let mut res: &Time = &v[0];
    for time in v {
        if time.ms > res.ms {
            res = &time;
        }
    }
    Some(res)
}

fn main() {
    let t1 = Time::new(1);
    let t2 = Time::new(10);
    println!("{:?}", time_diff_in_ms(&t1, &t2));
}

mod test {
    use super::*;

    #[test]
    fn test_in_hours() {
        let t = Time {
            ms: 3600 * 1000 * 5,
        };
        assert_eq!(5, in_hours(t));
    }

    #[test]
    fn test_greater() {
        let t1 = Time::new(5);
        let t2 = Time::new(1);
        let res = greater(t1, t2);
        assert_eq!(5, res.ms);
    }

    #[test]
    fn test_greatest() {
        let v = vec![Time::new(1), Time::new(5)];
        let res = greatest(v);
        assert_eq!(5, res.ms);
    }

    #[test]
    fn test_time_diff() {
        let t1 = Time::new(1);
        let t2 = Time::new(10);
        assert_eq!(9, time_diff_in_ms(&t1, &t2));
        assert_eq!(9, time_diff_in_ms(&t2, &t1));
    }

    #[test]
    fn test_greatest_ref() {
        let v = vec![Time::new(1), Time::new(5)];
        let res = greatest_ref(&v);
        assert_eq!(5, res.ms);
    }

    #[test]
    fn test_greatest_ref_safe_ok() {
        let v = vec![Time::new(1), Time::new(5)];
        match greatest_ref_safe(&v) {
            Some(res) => {
                assert_eq!(5, res.ms);
            }
            None => {}
        }
    }

    #[test]
    fn test_greatest_ref_safe_fail() {
        let v = vec![];
        match greatest_ref_safe(&v) {
            Some(_) => { }
            None => {
                assert_eq!(true, true);
            }
        }
    }
    
}
