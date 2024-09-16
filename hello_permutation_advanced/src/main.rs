use std::usize;

#[derive(Debug)]
struct PartialPermutation<const N: usize = 4> {
    data: [usize; N],
    left: usize,
    is_init: bool,
}

impl Default for PartialPermutation<4> {
    fn default() -> Self {
        PartialPermutation::<4>::new(4)
    }
}

impl<const N: usize> PartialPermutation<N> {
    pub fn new(digit: usize) -> Self {
        let mut data = [0; N];
        data.copy_from_slice(&(0..N).collect::<Vec<usize>>());

        Self {
            data,
            left: N - digit.min(N),
            is_init: true,
        }
    }

    pub fn next_from(&self, digit: usize) -> Option<Self> {
        let data = self.data;
        let mut temp = Self {
            data,
            left: 0,
            is_init: false,
        };
        let Some(_) = temp.next() else {
            return None;
        };

        Some(Self {
            data: temp.data,
            left: N - digit.min(N),
            is_init: true,
        })
    }

    pub fn last_from(&self) -> Option<Self> {
        let mut data = [0; N];

        // 先頭部分をコピー
        (0..self.left).for_each(|index| data[index] = self.data[index]);

        // 走査部分の最後の並びをコピー
        let mut tail_iterator = self.data[self.left..N].iter().rev();
        (self.left..N).for_each(|index| {
            let value = tail_iterator.next().unwrap();
            data[index] = *value;
        });

        Some(Self {
            data,
            left: self.left,
            is_init: false,
        })
    }

    pub fn next(&mut self) -> Option<&[usize]> {
        if self.is_init {
            self.is_init = false;
            return Some(&self.data);
        }

        let mut last = self.data.len() - 1;
        let mut pivot = last - 1;

        // 逆順にソート済みになってない場所を見つけて
        while self.data[pivot] > self.data[pivot + 1] {
            if pivot <= self.left {
                // 樹形図の末端まで到達していた
                return None;
            }
            pivot -= 1;
        }

        // 値を入れ替えて
        let mut second = last;
        while self.data[pivot] > self.data[second] {
            second -= 1;
        }
        self.data.swap(pivot, second);

        // 値を入れ替えた場所以降は逆順にソート済みなので reverse すると新たな木に突入する
        // reverse
        let mut swap_pivot = pivot + 1;
        while swap_pivot < last {
            self.data.swap(swap_pivot, last);
            swap_pivot += 1;
            last -= 1;
        }

        Some(&self.data)
    }
}

fn main() {
    let mut partial_permutation = PartialPermutation::<4>::new(3);
    let last = partial_permutation.last_from().unwrap();
    while let Some(indicies) = partial_permutation.next() {
        println!("{:?}", indicies);
    }

    println!("============");
    println!("{:?}", last);
    println!("============");

    let mut next_partial_permutation = partial_permutation.next_from(3).unwrap();
    while let Some(indicies) = next_partial_permutation.next() {
        println!("{:?}", indicies);
    }
}
