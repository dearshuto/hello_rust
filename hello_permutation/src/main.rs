fn permutation(data: &mut Vec<u8>) -> bool {
    let mut last = data.len() - 1;
    let mut pivot = last - 1;

    // 逆順にソート済みになってない場所を見つけて
    while data[pivot] > data[pivot + 1] {
        if pivot == 0 {
            // 樹形図の末端まで到達していた
            return false;
        }
        pivot -= 1;
    }

    // 値を入れ替えて
    let mut second = last;
    while data[pivot] > data[second] {
        second -= 1;
    }
    data.swap(pivot, second);

    // 値を入れ替えた場所以降は逆順にソート済みなので reverse すると新たな木に突入する
    // reverse
    let mut swap_pivot = pivot + 1;
    while swap_pivot < last {
        data.swap(swap_pivot, last);
        swap_pivot += 1;
        last -= 1;
    }

    true
}

fn main() {
    let mut data = vec![0, 1, 2, 3];

    loop {
        println!("{:?}", data);
        let has_next = permutation(&mut data);
        if !has_next {
            break;
        }
    }
}
