#[allow(dead_code)]
fn time<T, F>(fun: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> T,
{
    let now = std::time::Instant::now();
    let ret = fun();
    let elapsed = now.elapsed();
    (ret, elapsed)
}

#[allow(dead_code)]
fn read_stdin() -> String {
    use std::io::Read;
    let mut string = String::new();
    std::io::stdin().lock().read_to_string(&mut string).unwrap();
    string
}
