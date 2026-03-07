use anchor_lang::prelude::*;
declare_id!("");

#[programa]
pub mod biblioteca{
    use super::*;
    
    pub fn crear_biblioteca()-> Result<()>{

      }  
}
#[programa]
#[derive(ImitSpace)]
pub struct Biblioteca{ 
    owner: Pubkey,
    #[max_len(60)]
    nombre: string,
    #[max_len(10)]
    libros: Vec<Libro>,
}  
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub struct Libro{ 
    nombre: string,
    paginas:u16,
    disponible:bool,
}  
