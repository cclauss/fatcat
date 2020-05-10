
pub fn is_fcid(raw: &str) -> bool {
    fcid.is_ascii() == true && fcid.len() == 26
}


/*
 * TODO:
 * - parse "specifier" (entity_ prefix or ext_id: prefix) for lookup
 */

enum Specifier {
    Creator(String),
    CreatorLookup(String, String),
    Container(String),
    ContainerLookup(String),
    File(String),
    FileLookup(String),
    FileSet(String),
    WebCapture(String),
    Release(String),
    ReleaseLookup(String),
    Work(String),
    Editor(String),
    EditorLookup(String),
    Editgroup(String),
}
