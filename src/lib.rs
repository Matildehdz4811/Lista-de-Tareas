use anchor_lang::prelude::*;

declare_id!("AqC74PpqtSo9qpffPSd4npC6wchULpiDBSe7SwxFVnfK"); 

// --- Constantes de Tamaño ---
const MAX_CLINIC_NAME: usize = 50 * 4;
const MAX_PATIENT_NAME: usize = 40 * 4;
const MAX_TREATMENT: usize = 100 * 4;

#[program]
pub mod clinic_management {
    use super::*;

    // nicializa el consultorio (Solo el médico/admin)
    pub fn inicializar_consultorio(ctx: Context<InicializarConsultorio>, nombre: String) -> Result<()> {
        let consultorio = &mut ctx.accounts.consultorio;
        require!(nombre.chars().count() <= 50, ClinicError::TextoLargo);
        
        consultorio.admin = ctx.accounts.admin.key();
        consultorio.nombre = nombre;
        consultorio.total_pacientes = 0;
        
        msg!("Consultorio '{}' creado exitosamente", consultorio.nombre);
        Ok(())
    }

    // Registrar un nuevo paciente en el consultorio
    pub fn registrar_paciente(
        ctx: Context<RegistrarPaciente>, 
        nombre: String, 
        especie: String
    ) -> Result<()> {
        let paciente = &mut ctx.accounts.paciente;
        let consultorio = &mut ctx.accounts.consultorio;

        paciente.consultorio = consultorio.key();
        paciente.nombre = nombre;
        paciente.especie = especie;
        paciente.ultima_visita = Clock::get()?.unix_timestamp;
        
        //Incrementamos el contador del consultorio
        consultorio.total_pacientes += 1;

        msg!("Paciente {} registrado en el consultorio", paciente.nombre);
        Ok(())
    }

    //Añadir una nota médica o tratamiento
    pub fn añadir_tratamiento(ctx: Context<GestionarPaciente>, nota: String, costo: u64) -> Result<()> {
        let paciente = &mut ctx.accounts.paciente;
        
        paciente.ultimo_tratamiento = nota;
        paciente.ultima_visita = Clock::get()?.unix_timestamp;
        paciente.deuda_pendiente += costo;

        msg!("Tratamiento actualizado para {}", paciente.nombre);
        Ok(())
    }
}

// --- Estructuras de Datos ---



#[account]
pub struct Consultorio {
    pub admin: Pubkey,       // 32
    pub nombre: String,      // 4 + MAX_CLINIC_NAME
    pub total_pacientes: u64, // 8
}

#[account]
pub struct Paciente {
    pub consultorio: Pubkey,     // 32 (A qué clínica pertenece)
    pub nombre: String,          // 4 + MAX_PATIENT_NAME
    pub especie: String,         // 4 + 40 (Perro, Gato, etc)
    pub ultima_visita: i64,      // 8 (Timestamp)
    pub ultimo_tratamiento: String, // 4 + MAX_TREATMENT
    pub deuda_pendiente: u64,    // 8
}

// --- Contextos de Instrucciones ---

#[derive(Accounts)]
pub struct InicializarConsultorio<'info> {
    #[account(
        init, 
        payer = admin, 
        space = 8 + 32 + (4 + MAX_CLINIC_NAME) + 8,
        seeds = [b"consultorio", admin.key().as_ref()],
        bump
    )]
    pub consultorio: Account<'info, Consultorio>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nombre: String)] //Pasamos el nombre para usarlo en el seed si queremos
pub struct RegistrarPaciente<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + (4 + MAX_PATIENT_NAME) + 60 + 8 + (4 + MAX_TREATMENT) + 8,
        // Seed única por paciente dentro de ese consultorio
        seeds = [b"paciente", consultorio.key().as_ref(), nombre.as_bytes()],
        bump
    )]
    pub paciente: Account<'info, Paciente>,
    #[account(mut)]
    pub consultorio: Account<'info, Consultorio>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarPaciente<'info> {
    #[account(mut, has_one = consultorio)]
    pub paciente: Account<'info, Paciente>,
    #[account(constraint = consultorio.admin == admin.key())]
    pub consultorio: Account<'info, Consultorio>,
    pub admin: Signer<'info>,
}

#[error_code]
pub enum ClinicError {
    #[msg("El texto proporcionado es demasiado largo.")]
    TextoLargo,
    #[msg("No tienes permisos para modificar este paciente.")]
    NoAutorizado,
}
