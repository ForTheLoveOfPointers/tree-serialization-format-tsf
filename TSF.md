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
