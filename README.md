# UI

Interfaz gráfica de CosmicOS (móvil primero).

| Componente | Qué es | Lenguaje |
|---|---|---|
| `phanteonwm` | Gestor de ventanas / compositor (Smithay, Wayland) | Rust |
| `creationde` | Entorno de escritorio | C++ / CMake |
| `cboard` | Teclado en pantalla, proceso aislado | Rust |
| `cosmic-splash` | Pantalla de arranque | Rust |

## Compilar

Los crates de Rust entran en el workspace de Cargo. `creationde` queda fuera (se construye con CMake):

```sh
cargo check -p phanteonwm
cmake -S creationde -B build/creationde
```

## Licencia

GPL-3.0-or-later.
