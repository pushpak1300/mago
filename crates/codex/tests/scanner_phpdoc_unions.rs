#![allow(clippy::unwrap_used)]

use std::borrow::Cow;

use mago_allocator::LocalArena;
use mago_codex::scanner::scan_program;
use mago_codex::ttype::union::TUnion;
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_php_version::PHPVersion;
use mago_syntax::parser::parse_file;

fn parameter_type(docblock_type: &str) -> TUnion {
    let source = format!("<?php\n/** @param {docblock_type} $value */\nfunction f(mixed $value): void {{}}\n");
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(b"unions.php"), Cow::Owned(source.into_bytes()));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors(), "{:?}", program.errors);
    let names = NameResolver::new(&arena).resolve(program);
    let metadata = scan_program(&arena, &file, program, &names, PHPVersion::LATEST);

    let function = metadata.get_function(b"f").unwrap();
    let parameter = function.parameters.first().unwrap();

    parameter.type_metadata.as_ref().unwrap().type_union.clone()
}

#[test]
fn long_union_is_built_without_overflowing_the_stack() {
    let members = (0..150).map(|value| value.to_string()).collect::<Vec<_>>().join("|");
    let union = parameter_type(&members);

    assert_eq!(union.types.len(), 150);
    assert!(!union.is_nullable());
}

#[test]
fn long_union_with_repeated_null_is_built_without_overflowing_the_stack() {
    let members = (0..75).map(|value| format!("{value}|null")).collect::<Vec<_>>().join("|");
    let union = parameter_type(&members);

    assert_eq!(union.types.len(), 76);
    assert!(union.is_nullable());
}
