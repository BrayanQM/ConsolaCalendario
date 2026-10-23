# 1. Uso En El Flujo

**Proyecto:** unifranz-tasks
**Sistema:** TUI
**Fecha de documentación:** 2026-10-23
**Autor:** eynor

---

## Técnica / Método
**Arquitectura por Bloques + MVVM (adaptado a TUI)** con render en `ratatui`, eventos con `crossterm`, navegación por teclado.

## ¿Para qué sirve?
Recordar tareas del calendario `.ics` de Unifranz sin widget flotante. Muestra urgencia por días restantes (Verde/Amarillo/Rojo), permite abrir URL, ver descripción limpia y eliminar tarea con confirmación.

## Dónde lo ve el usuario
En terminal: panel izquierdo dividido en 3 columnas con cajas por tarea, panel derecho con descripción o atajos.
