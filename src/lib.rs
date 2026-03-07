use anchor_lang::prelude::*;
declare_id!("3ZGA2wmVSnnYWdt4sdCWRZ9W4pDffuWyNtz5NgJzZX3w");

#[program]
pub mod biblioteca {
    use super::*;

    pub fn crear_biblioteca(ctx: Context<NuevaBiblioteca>, nombre: String) -> Result<()> {
        let owner: Pubkey = ctx.accounts.owner.key();
        let libros: Vec<Libro> = Vec::new();

        ctx.accounts.biblioteca.set_inner(Biblioteca {
            owner,
            nombre,
            libros,
        });

        Ok(())
    }

    pub fn agregar_libro(ctx: Context<NuevoLibro>, nombre: String, paginas: u16) -> Result<()> {
        let libro = Libro {
            nombre,
            paginas,
            disponible: true,
        };

        let biblioteca = &mut ctx.accounts.biblioteca;

        // evitar overflow del Vec (max_len = 10)
        require!(biblioteca.libros.len() < 10, BibliotecaError::ListaLlena);

        biblioteca.libros.push(libro);

        Ok(())
    }

    pub fn ver_libros(ctx: Context<VerLibros>) -> Result<()> {
        msg!("Libros: {:#?}", ctx.accounts.biblioteca.libros);
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Biblioteca {
    pub owner: Pubkey,

    #[max_len(60)]
    pub nombre: String,

    #[max_len(10)]
    pub libros: Vec<Libro>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Libro {
    #[max_len(60)]
    pub nombre: String,
    pub paginas: u16,
    pub disponible: bool,
}

// -------- CONTEXTOS ------------------

#[derive(Accounts)]
pub struct NuevaBiblioteca<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Biblioteca::INIT_SPACE,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump
    )]
    pub biblioteca: Account<'info, Biblioteca>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoLibro<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub biblioteca: Account<'info, Biblioteca>,
}

#[derive(Accounts)]
pub struct VerLibros<'info> {
    #[account(
        seeds = [b"biblioteca", biblioteca.owner.as_ref()],
        bump
    )]
    pub biblioteca: Account<'info, Biblioteca>,
}

// -------- ERRORES ---------------------

#[error_code]
pub enum BibliotecaError {
    #[msg("La lista de libros ya está llena.")]
    ListaLlena,
}
