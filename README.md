# Clinic Management Vault (Solana Program)

Sistema descentralizado de gestión de consultorios médicos y veterinarios desarrollado con **Anchor Framework**. Permite a los profesionales de la salud gestionar expedientes de pacientes, tratamientos y deudas de forma segura en la blockchain de Solana.

## 🚀 Características
- **Jerarquía de Cuentas**: Administración centralizada de consultorios con múltiples pacientes vinculados.
- **Seguridad PDA**: Direcciones derivadas del programa para garantizar que cada expediente sea único y solo accesible por su dueño.
- **Control Financiero**: Seguimiento de costos de tratamiento y deudas pendientes por paciente.
- **Optimización de Espacio**: Gestión eficiente del almacenamiento para minimizar los costos de *Rent*.

## 🏗️ Estructura del Proyecto

### Cuentas Principales
1. **Consultorio**: Almacena la información del administrador y el contador global de pacientes.
2. **Paciente**: Contiene el historial clínico, especie, última visita y saldo pendiente.

### PDAs (Program Derived Addresses)
| Tipo | Semillas (Seeds) |
| :--- | :--- |
| **Consultorio** | `["consultorio", admin_pubkey]` |
| **Paciente** | `["paciente", consultorio_pubkey, nombre_paciente]` |

## 🛠️ Instrucciones Disponibles

- `inicializar_consultorio`: Crea la entidad del consultorio vinculada al médico.
- `registrar_paciente`: Registra un nuevo paciente bajo el control del consultorio.
- `añadir_tratamiento`: Actualiza el historial médico, la fecha de visita y el saldo deudor.

## 💻 Requisitos
- [Rust](https://www.rust-lang.org/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://www.anchor-lang.com/)

## 🔧 Configuración
1. Clonar el repositorio.
2. Cambiar el `program_id` en `declare_id!()` dentro de `lib.rs`.
3. Ejecutar `anchor build` para compilar.
4. Ejecutar `anchor test` para verificar el funcionamiento.

---
**Desarrollado con fines educativos en el ecosistema de Solana.**
