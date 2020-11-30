use ta_common::fixed_queue::FixedQueue;
use ta_common::traits::Indicator;
use ta_common::math::min::Min;
use ta_common::math::max::Max;

pub struct Fisher {
    period: u32,
    history: FixedQueue<[f64; 2]>,
    min: Min,
    max: Max,
    prev_val: f64,
    prev_fischer: f64,
}


impl Fisher {
    pub fn new(period: u32) -> Fisher {
        Self {
            period,
            history: FixedQueue::new(period),
            max: Max::new(period),
            min: Min::new(period),
            prev_val: 0.0,
            prev_fischer: 0.0,
        }
    }
}

impl Indicator<[f64; 2], Option<[f64; 2]>> for Fisher {
    fn next(&mut self, input: [f64; 2]) -> Option<[f64; 2]> {
        let [high, low] = input;
        self.history.add(input);
        let avg = (high + low) / 2.0;
        let min = self.min.next(avg);
        let max = self.max.next(avg);

        return if self.history.is_full() {
            let min = min.unwrap();
            let max = max.unwrap();
            let mut diff = max - min;
            if diff == 0.0 {
                diff = 0.001;
            }
            let mut val = 0.66 * (((avg - min) / diff) - 0.5) + 0.67 * self.prev_val;
            let temp = (1.0 + val) / (1.0 - val);
            let fisher = 0.5 * temp.ln() + 0.5 * self.prev_fischer;
           // println!("high{} low{} min {} max {} val {} fisher {} ", high, low, min, max, val, fisher);
            let result = [fisher, self.prev_fischer];
            self.prev_val = val;
            self.prev_fischer = fisher;
            Some(result)
        } else {
            None
        };
    }

    fn reset(&mut self) {
        self.history.reset();
        self.max.reset();
        self.min.reset();
        self.prev_val=0.0;
        self.prev_fischer=0.0;
    }
}


#[cfg(test)]
mod tests {
    use crate::Fisher;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut fisher = Fisher::new(5);
        assert_eq!(fisher.next([82.15, 81.29]), None);
        assert_eq!(fisher.next([81.89, 80.64]), None);
        assert_eq!(fisher.next([83.03, 81.31]), None);
        assert_eq!(fisher.next([83.30, 82.65]), None);
        assert_eq!(fisher.next([83.85, 83.07]), Some([0.34282825441539394, 0.0]));
        assert_eq!(fisher.next([83.90, 83.11]), Some([0.7913738721291064, 0.34282825441539394]));
        assert_eq!(fisher.next([83.33, 82.49]), Some([0.8253978616040214, 0.7913738721291064]));
        assert_eq!(fisher.next([84.30, 82.30]), Some([0.8057742927742315, 0.8253978616040214]));
        assert_eq!(fisher.next([84.84, 84.15]), Some([1.0662328466080955, 0.8057742927742315]));
        assert_eq!(fisher.next([85.00, 84.11]), Some([1.4386723332736164, 1.0662328466080955]));
        assert_eq!(fisher.next([85.90, 84.03]), Some([1.851401398210955, 1.4386723332736164]));
        assert_eq!(fisher.next([86.58, 85.39]), Some([2.274864111876407, 1.851401398210955]));
        assert_eq!(fisher.next([86.98, 85.76]), Some([2.697820152019159, 2.274864111876407]));
        assert_eq!(fisher.next([88.00, 87.17]), Some([3.1167647415593853, 2.697820152019159]));
        assert_eq!(fisher.next([87.87, 87.01]), Some([3.1846885349584984, 3.1167647415593853]));
    }
}
