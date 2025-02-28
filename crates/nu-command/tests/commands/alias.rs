use nu_test_support::fs::Stub::FileWithContentToBeTrimmed;
use nu_test_support::playground::Playground;
use nu_test_support::{nu, pipeline};

#[ignore = "TODO?: Aliasing parser keywords does not work anymore"]
#[test]
fn alias_simple() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            alias bar = use sample_def.nu greet;
            bar;
            greet
        "#
    ));

    assert_eq!(actual.out, "hello");
}

#[ignore = "TODO?: Aliasing parser keywords does not work anymore"]
#[test]
fn alias_hiding_1() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            overlay use ./activate-foo.nu;
            $nu.scope.aliases | find deactivate-foo | length
        "#
    ));

    assert_eq!(actual.out, "1");
}

#[ignore = "TODO?: Aliasing parser keywords does not work anymore"]
#[test]
fn alias_hiding_2() {
    let actual = nu!(
        cwd: "tests/fixtures/formats", pipeline(
        r#"
            overlay use ./activate-foo.nu;
            deactivate-foo;
            $nu.scope.aliases | find deactivate-foo | length
        "#
    ));

    assert_eq!(actual.out, "0");
}

#[test]
fn alias_fails_with_invalid_name() {
    let err_msg = "name can't be a number, a filesize, or contain a hash # or caret ^";
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
            alias 1234 = echo "test"
        "#
    ));
    assert!(actual.err.contains(err_msg));

    let actual = nu!(
        cwd: ".", pipeline(
        r#"
            alias 5gib = echo "test"
        "#
    ));
    assert!(actual.err.contains(err_msg));

    let actual = nu!(
        cwd: ".", pipeline(
        r#"
            alias "te#t" = echo "test"
        "#
    ));
    assert!(actual.err.contains(err_msg));

    let actual = nu!(
        cwd: ".", pipeline(
        r#"
            alias ^foo = echo "bar"
        "#
    ));
    assert!(actual.err.contains(err_msg));
}

#[test]
fn cant_alias_keyword() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
            alias ou = let
        "#
    ));
    assert!(actual.err.contains("cant_alias_keyword"));
}

#[test]
fn alias_wont_recurse() {
    let actual = nu!(
        cwd: ".", pipeline(
        r#"
            module myspamsymbol {
                export def myfoosymbol [prefix: string, msg: string] {
                    $prefix + $msg
                }
            };
            use myspamsymbol myfoosymbol;
            alias myfoosymbol = myfoosymbol 'hello';
            myfoosymbol ' world'
        "#
    ));

    assert_eq!(actual.out, "hello world");
    assert!(actual.err.is_empty());
}

// Issue https://github.com/nushell/nushell/issues/8246
#[test]
fn alias_wont_recurse2() {
    Playground::setup("alias_wont_recurse2", |dirs, sandbox| {
        sandbox.with_files(vec![FileWithContentToBeTrimmed(
            "spam.nu",
            r#"
                def eggs [] { spam 'eggs' }
                alias spam = spam 'spam'
            "#,
        )]);
        let actual = nu!(
            cwd: dirs.test(), pipeline(
            r#"
                def spam [what: string] { 'spam ' + $what };
                source spam.nu;
                spam
            "#
        ));

        assert_eq!(actual.out, "spam spam");
        assert!(actual.err.is_empty());
    })
}
