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
        .checkout_sessions()
        .create()
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn checkout_sessions_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.checkout_sessions().retrieve().id(id).await;
}

#[tokio::test]
async fn checkout_sessions_preview() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .checkout_sessions()
        .preview()
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn payments_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.payments().create().body(Default::default()).await;
}

#[tokio::test]
async fn payments_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let payment_id = "payment_id";
    let _ = client.payments().retrieve().payment_id(payment_id).await;
}

#[tokio::test]
async fn payments_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.payments().list().query(serde_json::json!({})).await;
}

#[tokio::test]
async fn payments_retrieve_line_items() {
    let Some(client) = make_client() else {
        return;
    };
    let payment_id = "payment_id";
    let _ = client
        .payments()
        .retrieve_line_items()
        .payment_id(payment_id)
        .await;
}

#[tokio::test]
async fn subscriptions_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .subscriptions()
        .create()
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn subscriptions_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let subscription_id = "subscription_id";
    let _ = client
        .subscriptions()
        .retrieve()
        .subscription_id(subscription_id)
        .await;
}

#[tokio::test]
async fn subscriptions_update() {
    let Some(client) = make_client() else {
        return;
    };
    let subscription_id = "subscription_id";
    let _ = client
        .subscriptions()
        .update()
        .subscription_id(subscription_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn subscriptions_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .subscriptions()
        .list()
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn subscriptions_cancel_change_plan() {
    let Some(client) = make_client() else {
        return;
    };
    let subscription_id = "subscription_id";
    let _ = client
        .subscriptions()
        .cancel_change_plan()
        .subscription_id(subscription_id)
        .await;
}

#[tokio::test]
async fn subscriptions_change_plan() {
    let Some(client) = make_client() else {
        return;
    };
    let subscription_id = "subscription_id";
    let _ = client
        .subscriptions()
        .change_plan()
        .subscription_id(subscription_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn subscriptions_charge() {
    let Some(client) = make_client() else {
        return;
    };
    let subscription_id = "subscription_id";
    let _ = client
        .subscriptions()
        .charge()
        .subscription_id(subscription_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn subscriptions_preview_change_plan() {
    let Some(client) = make_client() else {
        return;
    };
    let subscription_id = "subscription_id";
    let _ = client
        .subscriptions()
        .preview_change_plan()
        .subscription_id(subscription_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn subscriptions_retrieve_credit_usage() {
    let Some(client) = make_client() else {
        return;
    };
    let subscription_id = "subscription_id";
    let _ = client
        .subscriptions()
        .retrieve_credit_usage()
        .subscription_id(subscription_id)
        .await;
}

#[tokio::test]
async fn subscriptions_retrieve_usage_history() {
    let Some(client) = make_client() else {
        return;
    };
    let subscription_id = "subscription_id";
    let _ = client
        .subscriptions()
        .retrieve_usage_history()
        .subscription_id(subscription_id)
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn subscriptions_update_payment_method() {
    let Some(client) = make_client() else {
        return;
    };
    let subscription_id = "subscription_id";
    let _ = client
        .subscriptions()
        .update_payment_method()
        .subscription_id(subscription_id)
        .body(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn invoices_payments_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let payment_id = "payment_id";
    let _ = client
        .invoices()
        .payments()
        .retrieve()
        .payment_id(payment_id)
        .await;
}

#[tokio::test]
async fn invoices_payments_retrieve_payout() {
    let Some(client) = make_client() else {
        return;
    };
    let payout_id = "payout_id";
    let _ = client
        .invoices()
        .payments()
        .retrieve_payout()
        .payout_id(payout_id)
        .await;
}

#[tokio::test]
async fn invoices_payments_retrieve_refund() {
    let Some(client) = make_client() else {
        return;
    };
    let refund_id = "refund_id";
    let _ = client
        .invoices()
        .payments()
        .retrieve_refund()
        .refund_id(refund_id)
        .await;
}

#[tokio::test]
async fn licenses_activate() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.licenses().activate().body(Default::default()).await;
}

#[tokio::test]
async fn licenses_deactivate() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .licenses()
        .deactivate()
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn licenses_validate() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.licenses().validate().body(Default::default()).await;
}

#[tokio::test]
async fn license_keys_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .license_keys()
        .create()
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn license_keys_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.license_keys().retrieve().id(id).await;
}

#[tokio::test]
async fn license_keys_update() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .license_keys()
        .update()
        .id(id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn license_keys_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .license_keys()
        .list()
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn license_key_instances_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.license_key_instances().retrieve().id(id).await;
}

#[tokio::test]
async fn license_key_instances_update() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .license_key_instances()
        .update()
        .id(id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn license_key_instances_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .license_key_instances()
        .list()
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn customers_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.customers().create().body(Default::default()).await;
}

#[tokio::test]
async fn customers_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let customer_id = "customer_id";
    let _ = client.customers().retrieve().customer_id(customer_id).await;
}

#[tokio::test]
async fn customers_update() {
    let Some(client) = make_client() else {
        return;
    };
    let customer_id = "customer_id";
    let _ = client
        .customers()
        .update()
        .customer_id(customer_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn customers_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.customers().list().query(serde_json::json!({})).await;
}

#[tokio::test]
async fn customers_delete_payment_method() {
    let Some(client) = make_client() else {
        return;
    };
    let customer_id = "customer_id";
    let payment_method_id = "payment_method_id";
    let _ = client
        .customers()
        .delete_payment_method()
        .customer_id(customer_id)
        .payment_method_id(payment_method_id)
        .await;
}

#[tokio::test]
async fn customers_list_credit_entitlements() {
    let Some(client) = make_client() else {
        return;
    };
    let customer_id = "customer_id";
    let _ = client
        .customers()
        .list_credit_entitlements()
        .customer_id(customer_id)
        .await;
}

#[tokio::test]
async fn customers_list_entitlements() {
    let Some(client) = make_client() else {
        return;
    };
    let customer_id = "customer_id";
    let _ = client
        .customers()
        .list_entitlements()
        .customer_id(customer_id)
        .await;
}

#[tokio::test]
async fn customers_retrieve_payment_methods() {
    let Some(client) = make_client() else {
        return;
    };
    let customer_id = "customer_id";
    let _ = client
        .customers()
        .retrieve_payment_methods()
        .customer_id(customer_id)
        .await;
}

#[tokio::test]
async fn customers_customer_portal_create() {
    let Some(client) = make_client() else {
        return;
    };
    let customer_id = "customer_id";
    let _ = client
        .customers()
        .customer_portal()
        .create()
        .customer_id(customer_id)
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn customers_wallets_list() {
    let Some(client) = make_client() else {
        return;
    };
    let customer_id = "customer_id";
    let _ = client
        .customers()
        .wallets()
        .list()
        .customer_id(customer_id)
        .await;
}

#[tokio::test]
async fn customers_wallets_ledger_entries_create() {
    let Some(client) = make_client() else {
        return;
    };
    let customer_id = "customer_id";
    let _ = client
        .customers()
        .wallets()
        .ledger_entries()
        .create()
        .customer_id(customer_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn customers_wallets_ledger_entries_list() {
    let Some(client) = make_client() else {
        return;
    };
    let customer_id = "customer_id";
    let _ = client
        .customers()
        .wallets()
        .ledger_entries()
        .list()
        .customer_id(customer_id)
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn refunds_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.refunds().create().body(Default::default()).await;
}

#[tokio::test]
async fn refunds_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let refund_id = "refund_id";
    let _ = client.refunds().retrieve().refund_id(refund_id).await;
}

#[tokio::test]
async fn refunds_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.refunds().list().query(serde_json::json!({})).await;
}

#[tokio::test]
async fn disputes_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let dispute_id = "dispute_id";
    let _ = client.disputes().retrieve().dispute_id(dispute_id).await;
}

#[tokio::test]
async fn disputes_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.disputes().list().query(serde_json::json!({})).await;
}

#[tokio::test]
async fn payouts_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.payouts().list().query(serde_json::json!({})).await;
}

#[tokio::test]
async fn payouts_breakup_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let payout_id = "payout_id";
    let _ = client
        .payouts()
        .breakup()
        .retrieve()
        .payout_id(payout_id)
        .await;
}

#[tokio::test]
async fn payouts_breakup_details_list() {
    let Some(client) = make_client() else {
        return;
    };
    let payout_id = "payout_id";
    let _ = client
        .payouts()
        .breakup()
        .details()
        .list()
        .payout_id(payout_id)
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn payouts_breakup_details_download_csv() {
    let Some(client) = make_client() else {
        return;
    };
    let payout_id = "payout_id";
    let _ = client
        .payouts()
        .breakup()
        .details()
        .download_csv()
        .payout_id(payout_id)
        .await;
}

#[tokio::test]
async fn products_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.products().create().body(Default::default()).await;
}

#[tokio::test]
async fn products_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.products().retrieve().id(id).await;
}

#[tokio::test]
async fn products_update() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .products()
        .update()
        .id(id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn products_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.products().list().query(serde_json::json!({})).await;
}

#[tokio::test]
async fn products_archive() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.products().archive().id(id).await;
}

#[tokio::test]
async fn products_unarchive() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.products().unarchive().id(id).await;
}

#[tokio::test]
async fn products_update_files() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .products()
        .update_files()
        .id(id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn products_images_update() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .products()
        .images()
        .update()
        .id(id)
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn products_short_links_create() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .products()
        .short_links()
        .create()
        .id(id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn products_short_links_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .products()
        .short_links()
        .list()
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn misc_list_supported_countries() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.misc().list_supported_countries().await;
}

#[tokio::test]
async fn discounts_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.discounts().create().body(Default::default()).await;
}

#[tokio::test]
async fn discounts_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let discount_id = "discount_id";
    let _ = client.discounts().retrieve().discount_id(discount_id).await;
}

