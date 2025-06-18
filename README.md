# Intérprete de Lenguaje Simple en Rust

![image](https://github.com/user-attachments/assets/d2801837-0ef8-4b0f-bcc2-b03c5e38e2e0)

Este proyecto es un **intérprete educativo** de un lenguaje de programación simple, desarrollado completamente en **Rust**. Cuenta con una **interfaz gráfica moderna** (basada en [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe)) y un modo por **línea de comandos**, ambos con soporte para análisis léxico, sintáctico y ejecución de estructuras básicas como asignaciones, comparaciones, bucles y condicionales.

---

## Estructura del Proyecto

```

ANALIZADOR/
├── src/
│   ├── lexer.rs         # Analizador léxico: convierte texto en tokens
│   ├── parser.rs        # Analizador sintáctico + ejecución con detección de errores
│   ├── main.rs          # Interfaz gráfica usando egui/eframe
│   └── main_cli.rs      # Ejecución desde archivo por consola
├── programa.txt         # Programa de ejemplo a interpretar
├── Cargo.toml           # Configuración del proyecto Rust
├── Cargo.lock
└── README.md            # Este documento

````

---

## Características

Lenguaje simple con:

- Variables enteras
- Expresiones matemáticas y lógicas
- Estructuras `if`, `while`, `print`
- Comentarios con `#`

Interfaz gráfica moderna con:

- Campo de entrada de código
- Botones `Ejecutar` y `Limpiar`
- **Historial de entradas reutilizable**
- Scroll en área de resultados
- Mensajes de error resaltados en rojo con número de línea

Modo consola:

- Lee desde `programa.txt`
- Útil para pruebas automáticas o sin GUI

Errores sintácticos y de ejecución manejados limpiamente

Compatible con compilación a `.exe` para distribución

---

## Ejemplo de programa soportado

```text
# Contador del 0 al 2
x = 0

while x < 3
  print x
  x = x + 1
end

if x == 3
  print x
end
````

---

## Cómo ejecutar

### Requisitos

* [Rust](https://rustup.rs)
* Visual Studio Build Tools (con C++ y Windows SDK)

---

### GUI (Interfaz gráfica)

```bash
cargo run --release --bin analizador
```

> Esto abrirá la aplicación visual con entrada y salida de código en vivo.

---

### CLI (modo consola)

Coloca el código en `programa.txt` y ejecuta:

```bash
cargo run --release --bin main_cli
```

---

## Compilación del ejecutable

```bash
cargo build --release
```

* GUI: `target/release/analizador.exe`
* CLI: `target/release/main_cli.exe`

---

## Distribución

Puedes empaquetar el `.exe` en un `.zip` o crear un instalador con:

* [Inno Setup](https://jrsoftware.org/isinfo.php)
* [NSIS](https://nsis.sourceforge.io/)

---

## Autoría

Desarrollado por **Nathaly Michel Berroa Fermín**
