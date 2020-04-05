use crate::service::structure;

pub fn init_nota_folder(folder_name: &str) {
    info!("application/plumbing/init_nota_folder");
    structure::init_structure(folder_name);
}