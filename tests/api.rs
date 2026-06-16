use dodopayments::{Client, ClientConfig};

fn make_client() -> Option<Client> {
    let base_url = std::env::var("TEST_API_BASE_URL").ok()?;
    Client::new(ClientConfig::new(base_url).with_api_key("My API Key")).ok()
}

#[tokio::test]
async fn checkout_sessions_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .checkout_sessions_create(&serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn checkout_sessions_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.checkout_sessions_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn checkout_sessions_preview() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .checkout_sessions_preview(&serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn payments_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.payments_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn payments_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.payments_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn payments_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.payments_list().await;
}

#[tokio::test]
async fn payments_retrieve_line_items() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.payments_retrieve_line_items("REPLACE_ME").await;
}

#[tokio::test]
async fn subscriptions_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.subscriptions_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn subscriptions_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.subscriptions_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn subscriptions_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .subscriptions_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn subscriptions_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.subscriptions_list().await;
}

#[tokio::test]
async fn subscriptions_cancel_change_plan() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.subscriptions_cancel_change_plan("REPLACE_ME").await;
}

#[tokio::test]
async fn subscriptions_change_plan() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .subscriptions_change_plan("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn subscriptions_charge() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .subscriptions_charge("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn subscriptions_preview_change_plan() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .subscriptions_preview_change_plan("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn subscriptions_retrieve_credit_usage() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .subscriptions_retrieve_credit_usage("REPLACE_ME")
        .await;
}

#[tokio::test]
async fn subscriptions_retrieve_usage_history() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .subscriptions_retrieve_usage_history("REPLACE_ME")
        .await;
}

#[tokio::test]
async fn subscriptions_update_payment_method() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .subscriptions_update_payment_method("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn invoices_payments_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.invoices_payments_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn invoices_payments_retrieve_payout() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.invoices_payments_retrieve_payout("REPLACE_ME").await;
}

#[tokio::test]
async fn invoices_payments_retrieve_refund() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.invoices_payments_retrieve_refund("REPLACE_ME").await;
}

#[tokio::test]
async fn licenses_activate() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.licenses_activate(&serde_json::json!({})).await;
}

#[tokio::test]
async fn licenses_deactivate() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.licenses_deactivate(&serde_json::json!({})).await;
}

#[tokio::test]
async fn licenses_validate() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.licenses_validate(&serde_json::json!({})).await;
}

#[tokio::test]
async fn license_keys_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.license_keys_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn license_keys_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.license_keys_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn license_keys_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .license_keys_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn license_keys_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.license_keys_list().await;
}

#[tokio::test]
async fn license_key_instances_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.license_key_instances_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn license_key_instances_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .license_key_instances_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn license_key_instances_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.license_key_instances_list().await;
}

#[tokio::test]
async fn customers_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.customers_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn customers_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.customers_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn customers_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .customers_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn customers_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.customers_list().await;
}

#[tokio::test]
async fn customers_delete_payment_method() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .customers_delete_payment_method("REPLACE_ME", "REPLACE_ME")
        .await;
}

#[tokio::test]
async fn customers_list_credit_entitlements() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .customers_list_credit_entitlements("REPLACE_ME")
        .await;
}

#[tokio::test]
async fn customers_list_entitlements() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.customers_list_entitlements("REPLACE_ME").await;
}

#[tokio::test]
async fn customers_retrieve_payment_methods() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .customers_retrieve_payment_methods("REPLACE_ME")
        .await;
}

#[tokio::test]
async fn customers_customer_portal_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.customers_customer_portal_create("REPLACE_ME").await;
}

#[tokio::test]
async fn customers_wallets_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.customers_wallets_list("REPLACE_ME").await;
}

#[tokio::test]
async fn customers_wallets_ledger_entries_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .customers_wallets_ledger_entries_create("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn customers_wallets_ledger_entries_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .customers_wallets_ledger_entries_list("REPLACE_ME")
        .await;
}

#[tokio::test]
async fn refunds_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.refunds_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn refunds_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.refunds_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn refunds_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.refunds_list().await;
}

#[tokio::test]
async fn disputes_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.disputes_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn disputes_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.disputes_list().await;
}

#[tokio::test]
async fn payouts_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.payouts_list().await;
}

#[tokio::test]
async fn payouts_breakup_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.payouts_breakup_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn payouts_breakup_details_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.payouts_breakup_details_list("REPLACE_ME").await;
}

#[tokio::test]
async fn payouts_breakup_details_download_csv() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .payouts_breakup_details_download_csv("REPLACE_ME")
        .await;
}

#[tokio::test]
async fn products_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.products_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn products_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.products_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn products_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .products_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn products_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.products_list().await;
}

#[tokio::test]
async fn products_archive() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.products_archive("REPLACE_ME").await;
}

#[tokio::test]
async fn products_unarchive() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.products_unarchive("REPLACE_ME").await;
}

#[tokio::test]
async fn products_update_files() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .products_update_files("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn products_images_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.products_images_update("REPLACE_ME").await;
}

#[tokio::test]
async fn products_short_links_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .products_short_links_create("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn products_short_links_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.products_short_links_list().await;
}

#[tokio::test]
async fn misc_list_supported_countries() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.misc_list_supported_countries().await;
}

#[tokio::test]
async fn discounts_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.discounts_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn discounts_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.discounts_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn discounts_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .discounts_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn discounts_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.discounts_list().await;
}

#[tokio::test]
async fn discounts_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.discounts_delete("REPLACE_ME").await;
}

