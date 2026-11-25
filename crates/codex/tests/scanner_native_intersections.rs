#![allow(clippy::unwrap_used)]

use std::borrow::Cow;

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::scanner::scan_program;
use mago_database::file::File;
use mago_names::resolver::NameResolver;
use mago_php_version::PHPVersion;
use mago_syntax::parser::parse_file;

fn scan(source: &str) -> CodebaseMetadata {
    let arena = LocalArena::new();
    let file = File::ephemeral(Cow::Borrowed(b"intersections.php"), Cow::Owned(source.as_bytes().to_vec()));
    let program = parse_file(&arena, &file);
    assert!(!program.has_errors(), "{:?}", program.errors);
    let names = NameResolver::new(&arena).resolve(program);

    scan_program(&arena, &file, program, &names, PHPVersion::LATEST)
}

fn parameter_type_is_never(metadata: &CodebaseMetadata, function: &[u8]) -> bool {
    let function = metadata.get_function(function).unwrap();
    let parameter = function.parameters.first().unwrap();

    parameter.type_metadata.as_ref().unwrap().type_union.is_never()
}

#[test]
fn intersection_without_intersectable_members_is_never() {
    let metadata = scan("<?php\nfunction f(int&string $value): void {}\n");

    assert!(parameter_type_is_never(&metadata, b"f"));
}

#[test]
fn intersection_with_one_non_intersectable_member_is_never() {
    let metadata = scan("<?php\nfunction f(int&Countable $value): void {}\n");

    assert!(parameter_type_is_never(&metadata, b"f"));
}

#[test]
fn intersection_of_objects_is_kept() {
    let metadata = scan("<?php\nfunction f(Countable&Traversable $value): void {}\n");

    assert!(!parameter_type_is_never(&metadata, b"f"));
}