#[tokio::test]
async fn discounts_update() {
    let Some(client) = make_client() else {
        return;
    };
    let discount_id = "discount_id";
    let _ = client
        .discounts()
        .update()
        .discount_id(discount_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn discounts_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.discounts().list().query(serde_json::json!({})).await;
}

#[tokio::test]
async fn discounts_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let discount_id = "discount_id";
    let _ = client.discounts().delete().discount_id(discount_id).await;
}

#[tokio::test]
async fn discounts_retrieve_by_code() {
    let Some(client) = make_client() else {
        return;
    };
    let code = "code";
    let _ = client.discounts().retrieve_by_code().code(code).await;
}

#[tokio::test]
async fn addons_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.addons().create().body(Default::default()).await;
}

#[tokio::test]
async fn addons_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.addons().retrieve().id(id).await;
}

#[tokio::test]
async fn addons_update() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .addons()
        .update()
        .id(id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn addons_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.addons().list().query(serde_json::json!({})).await;
}

#[tokio::test]
async fn addons_update_images() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.addons().update_images().id(id).await;
}

#[tokio::test]
async fn brands_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.brands().create().body(Default::default()).await;
}

#[tokio::test]
async fn brands_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.brands().retrieve().id(id).await;
}

#[tokio::test]
async fn brands_update() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .brands()
        .update()
        .id(id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn brands_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.brands().list().await;
}

