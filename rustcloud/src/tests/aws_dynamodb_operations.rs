#[allow(unused_imports)]
use crate::aws::aws_apis::database::aws_dynamodb::*;
#[allow(unused_imports)]
use aws_sdk_dynamodb::config::Region;
#[allow(unused_imports)]
use aws_sdk_dynamodb::types::{
    AttributeDefinition, AttributeValue, BillingMode, ComparisonOperator, Condition,
    GlobalSecondaryIndex, KeySchemaElement, KeyType, LocalSecondaryIndex, ProvisionedThroughput,
    ReturnConsumedCapacity, ReturnValue, ScalarAttributeType, Select, SseSpecification,
    StreamSpecification, TableClass, WriteRequest,
};
#[allow(unused_imports)]
use aws_sdk_dynamodb::{Client, Config};
#[allow(unused_imports)]
use std::collections::HashMap;

async fn create_client() -> Client {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    return client;
}

#[tokio::test]
async fn test_create_table() {
    let client = create_client().await;

    let table_name = "test-table".to_string();
    let attribute_definitions = AttributeDefinition::builder()
        .attribute_name("id")
        .attribute_type(ScalarAttributeType::S)
        .build()
        .unwrap();
    let key_type = KeyType::Hash;
    let key_schema = KeySchemaElement::builder()
        .attribute_name("id")
        .key_type(key_type)
        .build()
        .unwrap();
    let provisioned_throughput = ProvisionedThroughput::builder()
        .read_capacity_units(5)
        .write_capacity_units(5)
        .build()
        .unwrap();

    let result = create_table(
        &client,
        attribute_definitions,
        table_name,
        key_schema,
        None,
        None,
        BillingMode::Provisioned,
        provisioned_throughput,
        None,
        None,
        None,
        TableClass::Standard,
        Some(false),
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_item() {
    let client = create_client().await;

    let table_name = "test-table".to_string();
    let mut key = HashMap::new();
    key.insert("id".to_string(), AttributeValue::S("test-id".to_string()));

    let result = delete_item(
        &client,
        table_name,
        Some(key),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_table() {
    let client = create_client().await;

    let table_name = "test-table".to_string();

    let result = delete_table(&client, table_name).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_item() {
    let client = create_client().await;

    let table_name = "test-table".to_string();
    let mut key = HashMap::new();
    key.insert("id".to_string(), AttributeValue::S("test-id".to_string()));

    let result = get_item(
        &client,
        table_name,
        Some(key),
        None,
        Some(false),
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_put_item() {
    let client = create_client().await;

    let table_name = "test-table".to_string();
    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S("test-id".to_string()));
    item.insert("name".to_string(), AttributeValue::S("test-name".to_string()));

    let result = put_item(
        &client,
        table_name,
        Some(item),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_item() {
    let client = create_client().await;

    let table_name = "test-table".to_string();
    let mut key = HashMap::new();
    key.insert("id".to_string(), AttributeValue::S("test-id".to_string()));

    let mut expr_values = HashMap::new();
    expr_values.insert(":n".to_string(), AttributeValue::S("updated-name".to_string()));

    let result = update_item(
        &client,
        table_name,
        Some(key),
        Some(ReturnValue::AllNew),
        None,
        None,
        None,
        None,
        Some("SET #n = :n".to_string()),
        Some(HashMap::from([("#n".to_string(), "name".to_string())])),
        Some(expr_values),
        None,
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_scan() {
    let client = create_client().await;

    let table_name = "test-table".to_string();

    let result = scan(
        &client,
        table_name,
        None,
        None,
        Some(10),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(false),
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_batch_write_item() {
    let client = create_client().await;

    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S("batch-id".to_string()));

    let write_request = WriteRequest::builder()
        .put_request(
            aws_sdk_dynamodb::types::PutRequest::builder()
                .set_item(Some(item))
                .build()
                .unwrap(),
        )
        .build();

    let request_items = HashMap::from([("test-table".to_string(), vec![write_request])]);

    let result = batch_write_item(
        &client,
        Some(request_items),
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_query() {
    let client = create_client().await;

    let table_name = "test-table".to_string();
    let mut key_conditions = HashMap::new();

    let condition = Condition::builder()
        .comparison_operator(ComparisonOperator::Eq)
        .attribute_value_list(AttributeValue::S("test-id".to_string()))
        .build()
        .expect("Failed to build condition");

    key_conditions.insert("id".to_string(), condition);

    let result = query(
        &client,
        table_name,
        None,
        Some(Select::AllAttributes),
        None,
        Some(10),
        Some(false),
        Some(key_conditions),
        None,
        None,
        Some(true),
        None,
        Some(ReturnConsumedCapacity::Total),
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
}
