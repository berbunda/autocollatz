fn check(n: u128) -> bool {
    print!("проверка n...");
    n % 2 == 0
}

fn main() {
    let x = u128::MAX;
    loop {
        for mut n in 1..x {
            println!("теперь n={}", n);
            loop {
                if n == 1 {
                    println!("прерывание внут n=1");
                    break;
                }
                while n > 1 {
                    if check(n) {
                        n = n / 2;
                        println!("поделили на 2. n={}", n);
                    } else {
                        n = n * 3 + 1;
                        println!("умножили на 3 и прибавили 1. n={}", n);
                    };
                };
                println!("прерывание внеш ");
            }
            println!("n={}", n)
        }
    }
}