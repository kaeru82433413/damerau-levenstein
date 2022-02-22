#[derive(Clone, Debug)]
enum Operation {
    Unset, // 計算前
    Empty, // 空文字列同士(ベースケース)
    Keep, // 何もしない
    Insertion, // 挿入
    Deletion, // 削除
    Substitution, // 置換
    Transposition, // 隣接文字交換
}

impl Operation {
    fn from(&self, i: &mut usize, j: &mut usize) {
        let (i_sub, j_sub) = match *self {
            Self::Empty => (0, 0),
            Self::Keep => (1, 1),
            Self::Insertion => (0, 1),
            Self::Deletion => (1, 0),
            Self::Substitution => (1, 1),
            Self::Transposition => (2, 2),
            _ => panic!(),
        };
        *i -= i_sub;
        *j -= j_sub;
    }
}

type DpTable = Vec<Vec<(usize, Operation)>>;

fn main() {
    let (s, t) = input();
    let s_vec: Vec<_> = s.chars().collect();
    let t_vec: Vec<_> = t.chars().collect();
    let dp = calculate(&s_vec, &t_vec);
    println!("{} -> {}", s, t);
    println!("Distance: {}", dp.last().unwrap().last().unwrap().0);

    let (s_diff, t_diff) = restore(&dp);
    println!("{}", s);
    println!("{}", s_diff.iter().map(|&b| if b {'^'} else {' '}).collect::<String>());
    println!("{}", t);
    println!("{}", t_diff.iter().map(|&b| if b {'^'} else {' '}).collect::<String>());
}

fn calculate(s: &[char], t: &[char]) -> DpTable {
    let (n, m) = (s.len(), t.len());
    let mut dp = vec![vec![(n.max(m), Operation::Unset); m+1]; n+1];
    
    for i in 0..=n {
        for j in 0..=m {
            if (i, j) == (0, 0) {
                dp[i][j] = (0, Operation::Empty);
                continue;
            }

            if i > 0 {
                if dp[i][j].0 > dp[i-1][j].0+1 {
                    dp[i][j] = (dp[i-1][j].0+1, Operation::Deletion);
                }
            }
            if j > 0 {
                if dp[i][j].0 > dp[i][j-1].0+1 {
                    dp[i][j] = (dp[i][j-1].0+1, Operation::Insertion);
                }
            }
            if i > 0 && j > 0 {
                if s[i-1] == t[j-1] {
                    if dp[i][j].0 > dp[i-1][j-1].0 {
                        dp[i][j] = (dp[i-1][j-1].0, Operation::Keep);
                    }
                } else {
                    if dp[i][j].0 > dp[i-1][j-1].0+1 {
                        dp[i][j] = (dp[i-1][j-1].0+1, Operation::Substitution);
                    }
                }
            }
            if i > 1 && j > 1 && (s[i-1], s[i-2]) == (t[j-2], t[j-1]) {
                if dp[i][j].0 > dp[i-2][j-2].0+1 {
                    dp[i][j] = (dp[i-2][j-2].0+1, Operation::Transposition);
                }
            }
        }
    }
    dp
}

fn restore(dp: &DpTable) -> (Vec<bool>, Vec<bool>) {
    let size = (dp.len()-1, dp.last().unwrap().len()-1);
    let mut res = (vec![false; size.0], vec![false; size.1]);
    let (mut i, mut j) = size;

    while (i, j) != (0, 0) {
        match dp[i][j].1 {
            Operation::Insertion => {
                if size.0 > 0 { res.0[i.min(size.0-1)] = true; }
                res.1[j-1] = true;
            },
            Operation::Deletion => {
                res.0[i-1] = true;
                if size.1 > 0 { res.1[j.min(size.1-1)] = true; }
            },
            Operation::Substitution => {
                res.0[i-1] = true;
                res.1[j-1] = true;
            },
            Operation::Transposition => {
                res.0[i-1] = true; res.0[i-2] = true;
                res.1[j-1] = true; res.1[j-2] = true;
            },
            _ => (),
        }
        dp[i][j].1.from(&mut i, &mut j);
    }
    res
}

fn input() -> (String, String) {
    use std::env;
    use std::io::stdin;

    let mut args = env::args();
    if let Some(a) = args.by_ref().skip(1).next() {
        if let Some(b) = args.next() {
            return (a, b);
        }
    }

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let a = input.trim().into();
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let b = input.trim().into();
    (a, b)
}