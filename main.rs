fn pascal_line(n: usize) -> Vec<usize> {
    match n {
        0 => vec![1],
        1 => vec![1, 1],
        2 => vec![1, 2, 1],
        _ => {
            let mut past_line: Vec<usize> = vec![1, 2, 1];
            for i in 3..n+1 {
                let mut current_line: Vec<usize> = Vec::new();
                current_line.push(1);
                current_line.push(i);
                let unique_indices = (i+1)/2 + (i+1)%2;
                for j in 2..unique_indices {
                    match j < past_line.len() {
                        true => current_line.push(past_line[j] + past_line[j-1]),
                        false => current_line.push(past_line[j-1] * 2),
                    }
                }
                past_line = current_line;
            }
            let mut other = past_line.clone();
            other.reverse();
            if (n+1) % 2 == 1 {
                other.remove(0);
            }
            past_line.append(&mut other);
            past_line
        }
    }
}
