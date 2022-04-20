// #[tokio::test(flavor = "multi_thread")]
// async fn test_user_subcount() {
//     let api = NcmApi::default();
//     let resp = api.user_subcount().await;
//     assert!(resp.is_ok());
//
//     let res = resp.unwrap();
//     let res = res.deserialize_to_implict();
//     assert_eq!(res.code, 200);
// }
