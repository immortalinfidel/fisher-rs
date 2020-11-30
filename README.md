# Fisher Transform (fisher) 

fisher.next([high,low])=>Some([fisher,signal]);
```
use ta_common::traits::Indicator;
use fisher_rs::Fisher;

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





```