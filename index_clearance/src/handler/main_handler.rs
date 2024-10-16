use crate::common::*;

use crate::service::index_clear_service::*;

pub struct MainHandler<I: IndexClearService> {
    index_clear_service: I
}

impl<I: IndexClearService> MainHandler<I> {
    
    pub fn new(index_clear_service: I) -> Self {
        Self { index_clear_service }
    }
    
    
    /*

    */
    pub async fn task_set(&self) -> Result<(), anyhow::Error> {

        

        Ok(())
    }
}