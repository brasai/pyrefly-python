use lsp_types::CompletionItemKind as CIK;
use lsp_types::DiagnosticSeverity;

// Mapeia o i32 de severidade do Pyrefly Playground para strings
pub fn map_severity_to_str(severity_code: i32) -> &'static str {
    // O playground.rs do Pyrefly usa severity: 8 para erros,
    // que corresponde a lsp_types::MarkerSeverity::Error.
    // Outros valores de DiagnosticSeverity são:
    // ERROR = 1, WARNING = 2, INFORMATION = 3, HINT = 4.
    // Vamos mapear o que o Pyrefly provavelmente usa.
    match severity_code {
        1 => "error",   // lsp_types::DiagnosticSeverity::ERROR
        2 => "warning", // lsp_types::DiagnosticSeverity::WARNING
        3 => "information", // lsp_types::DiagnosticSeverity::INFORMATION
        4 => "hint",    // lsp_types::DiagnosticSeverity::HINT
        8 => "error",   // Usado em playground.rs (provavelmente de MarkerSeverity)
        _ => "unknown",
    }
}

// Mapeia lsp_types::CompletionItemKind para strings
pub fn map_completion_kind_to_str(kind: Option<CIK>) -> &'static str {
    match kind {
        Some(CIK::TEXT) => "Text",
        Some(CIK::METHOD) => "Method",
        Some(CIK::FUNCTION) => "Function",
        Some(CIK::CONSTRUCTOR) => "Constructor",
        Some(CIK::FIELD) => "Field",
        Some(CIK::VARIABLE) => "Variable",
        Some(CIK::CLASS) => "Class",
        Some(CIK::INTERFACE) => "Interface",
        Some(CIK::MODULE) => "Module",
        Some(CIK::PROPERTY) => "Property",
        Some(CIK::UNIT) => "Unit",
        Some(CIK::VALUE) => "Value",
        Some(CIK::ENUM) => "Enum",
        Some(CIK::KEYWORD) => "Keyword",
        Some(CIK::SNIPPET) => "Snippet",
        Some(CIK::COLOR) => "Color",
        Some(CIK::FILE) => "File",
        Some(CIK::REFERENCE) => "Reference",
        Some(CIK::FOLDER) => "Folder",
        Some(CIK::ENUM_MEMBER) => "EnumMember",
        Some(CIK::CONSTANT) => "Constant",
        Some(CIK::STRUCT) => "Struct",
        Some(CIK::EVENT) => "Event",
        Some(CIK::OPERATOR) => "Operator",
        Some(CIK::TYPE_PARAMETER) => "TypeParameter",
        None => "Unknown",
        _ => "Other", // Para cobrir quaisquer variantes futuras não listadas explicitamente
    }
}
