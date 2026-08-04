# 2048 en Rust 🦀🎮

Un mini-proyecto para recrear el clásico juego matemático 2048, desarrollado 100% en Rust. 

Este proyecto fue creado con el objetivo de repasar y poner en práctica los conceptos fundamentales de Rust, tales como *borrowing*, *ownership*, y el manejo de estructuras de datos (matrices), implementando además una interfaz gráfica nativa.

## 🚀 Características

- **Lógica desde cero:** Movimientos, fusiones de fichas, sistema de puntuación y detección de Game Over / Victoria programados íntegramente en Rust.
- **Interfaz Gráfica:** Construida utilizando [Dioxus](https://dioxuslabs.com/), un framework declarativo inspirado en React que permite renderizar vistas nativas de escritorio.
- **Diseño fiel:** Paleta de colores, tipografías y espaciados basados en el juego original.

## 🛠️ Tecnologías utilizadas

- **Lenguaje:** Rust
- **Frontend / UI:** Dioxus (con el feature `desktop`)
- **Dependencias extra:** `rand` (para la generación aleatoria de fichas)
