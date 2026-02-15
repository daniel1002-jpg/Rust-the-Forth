# Rust-the-Forth

Ejercicio individual (1C 2025) para Taller de Programación I, FIUBA, cátedra Deymonnaz.

## Descripción
Este proyecto implementa un intérprete del lenguaje Forth en Rust, abordando parsing, ejecución de instrucciones, manipulación de pila, definición de palabras, operaciones aritméticas y lógicas, y manejo de memoria. El diseño modular permite definir y ejecutar palabras complejas, y la integración de pruebas unitarias asegura la robustez del sistema.

## Características principales
- Parsing de instrucciones Forth
- Manipulación de pila (push, drop, swap, over, rot)
- Definición y ejecución de palabras (user-defined words)
- Operaciones aritméticas y lógicas
- Soporte para definiciones multilinea
- Pruebas unitarias integradas
- Diseño modular orientado a extensibilidad

## Estructura del proyecto
- `src/forth/`: Lógica del intérprete, parser, definición de palabras
- `src/handler/`: Manejo de instrucciones y ejecución
- `src/stack/`: Implementación de la pila
- `tests/`: Pruebas unitarias e integración

## Ejemplo de uso
```bash
cargo run input.fth stack-size=1024
```
Donde `input.fth` contiene instrucciones Forth.

## Temas FIUBA
- fiuba
- TA045

## Autor
Daniel Mamani

## Enlaces
- [Portfolio](https://github.com/daniel1002-jpg/portfolio)
- [Página FIUBA Repos](https://fede.dm/FIUBA-Repos/?c=TA045)

---
> Proyecto académico orientado a arquitectura de lenguajes y sistemas, alineado con la formación en programación y estructuras de datos.