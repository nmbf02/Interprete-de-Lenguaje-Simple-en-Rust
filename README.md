# Intérprete de Lenguaje Simple en Rust

Este proyecto implementa un **intérprete completo de un lenguaje de programación básico**, escrito 100% en Rust. Incluye:

- Analizador léxico (lexer)
- Analizador sintáctico (parser)
- Análisis semántico (contexto de variables)
- Evaluación y ejecución de instrucciones
- Soporte para:
  - Asignaciones (`x = 5 + 2`)
  - Impresiones (`print x`)
  - Bloques condicionales (`if ... end`)
  - Bucles (`while ... end`)
  - Comentarios con `#`
  - Lectura de archivos `.txt` como entrada

## Estructura del proyecto

```

analizador/
├── Cargo.toml
├── programa.txt         # Código fuente de entrada
├── src/
│   ├── main.rs
│   ├── lexer.rs
│   └── parser.rs
└── .cargo/
└── config.toml      # Forzar uso de toolchain GNU (opcional)

````

## Ejemplo de entrada (`programa.txt`)

```txt
# Programa de prueba
x = 0

while x < 3
  print x
  x = x + 1
end

if x == 3
  print x
end
````

## Cómo ejecutar

1. Asegúrate de tener Rust con la toolchain `gnu`:

```bash
rustup install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

2. Ejecuta el proyecto:

```bash
cd C:\analizador
cargo run
```

3. Verás el resultado:

```
0
1
2
3
```

## Requisitos

* Rust (toolchain GNU)
* PowerShell o terminal CMD
* Editor como Visual Studio Code (opcional)

## Autora

**Nathaly Michel Berroa Fermín**