#[tokio::test]
async fn brands_update_images() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.brands().update_images().id(id).await;
}

#[tokio::test]
async fn webhooks_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.webhooks().create().body(Default::default()).await;
}

#[tokio::test]
async fn webhooks_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let webhook_id = "webhook_id";
    let _ = client.webhooks().retrieve().webhook_id(webhook_id).await;
}

#[tokio::test]
async fn webhooks_update() {
    let Some(client) = make_client() else {
        return;
    };
    let webhook_id = "webhook_id";
    let _ = client
        .webhooks()
        .update()
        .webhook_id(webhook_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn webhooks_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.webhooks().list().query(serde_json::json!({})).await;
}

#[tokio::test]
async fn webhooks_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let webhook_id = "webhook_id";
    let _ = client.webhooks().delete().webhook_id(webhook_id).await;
}

#[tokio::test]
async fn webhooks_retrieve_secret() {
    let Some(client) = make_client() else {
        return;
    };
    let webhook_id = "webhook_id";
    let _ = client
        .webhooks()
        .retrieve_secret()
        .webhook_id(webhook_id)
        .await;
}

#[tokio::test]
async fn webhooks_headers_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let webhook_id = "webhook_id";
    let _ = client
        .webhooks()
        .headers()
        .retrieve()
        .webhook_id(webhook_id)
        .await;
}

#[tokio::test]
async fn webhooks_headers_update() {
    let Some(client) = make_client() else {
        return;
    };
    let webhook_id = "webhook_id";
    let _ = client
        .webhooks()
        .headers()
        .update()
        .webhook_id(webhook_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn usage_events_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let event_id = "event_id";
    let _ = client.usage_events().retrieve().event_id(event_id).await;
}

#[tokio::test]
async fn usage_events_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .usage_events()
        .list()
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn usage_events_ingest() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .usage_events()
        .ingest()
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn meters_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.meters().create().body(Default::default()).await;
}

#[tokio::test]
async fn meters_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.meters().retrieve().id(id).await;
}

#[tokio::test]
async fn meters_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client.meters().list().query(serde_json::json!({})).await;
}

#[tokio::test]
async fn meters_archive() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.meters().archive().id(id).await;
}

#[tokio::test]
async fn meters_unarchive() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.meters().unarchive().id(id).await;
}

#[tokio::test]
async fn balances_retrieve_ledger() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .balances()
        .retrieve_ledger()
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn credit_entitlements_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .credit_entitlements()
        .create()
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn credit_entitlements_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.credit_entitlements().retrieve().id(id).await;
}

#[tokio::test]
async fn credit_entitlements_update() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .credit_entitlements()
        .update()
        .id(id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn credit_entitlements_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .credit_entitlements()
        .list()
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn credit_entitlements_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.credit_entitlements().delete().id(id).await;
}

#[tokio::test]
async fn credit_entitlements_undelete() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.credit_entitlements().undelete().id(id).await;
}

