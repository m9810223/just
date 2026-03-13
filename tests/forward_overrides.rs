use super::*;

#[test]
fn just_overrides_not_set_without_forward_overrides() {
  Test::new()
    .justfile(
      r#"
FOO := "default"

foo:
  echo ${JUST_OVERRIDES-UNSET}
"#,
    )
    .args(["--set", "FOO", "bar", "foo"])
    .stderr_regex(r".*echo \$\{JUST_OVERRIDES-UNSET\}.*\n")
    .stdout("UNSET\n")
    .success();
}

#[test]
fn just_overrides_set_with_forward_overrides_flag() {
  Test::new()
    .justfile(
      r#"
FOO := "default"

foo:
  echo ${JUST_OVERRIDES-UNSET}
"#,
    )
    .args(["--forward-overrides", "--set", "FOO", "bar", "foo"])
    .stderr_regex(r".*echo \$\{JUST_OVERRIDES-UNSET\}.*\n")
    .stdout("FOO=bar\n")
    .success();
}

#[test]
fn just_overrides_set_with_justfile_setting() {
  Test::new()
    .justfile(
      r#"
set forward-overrides

FOO := "default"

foo:
  echo ${JUST_OVERRIDES-UNSET}
"#,
    )
    .args(["--set", "FOO", "bar", "foo"])
    .stderr_regex(r".*echo \$\{JUST_OVERRIDES-UNSET\}.*\n")
    .stdout("FOO=bar\n")
    .success();
}

#[test]
fn just_overrides_not_set_without_overrides() {
  Test::new()
    .justfile(
      r#"
foo:
  echo ${JUST_OVERRIDES-UNSET}
"#,
    )
    .args(["--forward-overrides", "foo"])
    .stderr_regex(r".*echo \$\{JUST_OVERRIDES-UNSET\}.*\n")
    .stdout("UNSET\n")
    .success();
}

#[test]
fn just_overrides_multiple_overrides() {
  Test::new()
    .justfile(
      r#"
set forward-overrides

A := "default"
B := "default"

foo:
  echo ${JUST_OVERRIDES-UNSET}
"#,
    )
    .args(["--set", "A", "1", "--set", "B", "2", "foo"])
    .stderr_regex(r".*echo \$\{JUST_OVERRIDES-UNSET\}.*\n")
    .stdout_regex("(A=1\x1FB=2|B=2\x1FA=1)\n")
    .success();
}
