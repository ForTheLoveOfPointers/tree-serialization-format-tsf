# Tree Serialization Format (TSF) v0.1

## 1. Objetivo

TSF es un formato textual para serializar árboles ordenados.

No es un lenguaje de marcado ni un formato de presentación.

Su único propósito es representar estructuras jerárquicas de forma compacta, determinista y eficiente para transmisión, almacenamiento y reconstrucción.

---

## 2. Objetivos de diseño

### 2.1 Requisitos

* Parseo O(n).
* Streaming-friendly.
* Sin tags de cierre.
* Sin backtracking.
* Sin ambigüedad sintáctica.
* Una única representación válida por árbol.
* Fácil generación automática.

### 2.2 No objetivos

* Legibilidad humana máxima.
* Compatibilidad HTML.
* Comentarios embebidos.
* Tolerancia a errores.

---

## 3. Modelo de datos

Un documento TSF representa un árbol ordenado.

Cada nodo contiene:

* Nombre.
* Conjunto de atributos.
* Contenido opcional.
* Hijos ordenados.


---

## 4. Estructura general

Cada línea representa exactamente un nodo.

La profundidad se expresa mediante prefijo numérico.

Ejemplo:

0 html
1 head
2 title "My page"
1 body
2 h1 "Hello"
2 div class="container"

Árbol equivalente:

html
├─ head
│  └─ title
└─ body
├─ h1
└─ div

---

## 5. Gramática

depth SP name [SP attributes] [SP content]

Donde:

depth := integer >= 0
name := identifier
attributes := attribute*
content := string

---

## 6. Reglas de profundidad

La raíz debe comenzar en profundidad 0.

La profundidad sólo puede:

* mantenerse,
* aumentar en 1,
* disminuir arbitrariamente.

Válido:

0 root
1 child
2 grandchild
1 sibling

Inválido:

0 root
2 child

Error:

UnexpectedDepth

---

## 7. Identificadores

Sintaxis:

[a-zA-Z_][a-zA-Z0-9_-]*

Ejemplos válidos:

div
header
main-content
user_1

---

## 8. Atributos

Formato:

key=value

Ejemplos:

2 div class="container"
2 button disabled=true
2 item count=42

Múltiples atributos:

2 button class="primary" disabled=true

---

## 9. Tipos de valores

### String

"name"

### Integer

42

### Float

3.14

### Boolean

true
false

### Null

null

---

## 10. Contenido textual

El contenido siempre aparece al final.

Ejemplo:

2 title "My page"

Interpretación:

name = title
content = "My page"

---

## 11. Escape de cadenas

Caracteres especiales:

"    comilla
\    barra invertida
\n    salto de línea
\t    tabulación

Ejemplo:

2 text "hello\nworld"

---

## 12. Reconstrucción del árbol

El parser mantiene:

stack<Node>
current_depth

Para cada línea:

1. Leer profundidad.
2. Mientras stack.len() > depth:
   pop()
3. Crear nodo.
4. Agregar nodo como hijo del top actual.
5. Push nodo.

Complejidad:

Tiempo: O(n)
Memoria: O(max_depth)

---

## 13. Serialización canónica

Un árbol dado posee exactamente una representación textual válida.

Esto permite:

* hashing determinista,
* diffs reproducibles,
* firmas digitales.

---

## 14. Extensiones futuras

### Namespace

2 svg:circle radius=10

### Referencias

2 node ref=123

### IDs compactos

2 #45

### Codificación binaria

Versión TSF-B.

Mantiene exactamente el mismo modelo de datos.

---

## 15. Implementación de referencia

Componentes mínimos:

* Lexer
* Parser
* AST
* Serializer
* Validator

No se requiere motor de renderizado.

TSF define únicamente la representación de árboles.

---

## 16. Plan de implementación del parser (feature/parser)

### 16.1 Archivos a crear/modificar

