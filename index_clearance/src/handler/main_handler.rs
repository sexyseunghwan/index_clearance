use elasticsearch::cluster;

use crate::common::*;

use crate::service::index_clear_service::*;

use crate::repository::smtp_repository::*;

use crate::models::email_struct::*;

pub struct MainHandler<I: IndexClearService> {
    index_clear_service: I,
}

impl<I: IndexClearService> MainHandler<I> {
    pub fn new(index_clear_service: I) -> Self {
        Self {
            index_clear_service,
        }
    }

    #[doc = "인덱스 삭제 메인 작업 함수."]
    pub async fn task_set(&self) -> Result<(), anyhow::Error> {
        /* Elasticsearch cluster 이름 */
        let cluster_name = self.index_clear_service.get_cluster_name();
        
        /* 인덱스 삭제 함수. */
        let send_email_form: Vec<EmailStruct> =
            self.index_clear_service.delete_index_from_rule().await?;

        /* 인덱스 삭제 내역이 있다면, 메일로 post */
        if send_email_form.len() != 0 {
            /* smtp client 객체 생성 */
            let smtp_repo = get_smtp_repo();
            let cluster_name = self.index_clear_service.get_cluster_name();

            /* smtp client 를 통해서 메일 전송 */
            smtp_repo
                .send_message_to_receivers(&send_email_form, &cluster_name)
                .await?;
        } else {
            info!("[{}] Deleted index does not exist.", cluster_name);
        }
        
        info!("[{}] Index Schedule Program processed successfully", cluster_name);
        Ok(())
    }
}