#[tokio::test]
async fn credit_entitlements_balances_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let credit_entitlement_id = "credit_entitlement_id";
    let customer_id = "customer_id";
    let _ = client
        .credit_entitlements()
        .balances()
        .retrieve()
        .credit_entitlement_id(credit_entitlement_id)
        .customer_id(customer_id)
        .await;
}

#[tokio::test]
async fn credit_entitlements_balances_list() {
    let Some(client) = make_client() else {
        return;
    };
    let credit_entitlement_id = "credit_entitlement_id";
    let _ = client
        .credit_entitlements()
        .balances()
        .list()
        .credit_entitlement_id(credit_entitlement_id)
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn credit_entitlements_balances_create_ledger_entry() {
    let Some(client) = make_client() else {
        return;
    };
    let credit_entitlement_id = "credit_entitlement_id";
    let customer_id = "customer_id";
    let _ = client
        .credit_entitlements()
        .balances()
        .create_ledger_entry()
        .credit_entitlement_id(credit_entitlement_id)
        .customer_id(customer_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn credit_entitlements_balances_list_grants() {
    let Some(client) = make_client() else {
        return;
    };
    let credit_entitlement_id = "credit_entitlement_id";
    let customer_id = "customer_id";
    let _ = client
        .credit_entitlements()
        .balances()
        .list_grants()
        .credit_entitlement_id(credit_entitlement_id)
        .customer_id(customer_id)
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn credit_entitlements_balances_list_ledger() {
    let Some(client) = make_client() else {
        return;
    };
    let credit_entitlement_id = "credit_entitlement_id";
    let customer_id = "customer_id";
    let _ = client
        .credit_entitlements()
        .balances()
        .list_ledger()
        .credit_entitlement_id(credit_entitlement_id)
        .customer_id(customer_id)
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn entitlements_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .entitlements()
        .create()
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn entitlements_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.entitlements().retrieve().id(id).await;
}

#[tokio::test]
async fn entitlements_update() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .entitlements()
        .update()
        .id(id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn entitlements_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .entitlements()
        .list()
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn entitlements_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.entitlements().delete().id(id).await;
}

#[tokio::test]
async fn entitlements_files_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let file_id = "file_id";
    let _ = client
        .entitlements()
        .files()
        .delete()
        .id(id)
        .file_id(file_id)
        .await;
}

#[tokio::test]
async fn entitlements_files_upload() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.entitlements().files().upload().id(id).await;
}

#[tokio::test]
async fn entitlements_grants_list() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .entitlements()
        .grants()
        .list()
        .id(id)
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn entitlements_grants_revoke() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let grant_id = "grant_id";
    let _ = client
        .entitlements()
        .grants()
        .revoke()
        .id(id)
        .grant_id(grant_id)
        .await;
}

#[tokio::test]
async fn product_collections_create() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .product_collections()
        .create()
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn product_collections_retrieve() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.product_collections().retrieve().id(id).await;
}

#[tokio::test]
async fn product_collections_update() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .product_collections()
        .update()
        .id(id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn product_collections_list() {
    let Some(client) = make_client() else {
        return;
    };
    let _ = client
        .product_collections()
        .list()
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn product_collections_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.product_collections().delete().id(id).await;
}

#[tokio::test]
async fn product_collections_unarchive() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client.product_collections().unarchive().id(id).await;
}

#[tokio::test]
async fn product_collections_update_images() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .product_collections()
        .update_images()
        .id(id)
        .query(serde_json::json!({}))
        .await;
}

#[tokio::test]
async fn product_collections_groups_create() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let _ = client
        .product_collections()
        .groups()
        .create()
        .id(id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn product_collections_groups_update() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let group_id = "group_id";
    let _ = client
        .product_collections()
        .groups()
        .update()
        .id(id)
        .group_id(group_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn product_collections_groups_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let group_id = "group_id";
    let _ = client
        .product_collections()
        .groups()
        .delete()
        .id(id)
        .group_id(group_id)
        .await;
}

#[tokio::test]
async fn product_collections_groups_items_create() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let group_id = "group_id";
    let _ = client
        .product_collections()
        .groups()
        .items()
        .create()
        .id(id)
        .group_id(group_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn product_collections_groups_items_update() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let group_id = "group_id";
    let item_id = "item_id";
    let _ = client
        .product_collections()
        .groups()
        .items()
        .update()
        .id(id)
        .group_id(group_id)
        .item_id(item_id)
        .body(Default::default())
        .await;
}

#[tokio::test]
async fn product_collections_groups_items_delete() {
    let Some(client) = make_client() else {
        return;
    };
    let id = "id";
    let group_id = "group_id";
    let item_id = "item_id";
    let _ = client
        .product_collections()
        .groups()
        .items()
        .delete()
        .id(id)
        .group_id(group_id)
        .item_id(item_id)
        .await;
}
