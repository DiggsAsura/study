/* Iterating over Results
 * =======================
 *
 * An Iter::map operation might fail, for example:
 */
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);


    ex2();
    ex3();
    ex4();
    ex5();
    ex6();
}

/* Let's step through strategies for handling this. 
 *
 *
 * Ignore the failed items with filter_map()
 * ---------------------------------------------
 *
 * filter_map calles a function and filters out the results that are None. */

 fn ex2() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);
}


/* Collect the failed items with map_err() and filter_map()
 * ---------------------------------------------------------
 *
 * map_err calls a function with the error, so by adding that to the previous filter_map solution
 * we can save them off to the side while iterating. */

fn ex3() {
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}


/* Fail the entire operation with collect()
 * -------------------------------------------
 *
 * Result implements FromIter so that a vector of results (Vec<Result<T, E>>) can be turned
 * into a result with a vector (Result<Vec<T>, E>). Once a Result::Err is found, the iteration will
 * terminate. */

fn ex4() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}

/* This same technique can be used with Option. */


/* Collect all valid values and failures with partition() 
 * --------------------------------------------------------
 */

fn ex5() {
    println!("ex5:");
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

/* When you look at the results, you'll note that everything is still wrapped in Result. A little
 * more boilerplate is needed for this. */
fn ex6() {
    println!("ex6:");
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
