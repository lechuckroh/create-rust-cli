use assert_cmd::Command;

#[test]
fn runs() {
    // 현재 크레이트의 exif-rename 을 실행하는 Command를 생성합니다.
    // 실행 결과는 Result 를 반환하며, 실행 파일을 찾을 수 있기 때문에, `Result::unwrap`을 실행합니다.
    // 실행 파일을 찾지 못한 경우 패닉이 발생하면서 테스트가 실패합니다.
    let mut cmd = Command::cargo_bin("exif-rename").unwrap();
    // Assert::success 는 실행 종료 코드가 `0`인지 확인합니다.
    // `stdout` 함수를 사용해 STDOUT 에 출력된 문자열이 일치하는지 확인합니다.
    cmd.assert().success().stdout("Hello, world!\n");
}
