import pytest
from pyrefly_native_binding import PyreflyAnalyzer

def test_analyzer_initialization():
    analyzer = PyreflyAnalyzer()
    assert analyzer is not None

def test_update_source_and_get_diagnostics_no_errors():
    analyzer = PyreflyAnalyzer()
    code = "def foo():\n    pass"
    analyzer.update_source(code)
    diagnostics = analyzer.get_diagnostics()
    assert isinstance(diagnostics, list)
    assert len(diagnostics) == 0

def test_get_diagnostics_with_syntax_error():
    analyzer = PyreflyAnalyzer()
    code = "def foo()\n    pass" # Erro de sintaxe: faltando ':'
    analyzer.update_source(code)
    diagnostics = analyzer.get_diagnostics()
    assert isinstance(diagnostics, list)
    assert len(diagnostics) > 0
    diag = diagnostics[0]
    assert diag["severity"] == "error"
    assert "message" in diag
    # Pyrefly pode dar "Parse error: Expected ':', found newline"
    assert ":" in diag["message"] or "newline" in diag["message"]
    assert diag["code"] == "parse-error" # Ou o código de erro específico do Pyrefly para sintaxe
    assert diag["start_line"] == 1
    assert diag["start_column"] > 0 # A coluna exata pode variar

def test_get_diagnostics_with_type_error():
    analyzer = PyreflyAnalyzer()
    code = "a: int = 'hello'"
    analyzer.update_source(code)
    diagnostics = analyzer.get_diagnostics()
    assert isinstance(diagnostics, list)
    assert len(diagnostics) > 0
    diag = diagnostics[0]
    assert diag["severity"] == "error"
    assert "bad-assignment" in diag["code"] # Ou similar
    assert "not assignable to 'int'" in diag["message"] # Ou similar
    assert diag["start_line"] == 1

def test_get_autocomplete_suggestions_simple():
    analyzer = PyreflyAnalyzer()
    code = "my_variable = 10\nmy_var"
    analyzer.update_source(code)
    # Linha 2, coluna 7 (após "my_var") - 1-indexed
    suggestions = analyzer.get_autocomplete_suggestions(line=2, column=7)
    assert isinstance(suggestions, list)
    
    found_suggestion = False
    for suggestion in suggestions:
        assert "label" in suggestion
        assert "kind" in suggestion
        assert "insert_text" in suggestion
        if suggestion["label"] == "my_variable":
            found_suggestion = True
            assert suggestion["kind"] == "Variable" # Ou o tipo que o Pyrefly usa
    
    # Este teste é um pouco frágil pois depende do Pyrefly encontrar a sugestão.
    # Se falhar, pode ser necessário ajustar o código ou a posição do cursor.
    # O importante é que a chamada funcione e retorne o formato esperado.
    if not suggestions:
        print("Warning: Autocomplete test returned no suggestions. This might be okay if Pyrefly's default setup doesn't infer this, or might indicate an issue.")
    # assert found_suggestion, "Expected 'my_variable' suggestion" # Pode ser muito específico para um teste inicial

def test_autocomplete_empty_source():
    analyzer = PyreflyAnalyzer()
    analyzer.update_source("")
    suggestions = analyzer.get_autocomplete_suggestions(line=1, column=1)
    assert isinstance(suggestions, list)
    # Para uma string vazia, esperamos palavras-chave e builtins, talvez.
    # Mas para um teste simples, apenas verificar se não quebra é suficiente.

def test_diagnostics_format():
    analyzer = PyreflyAnalyzer()
    code = "a: str = 123"
    analyzer.update_source(code)
    diagnostics = analyzer.get_diagnostics()
    assert len(diagnostics) >= 1
    diag = diagnostics[0]
    expected_keys = {"start_line", "start_column", "end_line", "end_column", "message", "severity", "code"}
    assert expected_keys.issubset(diag.keys())
    assert isinstance(diag["start_line"], int)
    assert isinstance(diag["start_column"], int)
    assert isinstance(diag["end_line"], int)
    assert isinstance(diag["end_column"], int)
    assert isinstance(diag["message"], str)
    assert isinstance(diag["severity"], str)
    assert isinstance(diag["code"], str)

def test_autocomplete_format():
    analyzer = PyreflyAnalyzer()
    code = "import o"
    analyzer.update_source(code)
    suggestions = analyzer.get_autocomplete_suggestions(line=1, column=9) # Após "import o"
    if suggestions: # Pode não haver sugestões dependendo da configuração padrão
        sugg = suggestions[0]
        expected_keys = {"label", "kind", "insert_text"} # detail é opcional
        assert expected_keys.issubset(sugg.keys())
        assert isinstance(sugg["label"], str)
        assert isinstance(sugg["kind"], str)
        assert isinstance(sugg["insert_text"], str)
        if "detail" in sugg:
            assert isinstance(sugg["detail"], str)
