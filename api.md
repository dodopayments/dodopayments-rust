# Dodo Payments API

# CheckoutSessions

Methods:

- <code title="post /checkouts">client.checkout_sessions_create(body: &amp;CheckoutSessionsCreateParams) -&gt; Result&lt;CheckoutSessionResponse&gt;</code>
- <code title="get /checkouts/{id}">client.checkout_sessions_retrieve(id: &amp;str) -&gt; Result&lt;CheckoutSessionStatus&gt;</code>
- <code title="post /checkouts/preview">client.checkout_sessions_preview(body: &amp;CheckoutSessionsCreateParams) -&gt; Result&lt;CheckoutSessionPreviewResponse&gt;</code>

# Payments

Methods:

- <code title="post /payments">client.payments_create(body: &amp;PaymentsCreateParams) -&gt; Result&lt;PaymentCreateResponse&gt;</code>
- <code title="get /payments/{payment_id}">client.payments_retrieve(payment_id: &amp;str) -&gt; Result&lt;Payment&gt;</code>
- <code title="get /payments">client.payments_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;PaymentListResponse&gt;&gt;</code>
- <code title="get /payments/{payment_id}/line-items">client.payments_retrieve_line_items(payment_id: &amp;str) -&gt; Result&lt;PaymentRetrieveLineItemsResponse&gt;</code>

# Subscriptions

Methods:

