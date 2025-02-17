fn borrowing_doesnot_move_ownership() {
    let word = "UTRGV".to_string();
    let borrow_word = &word;
    println!("{} == {}", word, borrow_word);
}


fn print_string(w:& string){
    println("{}", w),
}