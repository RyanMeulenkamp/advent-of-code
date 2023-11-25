
#[tokio::main]
async fn other_main() {

}

#[tokio::main]
async fn main() {
    let henk = other_main();
    println!("henk: {henk:?}")
}
