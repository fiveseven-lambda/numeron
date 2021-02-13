const M: usize = 4; // 文字の種類数
const N: usize = 3; // 桁数

type Number = [usize; N];

fn main() {
    let numbers: Vec<Number> = {
        let mut v = Vec::new();
        fn gen(vec: &mut Vec<Number>, array: &mut Number, bit: u32, index: usize) {
            if index < N {
                for i in 0..M {
                    if bit >> i & 1 == 0 {
                        array[index] = i;
                        gen(vec, array, bit | 1 << i, index + 1);
                    }
                }
            } else {
                vec.push(*array);
            }
        }
        gen(&mut v, &mut [0; N], 0, 0);
        v
    };
    let maps: Vec<_> = numbers
        .iter()
        .map(|number| {
            let mut map = std::collections::BTreeMap::new();
            for (i, other) in numbers.iter().enumerate() {
                let result = hit_blow(number, other);
                *map.entry(result).or_insert(0) |= 1 << i;
            }
            map
        })
        .collect();
    let table: Vec<Vec<_>> = maps
        .iter()
        .map(|map| map.iter().map(|(_, &bit)| bit).collect())
        .collect();
    let mut dp = vec![(0, std::f64::MAX); 1 << numbers.len()];
    let mut next = 0;

    for i in 0..dp.len() {
        if i > dp.len() * next / 100 {
            println!("{}%", next);
            next += 5;
        }

        'label: for (j, vec) in table.iter().enumerate() {
            let mut sum = 0.;
            for k in vec {
                let int = i & k;
                sum += if int == 1 << j {
                    0.
                } else if int == i {
                    continue 'label;
                } else {
                    dp[int].1 * int.count_ones() as f64
                };
            }
            let tmp = sum / i.count_ones() as f64 + 1.;
            if dp[i].1 > tmp {
                dp[i] = (j, tmp);
            }
        }
    }
    println!("期待値: {}", dp.last().unwrap().1);
    println!("最適な戦略: ");
    for number in &numbers {
        let mut possible = dp.len() - 1;
        let mut count = 0;
        print!("{}:", to_string(number));
        loop {
            let next = dp[possible].0;
            count += 1;
            let (hit, blow) = hit_blow(number, &numbers[next]);
            print!(" {}:{}H{}B", to_string(&numbers[next]), hit, blow);
            if hit == N {
                break;
            } else {
                print!(" ->");
            }
            match maps[next].get(&(hit, blow)) {
                Some(bit) => {
                    possible &= bit;
                }
                None => panic!()
            }
        }
        println!(" ({})", count);
    }
    /*
     * 実際にユーザの入力を受け取って
     * hit & blow をプレイ
    let mut possible = dp.len() - 1;
    while possible.count_ones() > 1 {
        println!("{}", dp[possible].1);
        let next = dp[possible].0;
        println!("{:?}", numbers[next]);
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let input: Vec<usize> = s.trim().split_whitespace().map(|e| e.parse().ok().unwrap()).collect();
        let hit = input[0];
        let blow = input[1];
        match maps[next].get(&(hit, blow)) {
            Some(bit) => {
                possible &= bit;
            }
            None => {
                panic!();
            }
        }
    }
    println!("{:?}", numbers[possible.trailing_zeros() as usize]);
    */
}

fn hit_blow(left: &Number, right: &Number) -> (usize, usize) {
    let mut left: Vec<_> = left.iter().enumerate().map(|(x, &y)| (y, x)).collect();
    left.sort();
    let mut right: Vec<_> = right.iter().enumerate().map(|(x, &y)| (y, x)).collect();
    right.sort();

    let mut ret = (0, 0);

    let mut i = 0;
    let mut j = 0;
    while i < N && j < N {
        use std::cmp::Ordering;
        match left[i].0.cmp(&right[j].0) {
            Ordering::Less => i += 1,
            Ordering::Greater => j += 1,
            Ordering::Equal => {
                if left[i].1 == right[j].1 {
                    ret.0 += 1;
                } else {
                    ret.1 += 1;
                }
                i += 1;
                j += 1;
            }
        }
    }
    ret
}

fn to_string(n: &Number) -> String {
    format!("{}{}{}", n[0], n[1], n[2])
}