- <code title="post /subscriptions">client.subscriptions_create(body: &amp;SubscriptionsCreateParams) -&gt; Result&lt;SubscriptionCreateResponse&gt;</code>
- <code title="get /subscriptions/{subscription_id}">client.subscriptions_retrieve(subscription_id: &amp;str) -&gt; Result&lt;Subscription&gt;</code>
- <code title="patch /subscriptions/{subscription_id}">client.subscriptions_update(subscription_id: &amp;str, body: &amp;SubscriptionsUpdateParams) -&gt; Result&lt;Subscription&gt;</code>
- <code title="get /subscriptions">client.subscriptions_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;SubscriptionListResponse&gt;&gt;</code>
- <code title="delete /subscriptions/{subscription_id}/change-plan/scheduled">client.subscriptions_cancel_change_plan(subscription_id: &amp;str) -&gt; Result&lt;()&gt;</code>
- <code title="post /subscriptions/{subscription_id}/change-plan">client.subscriptions_change_plan(subscription_id: &amp;str, body: &amp;SubscriptionsChangePlanParams) -&gt; Result&lt;()&gt;</code>
- <code title="post /subscriptions/{subscription_id}/charge">client.subscriptions_charge(subscription_id: &amp;str, body: &amp;SubscriptionsChargeParams) -&gt; Result&lt;SubscriptionChargeResponse&gt;</code>
- <code title="post /subscriptions/{subscription_id}/change-plan/preview">client.subscriptions_preview_change_plan(subscription_id: &amp;str, body: &amp;SubscriptionsChangePlanParams) -&gt; Result&lt;SubscriptionPreviewChangePlanResponse&gt;</code>
- <code title="get /subscriptions/{subscription_id}/credit-usage">client.subscriptions_retrieve_credit_usage(subscription_id: &amp;str) -&gt; Result&lt;SubscriptionRetrieveCreditUsageResponse&gt;</code>
- <code title="get /subscriptions/{subscription_id}/usage-history">client.subscriptions_retrieve_usage_history(subscription_id: &amp;str, query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;SubscriptionRetrieveUsageHistoryResponse&gt;&gt;</code>
- <code title="post /subscriptions/{subscription_id}/update-payment-method">client.subscriptions_update_payment_method(subscription_id: &amp;str, body: &amp;serde_json::Value) -&gt; Result&lt;SubscriptionUpdatePaymentMethodResponse&gt;</code>

# Invoices

## Payments

Methods:

- <code title="get /invoices/payments/{payment_id}">client.invoices_payments_retrieve(payment_id: &amp;str) -&gt; Result&lt;Vec&lt;u8&gt;&gt;</code>
- <code title="get /invoices/payouts/{payout_id}">client.invoices_payments_retrieve_payout(payout_id: &amp;str) -&gt; Result&lt;Vec&lt;u8&gt;&gt;</code>
- <code title="get /invoices/refunds/{refund_id}">client.invoices_payments_retrieve_refund(refund_id: &amp;str) -&gt; Result&lt;Vec&lt;u8&gt;&gt;</code>

# Licenses

Methods:

- <code title="post /licenses/activate">client.licenses_activate(body: &amp;LicensesActivateParams) -&gt; Result&lt;LicenseActivateResponse&gt;</code>
- <code title="post /licenses/deactivate">client.licenses_deactivate(body: &amp;LicensesDeactivateParams) -&gt; Result&lt;()&gt;</code>
- <code title="post /licenses/validate">client.licenses_validate(body: &amp;LicensesValidateParams) -&gt; Result&lt;LicenseValidateResponse&gt;</code>

# LicenseKeys

Methods:

- <code title="post /license_keys">client.license_keys_create(body: &amp;LicenseKeysCreateParams) -&gt; Result&lt;LicenseKey&gt;</code>
- <code title="get /license_keys/{id}">client.license_keys_retrieve(id: &amp;str) -&gt; Result&lt;LicenseKey&gt;</code>
- <code title="patch /license_keys/{id}">client.license_keys_update(id: &amp;str, body: &amp;LicenseKeysUpdateParams) -&gt; Result&lt;LicenseKey&gt;</code>
- <code title="get /license_keys">client.license_keys_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;LicenseKey&gt;&gt;</code>

# LicenseKeyInstances

Methods:

- <code title="get /license_key_instances/{id}">client.license_key_instances_retrieve(id: &amp;str) -&gt; Result&lt;LicenseKeyInstance&gt;</code>
- <code title="patch /license_key_instances/{id}">client.license_key_instances_update(id: &amp;str, body: &amp;LicenseKeyInstancesUpdateParams) -&gt; Result&lt;LicenseKeyInstance&gt;</code>
- <code title="get /license_key_instances">client.license_key_instances_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;LicenseKeyInstance&gt;&gt;</code>

# Customers

Methods:

- <code title="post /customers">client.customers_create(body: &amp;CustomersCreateParams) -&gt; Result&lt;Customer&gt;</code>
- <code title="get /customers/{customer_id}">client.customers_retrieve(customer_id: &amp;str) -&gt; Result&lt;Customer&gt;</code>
- <code title="patch /customers/{customer_id}">client.customers_update(customer_id: &amp;str, body: &amp;CustomersUpdateParams) -&gt; Result&lt;Customer&gt;</code>
- <code title="get /customers">client.customers_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;Customer&gt;&gt;</code>
- <code title="delete /customers/{customer_id}/payment-methods/{payment_method_id}">client.customers_delete_payment_method(customer_id: &amp;str, payment_method_id: &amp;str) -&gt; Result&lt;()&gt;</code>
- <code title="get /customers/{customer_id}/credit-entitlements">client.customers_list_credit_entitlements(customer_id: &amp;str) -&gt; Result&lt;CustomerListCreditEntitlementsResponse&gt;</code>
- <code title="get /customers/{customer_id}/entitlements">client.customers_list_entitlements(customer_id: &amp;str) -&gt; Result&lt;CustomerListEntitlementsResponse&gt;</code>
- <code title="get /customers/{customer_id}/payment-methods">client.customers_retrieve_payment_methods(customer_id: &amp;str) -&gt; Result&lt;CustomerRetrievePaymentMethodsResponse&gt;</code>

## CustomerPortal

Methods:

- <code title="post /customers/{customer_id}/customer-portal/session">client.customers_customer_portal_create(customer_id: &amp;str, query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;CustomerPortalSession&gt;</code>

## Wallets

Methods:

- <code title="get /customers/{customer_id}/wallets">client.customers_wallets_list(customer_id: &amp;str) -&gt; Result&lt;WalletListResponse&gt;</code>

### LedgerEntries

Methods:

- <code title="post /customers/{customer_id}/wallets/ledger-entries">client.customers_wallets_ledger_entries_create(customer_id: &amp;str, body: &amp;CustomersWalletsLedgerEntriesCreateParams) -&gt; Result&lt;CustomerWallet&gt;</code>
- <code title="get /customers/{customer_id}/wallets/ledger-entries">client.customers_wallets_ledger_entries_list(customer_id: &amp;str, query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;CustomerWalletTransaction&gt;&gt;</code>

# Refunds

Methods:

- <code title="post /refunds">client.refunds_create(body: &amp;RefundsCreateParams) -&gt; Result&lt;Refund&gt;</code>
- <code title="get /refunds/{refund_id}">client.refunds_retrieve(refund_id: &amp;str) -&gt; Result&lt;Refund&gt;</code>
- <code title="get /refunds">client.refunds_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;RefundListItem&gt;&gt;</code>

# Disputes

Methods:

- <code title="get /disputes/{dispute_id}">client.disputes_retrieve(dispute_id: &amp;str) -&gt; Result&lt;GetDispute&gt;</code>
- <code title="get /disputes">client.disputes_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;DisputeListResponse&gt;&gt;</code>

# Payouts

Methods:

- <code title="get /payouts">client.payouts_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;PayoutListResponse&gt;&gt;</code>

## Breakup

Methods:

- <code title="get /payouts/{payout_id}/breakup">client.payouts_breakup_retrieve(payout_id: &amp;str) -&gt; Result&lt;BreakupRetrieveResponse&gt;</code>

### Details

Methods:

- <code title="get /payouts/{payout_id}/breakup/details">client.payouts_breakup_details_list(payout_id: &amp;str, query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;DetailListResponse&gt;&gt;</code>
- <code title="get /payouts/{payout_id}/breakup/details/csv">client.payouts_breakup_details_download_csv(payout_id: &amp;str) -&gt; Result&lt;()&gt;</code>

# Products

Methods:

- <code title="post /products">client.products_create(body: &amp;ProductsCreateParams) -&gt; Result&lt;Product&gt;</code>
- <code title="get /products/{id}">client.products_retrieve(id: &amp;str) -&gt; Result&lt;Product&gt;</code>
- <code title="patch /products/{id}">client.products_update(id: &amp;str, body: &amp;ProductsUpdateParams) -&gt; Result&lt;()&gt;</code>
- <code title="get /products">client.products_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;ProductListResponse&gt;&gt;</code>
- <code title="delete /products/{id}">client.products_archive(id: &amp;str) -&gt; Result&lt;()&gt;</code>
- <code title="post /products/{id}/unarchive">client.products_unarchive(id: &amp;str) -&gt; Result&lt;()&gt;</code>
- <code title="put /products/{id}/files">client.products_update_files(id: &amp;str, body: &amp;ProductsUpdateFilesParams) -&gt; Result&lt;ProductUpdateFilesResponse&gt;</code>

## Images

Methods:

- <code title="put /products/{id}/images">client.products_images_update(id: &amp;str, query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;ImageUpdateResponse&gt;</code>

## ShortLinks

Methods:

- <code title="post /products/{id}/short_links">client.products_short_links_create(id: &amp;str, body: &amp;ProductsShortLinksCreateParams) -&gt; Result&lt;ShortLinkCreateResponse&gt;</code>
- <code title="get /products/short_links">client.products_short_links_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;ShortLinkListResponse&gt;&gt;</code>

# Misc

Methods:

- <code title="get /checkout/supported_countries">client.misc_list_supported_countries() -&gt; Result&lt;MiscListSupportedCountriesResponse&gt;</code>

# Discounts

Methods:

- <code title="post /discounts">client.discounts_create(body: &amp;DiscountsCreateParams) -&gt; Result&lt;Discount&gt;</code>
- <code title="get /discounts/{discount_id}">client.discounts_retrieve(discount_id: &amp;str) -&gt; Result&lt;Discount&gt;</code>
- <code title="patch /discounts/{discount_id}">client.discounts_update(discount_id: &amp;str, body: &amp;DiscountsUpdateParams) -&gt; Result&lt;Discount&gt;</code>
- <code title="get /discounts">client.discounts_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;Discount&gt;&gt;</code>
- <code title="delete /discounts/{discount_id}">client.discounts_delete(discount_id: &amp;str) -&gt; Result&lt;()&gt;</code>
- <code title="get /discounts/code/{code}">client.discounts_retrieve_by_code(code: &amp;str) -&gt; Result&lt;Discount&gt;</code>

# Addons

Methods:

- <code title="post /addons">client.addons_create(body: &amp;AddonsCreateParams) -&gt; Result&lt;AddonResponse&gt;</code>
- <code title="get /addons/{id}">client.addons_retrieve(id: &amp;str) -&gt; Result&lt;AddonResponse&gt;</code>
- <code title="patch /addons/{id}">client.addons_update(id: &amp;str, body: &amp;AddonsUpdateParams) -&gt; Result&lt;AddonResponse&gt;</code>
- <code title="get /addons">client.addons_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;AddonResponse&gt;&gt;</code>
- <code title="put /addons/{id}/images">client.addons_update_images(id: &amp;str) -&gt; Result&lt;AddonUpdateImagesResponse&gt;</code>

# Brands

Methods:

- <code title="post /brands">client.brands_create(body: &amp;BrandsCreateParams) -&gt; Result&lt;Brand&gt;</code>
- <code title="get /brands/{id}">client.brands_retrieve(id: &amp;str) -&gt; Result&lt;Brand&gt;</code>
- <code title="patch /brands/{id}">client.brands_update(id: &amp;str, body: &amp;BrandsUpdateParams) -&gt; Result&lt;Brand&gt;</code>
- <code title="get /brands">client.brands_list() -&gt; Result&lt;BrandListResponse&gt;</code>
- <code title="put /brands/{id}/images">client.brands_update_images(id: &amp;str) -&gt; Result&lt;BrandUpdateImagesResponse&gt;</code>

# Webhooks

Methods:

- <code title="post /webhooks">client.webhooks_create(body: &amp;WebhooksCreateParams) -&gt; Result&lt;WebhookDetails&gt;</code>
- <code title="get /webhooks/{webhook_id}">client.webhooks_retrieve(webhook_id: &amp;str) -&gt; Result&lt;WebhookDetails&gt;</code>
- <code title="patch /webhooks/{webhook_id}">client.webhooks_update(webhook_id: &amp;str, body: &amp;WebhooksUpdateParams) -&gt; Result&lt;WebhookDetails&gt;</code>
- <code title="get /webhooks">client.webhooks_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;CursorPagePagination&lt;WebhookDetails&gt;&gt;</code>
- <code title="delete /webhooks/{webhook_id}">client.webhooks_delete(webhook_id: &amp;str) -&gt; Result&lt;()&gt;</code>
- <code title="get /webhooks/{webhook_id}/secret">client.webhooks_retrieve_secret(webhook_id: &amp;str) -&gt; Result&lt;WebhookRetrieveSecretResponse&gt;</code>

## Headers

Methods:

- <code title="get /webhooks/{webhook_id}/headers">client.webhooks_headers_retrieve(webhook_id: &amp;str) -&gt; Result&lt;HeaderRetrieveResponse&gt;</code>
- <code title="patch /webhooks/{webhook_id}/headers">client.webhooks_headers_update(webhook_id: &amp;str, body: &amp;WebhooksHeadersUpdateParams) -&gt; Result&lt;()&gt;</code>

# UsageEvents

Methods:

- <code title="get /events/{event_id}">client.usage_events_retrieve(event_id: &amp;str) -&gt; Result&lt;Event&gt;</code>
- <code title="get /events">client.usage_events_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;Event&gt;&gt;</code>
- <code title="post /events/ingest">client.usage_events_ingest(body: &amp;UsageEventsIngestParams) -&gt; Result&lt;UsageEventIngestResponse&gt;</code>

# Meters

Methods:

- <code title="post /meters">client.meters_create(body: &amp;MetersCreateParams) -&gt; Result&lt;Meter&gt;</code>
- <code title="get /meters/{id}">client.meters_retrieve(id: &amp;str) -&gt; Result&lt;Meter&gt;</code>
- <code title="get /meters">client.meters_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;Meter&gt;&gt;</code>
- <code title="delete /meters/{id}">client.meters_archive(id: &amp;str) -&gt; Result&lt;()&gt;</code>
- <code title="post /meters/{id}/unarchive">client.meters_unarchive(id: &amp;str) -&gt; Result&lt;()&gt;</code>

# Balances

Methods:

- <code title="get /balances/ledger">client.balances_retrieve_ledger(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;BalanceLedgerEntry&gt;&gt;</code>

# CreditEntitlements

Methods:

- <code title="post /credit-entitlements">client.credit_entitlements_create(body: &amp;CreditEntitlementsCreateParams) -&gt; Result&lt;CreditEntitlement&gt;</code>
- <code title="get /credit-entitlements/{id}">client.credit_entitlements_retrieve(id: &amp;str) -&gt; Result&lt;CreditEntitlement&gt;</code>
- <code title="patch /credit-entitlements/{id}">client.credit_entitlements_update(id: &amp;str, body: &amp;CreditEntitlementsUpdateParams) -&gt; Result&lt;()&gt;</code>
- <code title="get /credit-entitlements">client.credit_entitlements_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;CreditEntitlement&gt;&gt;</code>
- <code title="delete /credit-entitlements/{id}">client.credit_entitlements_delete(id: &amp;str) -&gt; Result&lt;()&gt;</code>
- <code title="post /credit-entitlements/{id}/undelete">client.credit_entitlements_undelete(id: &amp;str) -&gt; Result&lt;()&gt;</code>

## Balances

Methods:

- <code title="get /credit-entitlements/{credit_entitlement_id}/balances/{customer_id}">client.credit_entitlements_balances_retrieve(credit_entitlement_id: &amp;str, customer_id: &amp;str) -&gt; Result&lt;CustomerCreditBalance&gt;</code>
- <code title="get /credit-entitlements/{credit_entitlement_id}/balances">client.credit_entitlements_balances_list(credit_entitlement_id: &amp;str, query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;CustomerCreditBalance&gt;&gt;</code>
- <code title="post /credit-entitlements/{credit_entitlement_id}/balances/{customer_id}/ledger-entries">client.credit_entitlements_balances_create_ledger_entry(credit_entitlement_id: &amp;str, customer_id: &amp;str, body: &amp;CreditEntitlementsBalancesCreateLedgerEntryParams) -&gt; Result&lt;BalanceCreateLedgerEntryResponse&gt;</code>
- <code title="get /credit-entitlements/{credit_entitlement_id}/balances/{customer_id}/grants">client.credit_entitlements_balances_list_grants(credit_entitlement_id: &amp;str, customer_id: &amp;str, query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;BalanceListGrantsResponse&gt;&gt;</code>
- <code title="get /credit-entitlements/{credit_entitlement_id}/balances/{customer_id}/ledger">client.credit_entitlements_balances_list_ledger(credit_entitlement_id: &amp;str, customer_id: &amp;str, query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;CreditLedgerEntry&gt;&gt;</code>

# Entitlements

Methods:

- <code title="post /entitlements">client.entitlements_create(body: &amp;EntitlementsCreateParams) -&gt; Result&lt;Entitlement&gt;</code>
- <code title="get /entitlements/{id}">client.entitlements_retrieve(id: &amp;str) -&gt; Result&lt;Entitlement&gt;</code>
- <code title="patch /entitlements/{id}">client.entitlements_update(id: &amp;str, body: &amp;EntitlementsUpdateParams) -&gt; Result&lt;Entitlement&gt;</code>
- <code title="get /entitlements">client.entitlements_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;Entitlement&gt;&gt;</code>
- <code title="delete /entitlements/{id}">client.entitlements_delete(id: &amp;str) -&gt; Result&lt;()&gt;</code>

## Files

Methods:

- <code title="delete /entitlements/{id}/files/{file_id}">client.entitlements_files_delete(id: &amp;str, file_id: &amp;str) -&gt; Result&lt;()&gt;</code>
- <code title="post /entitlements/{id}/files">client.entitlements_files_upload(id: &amp;str) -&gt; Result&lt;FileUploadResponse&gt;</code>

## Grants

Methods:

- <code title="get /entitlements/{id}/grants">client.entitlements_grants_list(id: &amp;str, query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;EntitlementGrant&gt;&gt;</code>
- <code title="delete /entitlements/{id}/grants/{grant_id}">client.entitlements_grants_revoke(id: &amp;str, grant_id: &amp;str) -&gt; Result&lt;EntitlementGrant&gt;</code>

# ProductCollections

Methods:

- <code title="post /product-collections">client.product_collections_create(body: &amp;ProductCollectionsCreateParams) -&gt; Result&lt;ProductCollection&gt;</code>
- <code title="get /product-collections/{id}">client.product_collections_retrieve(id: &amp;str) -&gt; Result&lt;ProductCollection&gt;</code>
- <code title="patch /product-collections/{id}">client.product_collections_update(id: &amp;str, body: &amp;ProductCollectionsUpdateParams) -&gt; Result&lt;()&gt;</code>
- <code title="get /product-collections">client.product_collections_list(query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;DefaultPageNumberPagination&lt;ProductCollectionListResponse&gt;&gt;</code>
- <code title="delete /product-collections/{id}">client.product_collections_delete(id: &amp;str) -&gt; Result&lt;()&gt;</code>
- <code title="post /product-collections/{id}/unarchive">client.product_collections_unarchive(id: &amp;str) -&gt; Result&lt;ProductCollectionUnarchiveResponse&gt;</code>
- <code title="put /product-collections/{id}/images">client.product_collections_update_images(id: &amp;str, query: Option&lt;&amp;serde_json::Value&gt;) -&gt; Result&lt;ProductCollectionUpdateImagesResponse&gt;</code>

## Groups

Methods:

- <code title="post /product-collections/{id}/groups">client.product_collections_groups_create(id: &amp;str, body: &amp;ProductCollectionsGroupsCreateParams) -&gt; Result&lt;ProductCollectionGroupResponse&gt;</code>
- <code title="patch /product-collections/{id}/groups/{group_id}">client.product_collections_groups_update(id: &amp;str, group_id: &amp;str, body: &amp;ProductCollectionsGroupsUpdateParams) -&gt; Result&lt;()&gt;</code>
- <code title="delete /product-collections/{id}/groups/{group_id}">client.product_collections_groups_delete(id: &amp;str, group_id: &amp;str) -&gt; Result&lt;()&gt;</code>

### Items

Methods:

- <code title="post /product-collections/{id}/groups/{group_id}/items">client.product_collections_groups_items_create(id: &amp;str, group_id: &amp;str, body: &amp;ProductCollectionsGroupsItemsCreateParams) -&gt; Result&lt;ItemCreateResponse&gt;</code>
- <code title="patch /product-collections/{id}/groups/{group_id}/items/{item_id}">client.product_collections_groups_items_update(id: &amp;str, group_id: &amp;str, item_id: &amp;str, body: &amp;ProductCollectionsGroupsItemsUpdateParams) -&gt; Result&lt;()&gt;</code>
- <code title="delete /product-collections/{id}/groups/{group_id}/items/{item_id}">client.product_collections_groups_items_delete(id: &amp;str, group_id: &amp;str, item_id: &amp;str) -&gt; Result&lt;()&gt;</code>
