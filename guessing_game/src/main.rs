extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number); //秘密の数字は次の通り: {}
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // &で参照するおかげでデータを複数回メモリにコピーせずとも同じデータにアクセスできる
        // 安全かつ簡単

        // read_lineの返却値はio::Result
        // もし、Resultにexpectメソッドを使用していなければ、コンパイルエラーを示す
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Rustでの標準は、i32型であり、型情報をどこかに追加して、コンパイラに異なる数値型だと推論させない限り
        // u32 32ビットの非負数字
        // シャドーイング(shadowing)のおかげで別々の変数を2つ作らされることなく、guessという変数名を再利用することができる
        // trimで改行\nや両端の空白を消している
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // u32を指定してるけど、引いて負の数だとクラッシュした。コンパイルエラーは起きなかった。
        // let guess = guess - 1;

        // match式 めちゃええやん 全てのシチュエーションに対処するし、式だから定数で書ける
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // cmp 2値を比較
        // match式は、全てのシチュエーションに対処していることを保証するのを手助けしてくれる
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
