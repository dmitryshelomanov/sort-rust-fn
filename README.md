# Simple Sort functions 

[![Build Status](https://travis-ci.org/userbq201/sort-rust-fn.svg?branch=master)](https://travis-ci.org/userbq201/sort-rust-fn)

## How to use

```rust
    /*
        sort_types
        bubble_sort,
        inerted_sort,
        shake_sort,
        odd_even_sort,
        comb_sort,
        shell_sort
    */
    use base_sort::*;
    
    fn main() {
        let mut example = vec![3, 2, 1];

        example.bubble_sort(); // example.[sort_types]()
    }
```

## Branch result

#### buble sort (not sorted array)
```
    time:   [1.3388 ms 1.3572 ms 1.3743 ms]
    change: [-8.7069% -7.6302% -6.5608%] (p = 0.00 < 0.05)
    Performance has improved.
```
#### buble sort (sorted array)
```
    time:   [1.0686 us 1.0735 us 1.0795 us]
    change: [-2.5749% -1.8760% -1.1529%] (p = 0.00 < 0.05)
    Performance has improved.
```

#### comb sort (not sorted array)
```
    time:   [656.31 ns 667.19 ns 677.67 ns]
    change: [+5093.458740633548% +5146.2281576009045% +5205.441298795505%] (p = 0.00 < 0.05)
    Performance has regressed.
```
#### comb sort  (sorted array)
```
    time:   [306.36 ns 307.57 ns 308.98 ns]
    change: [+2609.3781030425757% +2637.546241073853% +2664.127729847495%] (p = 0.00 < 0.05)
    Performance has regressed.
```

#### inerted sort (not sorted array)
```
    time:   [821.18 us 833.12 us 845.42 us]
    change: [+9376098.756041344% +9477495.682514401% +9583208.667440869%] (p = 0.00 < 0.05)
    Performance has regressed.
```
#### inerted sort  (sorted array)
```
    time:   [380.69 us 382.33 us 384.25 us]
    change: [+4982163.110187467% +5020264.317365841% +5056037.778851638%] (p = 0.00 < 0.05)
    Performance has regressed.
```

#### shell sort (not sorted array)
```
    time:   [17.415 us 17.478 us 17.548 us]
    change: [+228346.10905112608% +229286.69325338802% +230319.25537997726%] (p = 0.00 < 0.05)
    Performance has regressed.
```
#### shell sort  (sorted array)
```
    time:   [743.07 ns 745.51 ns 748.16 ns]
    change: [+9664.82980375754% +9703.85326686511% +9745.880631940849%] (p = 0.00 < 0.05)
    Performance has regressed.
```

#### shake sort (not sorted array)
```
    time:   [798.76 us 800.30 us 801.93 us]
    change: [+10734128.094305376% +10792953.864785861% +10848771.380099326%] (p = 0.00 < 0.05)
    Performance has regressed.
```
#### shake sort  (sorted array)
```
    time:   [429.31 us 430.77 us 432.48 us]
    change: [+5299168.23151441% +5366038.124962242% +5436060.967865664%] (p = 0.00 < 0.05)
    Performance has regressed.
```

#### odd_even sort (not sorted array)
```
    time:   [665.06 us 671.85 us 679.42 us]
    change: [+9675758.004897878% +9791024.532760285% +9905317.305065522%] (p = 0.00 < 0.05)
    Performance has regressed.
```
#### odd_even sort  (sorted array)
```
    time:   [1.0750 us 1.0864 us 1.0972 us]
    change: [+14972.35749205396% +15110.73513291451% +15252.810122551899%] (p = 0.00 < 0.05)
    Performance has regressed.
```

comb sort wins witt result `time:   [656.31 ns 667.19 ns 677.67 ns]`