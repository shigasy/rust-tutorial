use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    // &で参照するおかげでデータを複数回メモリにコピーせずとも同じデータにアクセスできる
    // 安全かつ簡単

    // read_lineの返却値はio::Result
    // もし、Resultにexpectメソッドを使用していなければ、コンパイルエラーを示す
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess)
}
