// use crate::{
//     adapters::inbounds::rest::lottery_dto,
//     shared::errors::Rfc7807Problem,
// };
//
// /// Purchases a new set of lottery tickets, each with length 5. Max 5 tickets, min 1
// pub struct PurchaseLotteryUseCase<R: ClientLotteryRepo> {
//     repo: R,
// }
//
// impl<R: ClientLotteryRepo> PurchaseLotteryUseCase<R> {
//     pub fn new(repo: R) -> Self {
//         Self { repo }
//     }
//
//     pub async fn execute(
//         &self,
//         client_lottery: lottery_dto::LotteriesPurchaseDto,
//     ) -> Result<client_lotteries::Model, Rfc7807Problem> {
//         let length = client_lottery.tickets.len();
//
//         // the number of tickets must be between 1 and 5
//         if !(1..=5).contains(&length) {
//             return Err(Rfc7807Problem::simple_bad_request(
//                 Some("Error al comprar sus tickets"),
//                 Some("Debe comprar entre 1 y 5 tockets"),
//                 None,
//             ));
//         }
//
//         // all the tickets must have 5 numbers
//         for ticket in &client_lottery.tickets {
//             if ticket.numbers.len() != 5 {
//                 return Err(Rfc7807Problem::simple_bad_request(
//                     Some("Error al comprar sus tickets"),
//                     Some("Todos los tickets deben tener 5 números"),
//                     None,
//                 ));
//             }
//         }
//
//         todo!()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         domain::{entities::client_lotteries, repositories::client_lottery::ClientLotteryRepo},
//         shared::errors::Rfc7807Problem,
//     };
//     use async_trait::async_trait;
//     use serde_valid::Validate;
//
//     struct MockClientLotteryRepo {
//         should_succeed: bool,
//     }
//
//     impl MockClientLotteryRepo {
//         fn new_success() -> Self {
//             Self {
//                 should_succeed: true,
//             }
//         }
//
//         fn new_failure() -> Self {
//             Self {
//                 should_succeed: false,
//             }
//         }
//     }
//
//     #[async_trait]
//     impl ClientLotteryRepo for MockClientLotteryRepo {
//         async fn create_client_lottery(
//             &self,
//             _client_lottery: client_lotteries::ActiveModel,
//         ) -> Result<String, Rfc7807Problem> {
//             if self.should_succeed {
//                 Ok(String::from("Registro guardado exitosamente"))
//             } else {
//                 Err(Rfc7807Problem {
//                     r#type: "/v1/database-error".to_string(),
//                     title: "Database Error".to_string(),
//                     status: 500,
//                     detail: Some("Failed to save lottery purchase".to_string()),
//                     error_code: Some("DB_ERROR".to_string()),
//                     errors: None,
//                 })
//             }
//         }
//     }
//
//     #[tokio::test]
//     async fn lottery_tickets_between_1_and_5() {
//         let mock_repo = MockClientLotteryRepo::new_success();
//         let use_case = PurchaseLotteryUseCase::new(mock_repo);
//
//         // checking what if it's 0
//         let purchase_dto = lottery_dto::LotteriesPurchaseDto::default();
//
//         let result = use_case.execute(purchase_dto).await;
//         assert!(result.is_err());
//
//         // checking if it's 6 (>5)
//         let purchase_dto = lottery_dto::LotteriesPurchaseDto {
//             tickets: (1..=6)
//                 .map(|_| lottery_dto::CreateLotteryDto::default())
//                 .collect(),
//             ..Default::default()
//         };
//
//         let result = use_case.execute(purchase_dto).await;
//         assert!(result.is_err());
//     }
//
//     #[tokio::test]
//     async fn all_tickets_numbers_length_5() {
//         let mock_repo = MockClientLotteryRepo::new_failure();
//         let use_case = PurchaseLotteryUseCase::new(mock_repo);
//
//         let purchase_dto = lottery_dto::LotteriesPurchaseDto {
//             tickets: vec![lottery_dto::CreateLotteryDto {
//                 numbers: vec![10, 20],
//             }],
//             ..Default::default()
//         };
//
//         let result = use_case.execute(purchase_dto).await;
//         assert!(result.is_err());
//     }
//
//     #[tokio::test]
//     async fn validate_lottery_purchase_dto() {
//         // failing on purpose
//         let mut purchase_dto = lottery_dto::LotteriesPurchaseDto {
//             client_name: "devall".to_string(),
//             client_last_name: Some("sas".to_string()),
//             client_email: "devall@devalsas.com".to_string(),
//             client_cellphone: "+593987654321".to_string(),
//             location: Some("Ecuador".to_string()),
//             ..Default::default()
//         };
//
//         let result = purchase_dto.validate();
//         assert!(result.is_ok());
//
//         // failing due to wrong client name 
//         purchase_dto.client_name = "".to_string();
//         let result = purchase_dto.validate();
//         assert!(result.is_err());
//
//         // failing due to wrong client last name
//         purchase_dto.client_last_name = Some("".to_string());
//         let result = purchase_dto.validate();
//         assert!(result.is_err());
//
//         // failing due to wrong client email
//         purchase_dto.client_email = "".to_string();
//         let result = purchase_dto.validate();
//         assert!(result.is_err());
//
//         // failing due to wrong client cellphone
//         purchase_dto.client_cellphone = "".to_string();
//         let result = purchase_dto.validate();
//         assert!(result.is_err());
//
//         // failing due to wrong location
//         purchase_dto.location = Some("".to_string());
//         let result = purchase_dto.validate();
//         assert!(result.is_err());
//     }
// }
