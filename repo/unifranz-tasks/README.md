# unifranz-tasks (TUI)

TUI para recordar tareas del calendario iCal de Unifranz. Sin widget flotante, solo terminal con layout por columnas y notificaciones/gestión de tareas.

## Características
- Layout: izquierda (3 columnas: TIEMPO VERDE >=7 días, TIEMPO AMARILLO 4-6 días, TIEMPO ROJO <=2 días) + derecha (descripción o COMMAND KEYBOARD)
- Caja con: Título, Fecha dd/mm/yy, Hora hh:mm AM/PM, Materia, URL clickeable (Ctrl+Enter/o para abrir)
- Navegación con teclado (flechas/hjkl)
- Eliminar tarea: seleccionar + Enter -> escribir `delete` + Enter
- Panel derecho muestra descripción limpia en markdown o ayuda con atajos
- Usa `~/.config/unifranz-tasks/config.toml` y `~/.local/state/unifranz-tasks/seen.json`

## Ejecutar
```bash
cp config/config.toml.example ~/.config/unifranz-tasks/config.toml
cargo run --release
```
