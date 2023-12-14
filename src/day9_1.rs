pub fn run(inp: String) -> String {
    let mut score = 0;
    for ele in inp.split('\n') {
        let mut sequences: Vec<Vec<i64>> = vec!(ele.split(' ').map(|x| x.parse().unwrap()).collect());
        loop {
            let current = sequences.last().unwrap();
            let mut all_zero = true;
            let mut result = Vec::with_capacity(current.len() - 1);
            for i in 1..current.len() {
                let to_add = current.get(i).unwrap() - current.get(i-1).unwrap();
                if to_add != 0 {
                    all_zero = false;
                }
                result.push(to_add);
            }
            if all_zero {
                break;
            }
            sequences.push(result);
        }
        let mut current = sequences.last().unwrap().clone();
        current.push(current.last().unwrap().clone());
        for depth in (0..sequences.len() - 1).rev() {
            let mut next = sequences.get(depth).unwrap().clone();
            next.push(next.last().unwrap() + current.last().unwrap());
            current = next;
        }
        score += current.last().unwrap();
    }

    return format!("{}", score);
}