| Archivo | Acción |
|---------|--------|
| `src/parser.rs` | Crear — declaración del módulo: `pub mod parser_types;` |
| `src/parser/parser_types.rs` | Crear — `Parser`, `ParseError`, lógica completa |
| `src/errors.rs` | Crear — declaración del módulo: `pub mod error_types;` |
| `src/errors/error_types.rs` | Crear — `ParseError` y demás tipos de error |
| `src/main.rs` | Modificar — usar `Parser::parse()` en lugar del bucle de tokens |

### 16.2 `ParseError` (src/errors/error_types.rs)

```rust
pub enum ParseError {
    IoError(std::io::Error),
    DepthJump { current_depth: u32, target_depth: u32 },
    UnexpectedToken { expected: String, found: Token },
    InvalidValue { raw: String },
    EmptyInput,
}
```

Implementa `fmt::Display` y `std::error::Error`.

### 16.3 `Parser<R: BufRead>` (src/parser/parser_types.rs)

```rust
pub struct Parser<R: BufRead> {
    lex: Lexer<R>,
    lookahead: Option<Token>,
}
```

**Métodos públicos:**
- `Parser::new(reader: R) -> Self` — construye el parser con un token de lookahead.
- `Parser::parse(&mut self) -> Result<Document, ParseError>` — consume toda la entrada y construye el árbol.

**Métodos privados:**
- `peek(&self) -> Option<&Token>` — lookahead de un token sin consumir.
- `advance(&mut self) -> Token` — consume y retorna el lookahead, cargando el siguiente.
- `expect_identifier(&mut self) -> Result<String, ParseError>` — consume un `Identifier` o error.
- `parse_line(&mut self) -> Result<(u32, Node), ParseError>` — una línea completa: `Depth name [attrs] [content] NewLine`.
- `parse_attributes(&mut self) -> Result<Vec<Attribute>, ParseError>` — cero o más `Identifier Assign Value`.
- `parse_value(&mut self) -> Result<Value, ParseError>` — `Text` → String; `Identifier` → Integer/Float/Boolean/Null o error.

### 16.4 Algoritmo de reconstrucción del árbol (sección 12)

```
stack = []
loop:
    if peek es Eof → break
    (depth, node) = parse_line()
    if depth > stack.len() → error DepthJump
    while stack.len() > depth → pop
    if let Some(parent) = stack.last_mut() → parent.children.push(node)
    stack.push(node)
return Document { root: stack[0] }
```

**Validaciones:**
- Primera línea debe tener depth 0 → si no, `DepthJump { 0, depth }`.
- Depth sólo puede aumentar en 1 → `depth > stack.len()` es salto inválido.
- Después de `=`, `Text` → `Value::String`; `Identifier` se parsea como entero, luego flotante, luego booleano, luego null, o error.

### 16.5 Cambios en main.rs

```rust
fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut parser = Parser::new(reader);
    match parser.parse() {
        Ok(doc) => println!("{}", doc),
        Err(e) => eprintln!("{}", e),
    }
}
```

### 16.6 Tests

Siguiendo el estilo de `syntax_tree_types.rs` (`#[cfg(test)] mod tests`):

| Test | Descripción |
|------|-------------|
| `parse_root_only` | `"0 html\n"` → Document con un solo nodo |
| `parse_nested_tree` | Árbol completo → coincide con AST esperado |
| `parse_attributes` | Línea con `key=value` y `key="string"` |
| `parse_content` | Línea con contenido textual al final |
| `parse_roundtrip` | Serializar AST, parsear resultado, comparar |
| `parse_error_empty` | `""` → `EmptyInput` |
| `parse_error_depth` | `"2 root\n"` → `DepthJump` |
| `parse_error_depth_jump` | `"0 root\n2 child\n"` → `DepthJump` |
| `parse_error_invalid_value` | `"0 n key=baz\n"` → `InvalidValue` |
| `parse_error_unexpected_token` | `"0 =bad\n"` → `UnexpectedToken` |