#[tokio::test]
async fn discounts_retrieve_by_code() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.discounts_retrieve_by_code("REPLACE_ME").await;
}

#[tokio::test]
async fn addons_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.addons_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn addons_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.addons_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn addons_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .addons_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn addons_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.addons_list().await;
}

#[tokio::test]
async fn addons_update_images() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.addons_update_images("REPLACE_ME").await;
}

#[tokio::test]
async fn brands_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.brands_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn brands_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.brands_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn brands_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .brands_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn brands_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.brands_list().await;
}

#[tokio::test]
async fn brands_update_images() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.brands_update_images("REPLACE_ME").await;
}

#[tokio::test]
async fn webhooks_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.webhooks_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn webhooks_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.webhooks_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn webhooks_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .webhooks_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn webhooks_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.webhooks_list().await;
}

#[tokio::test]
async fn webhooks_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.webhooks_delete("REPLACE_ME").await;
}

#[tokio::test]
async fn webhooks_retrieve_secret() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.webhooks_retrieve_secret("REPLACE_ME").await;
}

#[tokio::test]
async fn webhooks_headers_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.webhooks_headers_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn webhooks_headers_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .webhooks_headers_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn usage_events_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.usage_events_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn usage_events_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.usage_events_list().await;
}

#[tokio::test]
async fn usage_events_ingest() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.usage_events_ingest(&serde_json::json!({})).await;
}

#[tokio::test]
async fn meters_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.meters_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn meters_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.meters_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn meters_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.meters_list().await;
}

#[tokio::test]
async fn meters_archive() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.meters_archive("REPLACE_ME").await;
}

#[tokio::test]
async fn meters_unarchive() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.meters_unarchive("REPLACE_ME").await;
}

#[tokio::test]
async fn balances_retrieve_ledger() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.balances_retrieve_ledger().await;
}

#[tokio::test]
async fn credit_entitlements_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .credit_entitlements_create(&serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn credit_entitlements_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.credit_entitlements_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn credit_entitlements_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .credit_entitlements_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn credit_entitlements_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.credit_entitlements_list().await;
}

#[tokio::test]
async fn credit_entitlements_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.credit_entitlements_delete("REPLACE_ME").await;
}

#[tokio::test]
async fn credit_entitlements_undelete() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.credit_entitlements_undelete("REPLACE_ME").await;
}

#[tokio::test]
async fn credit_entitlements_balances_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .credit_entitlements_balances_retrieve("REPLACE_ME", "REPLACE_ME")
        .await;
}

#[tokio::test]
async fn credit_entitlements_balances_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.credit_entitlements_balances_list("REPLACE_ME").await;
}

#[tokio::test]
async fn credit_entitlements_balances_create_ledger_entry() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .credit_entitlements_balances_create_ledger_entry(
            "REPLACE_ME",
            "REPLACE_ME",
            &serde_json::json!({}),
        )
        .await;
}

#[tokio::test]
async fn credit_entitlements_balances_list_grants() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .credit_entitlements_balances_list_grants("REPLACE_ME", "REPLACE_ME")
        .await;
}

#[tokio::test]
async fn credit_entitlements_balances_list_ledger() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .credit_entitlements_balances_list_ledger("REPLACE_ME", "REPLACE_ME")
        .await;
}

#[tokio::test]
async fn entitlements_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.entitlements_create(&serde_json::json!({})).await;
}

#[tokio::test]
async fn entitlements_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.entitlements_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn entitlements_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .entitlements_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn entitlements_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.entitlements_list().await;
}

#[tokio::test]
async fn entitlements_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.entitlements_delete("REPLACE_ME").await;
}

#[tokio::test]
async fn entitlements_files_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .entitlements_files_delete("REPLACE_ME", "REPLACE_ME")
        .await;
}

#[tokio::test]
async fn entitlements_files_upload() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.entitlements_files_upload("REPLACE_ME").await;
}

#[tokio::test]
async fn entitlements_grants_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.entitlements_grants_list("REPLACE_ME").await;
}

#[tokio::test]
async fn entitlements_grants_revoke() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .entitlements_grants_revoke("REPLACE_ME", "REPLACE_ME")
        .await;
}

#[tokio::test]
async fn product_collections_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .product_collections_create(&serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn product_collections_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.product_collections_retrieve("REPLACE_ME").await;
}

#[tokio::test]
async fn product_collections_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .product_collections_update("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn product_collections_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.product_collections_list().await;
}

#[tokio::test]
async fn product_collections_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.product_collections_delete("REPLACE_ME").await;
}

#[tokio::test]
async fn product_collections_unarchive() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.product_collections_unarchive("REPLACE_ME").await;
}

#[tokio::test]
async fn product_collections_update_images() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.product_collections_update_images("REPLACE_ME").await;
}

#[tokio::test]
async fn product_collections_groups_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .product_collections_groups_create("REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn product_collections_groups_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .product_collections_groups_update("REPLACE_ME", "REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn product_collections_groups_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .product_collections_groups_delete("REPLACE_ME", "REPLACE_ME")
        .await;
}

#[tokio::test]
async fn product_collections_groups_items_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .product_collections_groups_items_create("REPLACE_ME", "REPLACE_ME", &serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn product_collections_groups_items_update() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .product_collections_groups_items_update(
            "REPLACE_ME",
            "REPLACE_ME",
            "REPLACE_ME",
            &serde_json::json!({}),
        )
        .await;
}

#[tokio::test]
async fn product_collections_groups_items_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .product_collections_groups_items_delete("REPLACE_ME", "REPLACE_ME", "REPLACE_ME")
        .await;
}
