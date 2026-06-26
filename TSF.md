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

Cada línea debe terminar con un salto de línea (`\n`). No se permite un EOF sin `\n` al final.

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

| Componente   | Estado     |
|--------------|------------|
| Lexer        | ✓ Completo |
| Parser       | ✓ Completo |
| AST          | ✓ Completo |
| Serializer   | ✓ Completo |
| Validator    | Pendiente  |

No se requiere motor de renderizado.

TSF define únicamente la representación de árboles.

---

## 16. Parser implementado

El parser se implementó como parte de la referencia Rust. A continuación se documenta lo construido.

### 16.1 Archivos

| Archivo | Rol |
|---------|-----|
| `src/parser.rs` | Declaración del módulo: `pub mod parser_types;` |
| `src/parser/parser_types.rs` | `Parser<R: BufRead>` y lógica completa |
| `src/errors.rs` | Declaración del módulo: `pub mod error_types;` |
| `src/errors/error_types.rs` | `ParseError` con `Display` + `Error` |
| `src/lexer/lexer_types.rs` | Se añadió `Clone` a `Token` (necesario para errores) |
| `src/main.rs` | Actualizado para usar `Parser::parse()` |

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

Implementa `fmt::Display` y `std::error::Error`. Incluye `From<std::io::Error>`.

### 16.3 `Parser<R: BufRead>` (src/parser/parser_types.rs)

```rust
pub struct Parser<R: BufRead> {
    lex: Lexer<R>,
    lookahead: Option<Token>,
}
```

**Métodos públicos:**
- `Parser::new(reader: R) -> Self` — construye el parser y precarga el primer token.
- `Parser::parse(&mut self) -> Result<Document, ParseError>` — consume toda la entrada y construye el árbol.

**Métodos privados:**
- `peek(&self) -> Option<&Token>` — lookahead sin consumir.
- `advance(&mut self) -> Token` — consume el lookahead y carga el siguiente token.
- `expect_depth(&mut self) -> Result<String, ParseError>` — consume un `Depth` o error.
- `expect_identifier(&mut self) -> Result<String, ParseError>` — consume un `Identifier` o error.
- `expect_assign(&mut self) -> Result<(), ParseError>` — consume un `Assign` o error.
- `parse_value(&mut self) -> Result<Value, ParseError>` — `Text` → `Value::String`; `Identifier` se coeerce a Integer, Float, Boolean, Null, o error.
- `parse_line(&mut self) -> Result<(u32, Node), ParseError>` — una línea completa: `Depth name [attrs] [content] NewLine`.

### 16.4 Algoritmo de reconstrucción del árbol

El parser construye el árbol usando una pila de nodos. A diferencia del pseudocódigo de la sección 12, la implementación en Rust no puede clonar el nodo sin romper el enlazado padre-hijo (el clon no vería los hijos añadidos posteriormente). En su lugar, los hijos se vinculan al padre **cuando se desapilan**, no al apilarse.

```
stack = []
loop:
    if peek es Eof → break
    (depth, node) = parse_line()
    if depth > stack.len() → error DepthJump
    while stack.len() > depth:
        child = stack.pop()
        parent = stack.last_mut()
        parent.children.push(child)
    stack.push(node)

// Vaciar pila: vincular nodos restantes hacia arriba
while stack.len() > 1:
    child = stack.pop()
    parent = stack.last_mut()
    parent.children.push(child)

return Document { root: stack[0] }
```

**Validaciones activas:**
- Primera línea debe tener depth 0 → si no, `DepthJump { 0, depth }`.
- Depth sólo puede aumentar en 1 → `depth > stack.len()` es salto inválido.
- Múltiples líneas en depth 0 → `UnexpectedToken` (un solo árbol por documento).
- Después de `=`, `Text` → `Value::String`; `Identifier` se parsea como entero → flotante → booleano → null, o `InvalidValue`.
- Contenido textual después de atributos ya consumidos → `UnexpectedToken`.

