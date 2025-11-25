<h1 align="center">
  <img src="static/img/molecule.png" alt="Glyph Icon" width="128" height="128">
  <div align="center">Glyph</div>
</h1>

<div align="center">

[![GitHub: Repo](https://img.shields.io/badge/glyph-58A6FF?&logo=github)](https://github.com/Yrrrrrf/glyph)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow)](./LICENSE)
<!-- [![JSR](https://jsr.io/badges/@yrrrrrf/glyph)](https://jsr.io/@yrrrrrf/glyph) -->
<!-- [![Crates.io](https://img.shields.io/crates/v/glyph.svg?logo=rust)](https://crates.io/crates/glyph) -->

</div>

> Un framework basado en SvelteKit para construir aplicaciones web con un enfoque en la visualización científica y análisis de datos.

Glyph es un framework web que aprovecha el poder de SvelteKit en el frontend y WebAssembly compilado con Rust en el backend. Está diseñado para proporcionar un entorno rico e interactivo para visualizar y analizar conjuntos de datos complejos directamente en el navegador.

## Demo

![Showcase](static/img/showcase.png)

## Instalación

1.  **Clona el repositorio:**
    ```sh
    git clone https://github.com/Yrrrrrf/glyph.git
    cd glyph
    ```

2.  **Instala las dependencias del frontend:**
    ```sh
    deno task install
    ```

3.  **Compile el módulo WebAssembly de Rust:**
    ```sh
    ./build-wasm.sh
    ```

4. **Ejecuta la aplicación:**
    ```sh
    deno task dev
    ```

> Esto lanzará la aplicación SvelteKit, con el motor de análisis impulsado por Rust ejecutándose como un módulo de WebAssembly.

## Licencia

Este proyecto está licenciado bajo la [**Licencia MIT**](LICENSE).