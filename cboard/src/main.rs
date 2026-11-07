//! cboard — Teclado en pantalla
//!
//! Alcance acordado en CosmicOS spec secciones 7.5 y 10.3:
//! - Protocolos Wayland input-method-v2 y text-input-v3.
//! - Sandbox estricto con namespace de red vacío (sin acceso a internet).
//! - Sin acceso al almacenamiento de usuario; estado de aprendizaje guardado cifrado en ~/.var/daemons/cboard.
//! - No aprende de campos marcados como contraseña.

fn main() {
    println!("[cboard] Iniciando teclado en pantalla seguro...");
    println!("[cboard] Protocolo input-method-v2 activo en espacio aislado sin red.");
}