### 16.5 Entry point (src/main.rs)

```rust
fn main() {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let mut parser = parser::parser_types::Parser::new(reader);
    match parser.parse() {
        Ok(doc) => println!("{}", doc),
        Err(e) => eprintln!("{}", e),
    }
}
```

### 16.6 Tests

17 tests inline en `src/parser/parser_types.rs` (`#[cfg(test)] mod tests`):

| Test | Descripción |
|------|-------------|
| `parse_root_only` | `"0 html\n"` → Document con un solo nodo |
| `parse_nested_tree` | Árbol completo → coincide con AST esperado |
| `parse_attributes` | Línea con `key="string"` |
| `parse_attributes_unquoted` | `true`, `42`, `3.14` como valores |
| `parse_attrs_and_content` | Atributos + contenido textual |
| `parse_content` | Línea solo con contenido |
| `parse_negative_values` | `-42` y `-3.14` en atributos |
| `parse_null_value` | `key=null` |
| `roundtrip_forward_parse_serialize` | Parse → Serialize, output == input |
| `roundtrip_reverse_serialize_parse` | Serialize → Parse, AST == AST original |
| `parse_error_empty` | `""` → `EmptyInput` |
| `parse_error_depth_nonzero_root` | `"2 root\n"` → `DepthJump` |
| `parse_error_depth_jump` | `"0 root\n2 child\n"` → `DepthJump` |
| `parse_error_invalid_value` | `"0 n key=baz\n"` → `InvalidValue` |
| `parse_error_unexpected_token` | `"0 =bad\n"` → `UnexpectedToken` |
| `parse_error_multiple_roots` | `"0 a\n0 b\n"` → `UnexpectedToken` |
| `parse_error_content_then_attr` | `"0 d \"x\" c=y\n"` → `UnexpectedToken` |

---

## 17. Plan del validador (próximo)

El validador será el último componente de la implementación de referencia. Su función es verificar reglas semánticas que el parser no cubre (el parser se enfoca en estructura sintáctica y reglas de profundidad).

### 17.1 Archivos a crear

| Archivo | Acción |
|---------|--------|
| `src/validator.rs` | Crear — declaración del módulo |
| `src/validator/validator_types.rs` | Crear — `Validator`, `ValidationError` |

### 17.2 `ValidationError`

```rust
pub enum ValidationError {
    InvalidIdentifier { raw: String, kind: IdentifierKind },
    DepthLeadingZeros { raw: String },
    DuplicateAttributeKey { node_index: usize, key: String },
}
```

Donde `IdentifierKind` diferencia entre nombre de nodo, clave de atributo y valor de atributo.

### 17.3 `Validator`

```rust
pub struct Validator;

impl Validator {
    pub fn validate(&self, doc: &Document) -> Vec<ValidationError> {
        // Recorrer el árbol y recolectar errores
    }
}
```

### 17.4 Validaciones

| Validación | Regla |
|------------|-------|
| **Identificador de nodo** | `[a-zA-Z_][a-zA-Z0-9_-]*` |
| **Clave de atributo** | `[a-zA-Z_][a-zA-Z0-9_-]*` |
| **Depth sin ceros a la izquierda** | `"0"` o `[1-9][0-9]*` |
| **Claves de atributo duplicadas** | Misma clave dos veces en un nodo → error |
| **Integración** | Llamar `Validator::validate()` después de `Parser::parse()` en `main.rs` |

### 17.5 Tests

| Test | Descripción |
|------|-------------|
| `invalid_node_name` | `"0 123\n"` → error de identificador |
| `invalid_attr_key` | `"0 n 1key=v\n"` → error de identificador |
| `depth_leading_zeros` | `"00 root\n"` → error |
| `duplicate_attr_key` | `"0 n k=v k=w\n"` → error |
| `valid_full_tree` | Árbol canónico → sin errores |
