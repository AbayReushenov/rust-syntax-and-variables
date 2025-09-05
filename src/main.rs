// Метки циклов для устранения неоднозначности между несколькими циклами
// Если у вас есть циклы внутри циклов, break и continue применяются к самому внутреннему циклу в этой цепочке.
// При желании вы можете создать метку цикла, которую вы затем сможете использовать с break или continue для указания,
// что эти ключевые слова применяются к помеченному циклу, а не к самому внутреннему циклу.
// Метки цикла должны начинаться с одинарной кавычки.
// Вот пример с двумя вложенными циклами:


fn main() {
    let mut count = 0;

    // Метки цикла должны начинаться с одинарной кавычки.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    
    println!("End count = {count}");
}
// count = 0
// remaining = 10
// remaining = 9
// count = 1
// remaining = 10
// remaining = 9
// count = 2
// remaining = 10
// End count = 2
