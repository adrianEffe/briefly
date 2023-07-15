use briefly::routes::health_check;

#[tokio::test]
async fn test_is_even() {
    let x = 10;
    health_check();
    print!("testing");
    assert_eq!(10, x);
}
