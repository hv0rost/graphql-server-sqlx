use async_graphql::*;
use sqlx::PgPool;
use crate::data_base::models::*;
use crate::db_requests::*;
use crate::db_requests::store::Store;

#[derive(Default)]
pub struct Query;
#[Object]
impl Query {
    //Store field
    async fn get_all_stores(&self, ctx: &Context<'_>) -> Result<Vec<Store>, Error> {
        let stores = Store::get_all_stores(&ctx.data::<PgPool>().unwrap()).await?;
        Ok(stores)
    }

   /* async fn get_stores_from_customer_name(&self, customers_name: String) -> Vec<Store> {
        store::get_stores_from_customer_name(&establish_connection(), customers_name)
            .expect("Can't get customers")
            .iter()
            .map(Store::from)
            .collect()
    }

    //Customers field
    async fn get_all_customers(&self) -> Vec<Customer> {
        customers::get_all_customers(&establish_connection())
        .expect("Can't get customers")
        .iter()
        .map(Customer::from)
        .collect()
    }*/
}
#[derive(Default)]
pub struct Mutation;
#[Object]
impl Mutation {
    async fn create_store(&self, ctx: &Context<'_>, name : String, clients: Option<i32>) -> Result<Store, Error> {
        let new_store = Store {
            id: 10,
            name,
            clients,
        };

        let created_store_entity =
            Store::create_store(&ctx.data::<PgPool>().unwrap(), new_store).await?;

        Ok(created_store_entity)
    }

   /* async fn create_customer(&self, name : String, data: Option<serde_json::Value>, date : Option<chrono::NaiveDateTime>) -> Result<Customer> {
        let new_customer = NewCustomerEntity {
            name,
            data,
            date,
        };

        let created_customer_entity =
            customers::create_customer(new_customer, &establish_connection())?;
        Ok(Customer::from(&created_customer_entity))
    }

    async fn delete_store(&self, id : i32) -> Result<Store>{
            let deleted_store_entity =
                store::delete_store(id, &establish_connection())?;

        Ok(Store::from(&deleted_store_entity))
    }

    async fn update_store(&self, id : i32, new_name: String) -> Result<Store>{
        let updated_store_entity =
            store::update_store(&establish_connection(), id, new_name)?;

        Ok(Store::from(&updated_store_entity))
    }*/
}

