# Dodo Payments API

# CheckoutSessions

Methods:

- <code title="post /checkouts">client.checkout_sessions().create().body(body: CheckoutSessionsCreateParams) -&gt; Result&lt;CheckoutSessionResponse&gt;</code>
- <code title="get /checkouts/{id}">client.checkout_sessions().retrieve().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;CheckoutSessionStatus&gt;</code>
- <code title="post /checkouts/preview">client.checkout_sessions().preview().body(body: CheckoutSessionsCreateParams) -&gt; Result&lt;CheckoutSessionPreviewResponse&gt;</code>

# Payments

Methods:

- <code title="post /payments">client.payments().create().body(body: PaymentsCreateParams) -&gt; Result&lt;PaymentCreateResponse&gt;</code>
- <code title="get /payments/{payment_id}">client.payments().retrieve().payment_id(payment_id: impl Into&lt;String&gt;) -&gt; Result&lt;Payment&gt;</code>
- <code title="get /payments">client.payments().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;PaymentListResponse&gt;&gt;</code>
- <code title="get /payments/{payment_id}/line-items">client.payments().retrieve_line_items().payment_id(payment_id: impl Into&lt;String&gt;) -&gt; Result&lt;PaymentRetrieveLineItemsResponse&gt;</code>

# Subscriptions

Methods:

- <code title="post /subscriptions">client.subscriptions().create().body(body: SubscriptionsCreateParams) -&gt; Result&lt;SubscriptionCreateResponse&gt;</code>
- <code title="get /subscriptions/{subscription_id}">client.subscriptions().retrieve().subscription_id(subscription_id: impl Into&lt;String&gt;) -&gt; Result&lt;Subscription&gt;</code>
- <code title="patch /subscriptions/{subscription_id}">client.subscriptions().update().subscription_id(subscription_id: impl Into&lt;String&gt;).body(body: SubscriptionsUpdateParams) -&gt; Result&lt;Subscription&gt;</code>
- <code title="get /subscriptions">client.subscriptions().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;SubscriptionListResponse&gt;&gt;</code>
- <code title="delete /subscriptions/{subscription_id}/change-plan/scheduled">client.subscriptions().cancel_change_plan().subscription_id(subscription_id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>
- <code title="post /subscriptions/{subscription_id}/change-plan">client.subscriptions().change_plan().subscription_id(subscription_id: impl Into&lt;String&gt;).body(body: SubscriptionsChangePlanParams) -&gt; Result&lt;()&gt;</code>
- <code title="post /subscriptions/{subscription_id}/charge">client.subscriptions().charge().subscription_id(subscription_id: impl Into&lt;String&gt;).body(body: SubscriptionsChargeParams) -&gt; Result&lt;SubscriptionChargeResponse&gt;</code>
- <code title="post /subscriptions/{subscription_id}/change-plan/preview">client.subscriptions().preview_change_plan().subscription_id(subscription_id: impl Into&lt;String&gt;).body(body: SubscriptionsChangePlanParams) -&gt; Result&lt;SubscriptionPreviewChangePlanResponse&gt;</code>
- <code title="get /subscriptions/{subscription_id}/credit-usage">client.subscriptions().retrieve_credit_usage().subscription_id(subscription_id: impl Into&lt;String&gt;) -&gt; Result&lt;SubscriptionRetrieveCreditUsageResponse&gt;</code>
- <code title="get /subscriptions/{subscription_id}/usage-history">client.subscriptions().retrieve_usage_history().subscription_id(subscription_id: impl Into&lt;String&gt;).query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;SubscriptionRetrieveUsageHistoryResponse&gt;&gt;</code>
- <code title="post /subscriptions/{subscription_id}/update-payment-method">client.subscriptions().update_payment_method().subscription_id(subscription_id: impl Into&lt;String&gt;).body(body: serde_json::Value) -&gt; Result&lt;SubscriptionUpdatePaymentMethodResponse&gt;</code>

# Invoices

## Payments

Methods:

- <code title="get /invoices/payments/{payment_id}">client.invoices().payments().retrieve().payment_id(payment_id: impl Into&lt;String&gt;) -&gt; Result&lt;Vec&lt;u8&gt;&gt;</code>
- <code title="get /invoices/payouts/{payout_id}">client.invoices().payments().retrieve_payout().payout_id(payout_id: impl Into&lt;String&gt;) -&gt; Result&lt;Vec&lt;u8&gt;&gt;</code>
- <code title="get /invoices/refunds/{refund_id}">client.invoices().payments().retrieve_refund().refund_id(refund_id: impl Into&lt;String&gt;) -&gt; Result&lt;Vec&lt;u8&gt;&gt;</code>

# Licenses

Methods:

- <code title="post /licenses/activate">client.licenses().activate().body(body: LicensesActivateParams) -&gt; Result&lt;LicenseActivateResponse&gt;</code>
- <code title="post /licenses/deactivate">client.licenses().deactivate().body(body: LicensesDeactivateParams) -&gt; Result&lt;()&gt;</code>
- <code title="post /licenses/validate">client.licenses().validate().body(body: LicensesValidateParams) -&gt; Result&lt;LicenseValidateResponse&gt;</code>

# LicenseKeys

Methods:

- <code title="post /license_keys">client.license_keys().create().body(body: LicenseKeysCreateParams) -&gt; Result&lt;LicenseKey&gt;</code>
- <code title="get /license_keys/{id}">client.license_keys().retrieve().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;LicenseKey&gt;</code>
- <code title="patch /license_keys/{id}">client.license_keys().update().id(id: impl Into&lt;String&gt;).body(body: LicenseKeysUpdateParams) -&gt; Result&lt;LicenseKey&gt;</code>
- <code title="get /license_keys">client.license_keys().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;LicenseKey&gt;&gt;</code>

# LicenseKeyInstances

Methods:

- <code title="get /license_key_instances/{id}">client.license_key_instances().retrieve().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;LicenseKeyInstance&gt;</code>
- <code title="patch /license_key_instances/{id}">client.license_key_instances().update().id(id: impl Into&lt;String&gt;).body(body: LicenseKeyInstancesUpdateParams) -&gt; Result&lt;LicenseKeyInstance&gt;</code>
- <code title="get /license_key_instances">client.license_key_instances().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;LicenseKeyInstance&gt;&gt;</code>

# Customers

Methods:

- <code title="post /customers">client.customers().create().body(body: CustomersCreateParams) -&gt; Result&lt;Customer&gt;</code>
- <code title="get /customers/{customer_id}">client.customers().retrieve().customer_id(customer_id: impl Into&lt;String&gt;) -&gt; Result&lt;Customer&gt;</code>
- <code title="patch /customers/{customer_id}">client.customers().update().customer_id(customer_id: impl Into&lt;String&gt;).body(body: CustomersUpdateParams) -&gt; Result&lt;Customer&gt;</code>
- <code title="get /customers">client.customers().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;Customer&gt;&gt;</code>
- <code title="delete /customers/{customer_id}/payment-methods/{payment_method_id}">client.customers().delete_payment_method().customer_id(customer_id: impl Into&lt;String&gt;).payment_method_id(payment_method_id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>
- <code title="get /customers/{customer_id}/credit-entitlements">client.customers().list_credit_entitlements().customer_id(customer_id: impl Into&lt;String&gt;) -&gt; Result&lt;CustomerListCreditEntitlementsResponse&gt;</code>
- <code title="get /customers/{customer_id}/entitlements">client.customers().list_entitlements().customer_id(customer_id: impl Into&lt;String&gt;) -&gt; Result&lt;CustomerListEntitlementsResponse&gt;</code>
- <code title="get /customers/{customer_id}/payment-methods">client.customers().retrieve_payment_methods().customer_id(customer_id: impl Into&lt;String&gt;) -&gt; Result&lt;CustomerRetrievePaymentMethodsResponse&gt;</code>

## CustomerPortal

Methods:

- <code title="post /customers/{customer_id}/customer-portal/session">client.customers().customer_portal().create().customer_id(customer_id: impl Into&lt;String&gt;).query(query: serde_json::Value) -&gt; Result&lt;CustomerPortalSession&gt;</code>

## Wallets

Methods:

- <code title="get /customers/{customer_id}/wallets">client.customers().wallets().list().customer_id(customer_id: impl Into&lt;String&gt;) -&gt; Result&lt;WalletListResponse&gt;</code>

### LedgerEntries

Methods:

- <code title="post /customers/{customer_id}/wallets/ledger-entries">client.customers().wallets().ledger_entries().create().customer_id(customer_id: impl Into&lt;String&gt;).body(body: CustomersWalletsLedgerEntriesCreateParams) -&gt; Result&lt;CustomerWallet&gt;</code>
- <code title="get /customers/{customer_id}/wallets/ledger-entries">client.customers().wallets().ledger_entries().list().customer_id(customer_id: impl Into&lt;String&gt;).query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;CustomerWalletTransaction&gt;&gt;</code>

# Refunds

Methods:

- <code title="post /refunds">client.refunds().create().body(body: RefundsCreateParams) -&gt; Result&lt;Refund&gt;</code>
- <code title="get /refunds/{refund_id}">client.refunds().retrieve().refund_id(refund_id: impl Into&lt;String&gt;) -&gt; Result&lt;Refund&gt;</code>
- <code title="get /refunds">client.refunds().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;RefundListItem&gt;&gt;</code>

# Disputes

Methods:

- <code title="get /disputes/{dispute_id}">client.disputes().retrieve().dispute_id(dispute_id: impl Into&lt;String&gt;) -&gt; Result&lt;GetDispute&gt;</code>
- <code title="get /disputes">client.disputes().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;DisputeListResponse&gt;&gt;</code>

# Payouts

Methods:

- <code title="get /payouts">client.payouts().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;PayoutListResponse&gt;&gt;</code>

## Breakup

Methods:

- <code title="get /payouts/{payout_id}/breakup">client.payouts().breakup().retrieve().payout_id(payout_id: impl Into&lt;String&gt;) -&gt; Result&lt;BreakupRetrieveResponse&gt;</code>

### Details

Methods:

- <code title="get /payouts/{payout_id}/breakup/details">client.payouts().breakup().details().list().payout_id(payout_id: impl Into&lt;String&gt;).query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;DetailListResponse&gt;&gt;</code>
- <code title="get /payouts/{payout_id}/breakup/details/csv">client.payouts().breakup().details().download_csv().payout_id(payout_id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>

# Products

Methods:

- <code title="post /products">client.products().create().body(body: ProductsCreateParams) -&gt; Result&lt;Product&gt;</code>
- <code title="get /products/{id}">client.products().retrieve().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;Product&gt;</code>
- <code title="patch /products/{id}">client.products().update().id(id: impl Into&lt;String&gt;).body(body: ProductsUpdateParams) -&gt; Result&lt;()&gt;</code>
- <code title="get /products">client.products().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;ProductListResponse&gt;&gt;</code>
- <code title="delete /products/{id}">client.products().archive().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>
- <code title="post /products/{id}/unarchive">client.products().unarchive().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>
- <code title="put /products/{id}/files">client.products().update_files().id(id: impl Into&lt;String&gt;).body(body: ProductsUpdateFilesParams) -&gt; Result&lt;ProductUpdateFilesResponse&gt;</code>

## Images

Methods:

- <code title="put /products/{id}/images">client.products().images().update().id(id: impl Into&lt;String&gt;).query(query: serde_json::Value) -&gt; Result&lt;ImageUpdateResponse&gt;</code>

## ShortLinks

Methods:

- <code title="post /products/{id}/short_links">client.products().short_links().create().id(id: impl Into&lt;String&gt;).body(body: ProductsShortLinksCreateParams) -&gt; Result&lt;ShortLinkCreateResponse&gt;</code>
- <code title="get /products/short_links">client.products().short_links().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;ShortLinkListResponse&gt;&gt;</code>

# Misc

Methods:

- <code title="get /checkout/supported_countries">client.misc().list_supported_countries() -&gt; Result&lt;MiscListSupportedCountriesResponse&gt;</code>

# Discounts

Methods:

- <code title="post /discounts">client.discounts().create().body(body: DiscountsCreateParams) -&gt; Result&lt;Discount&gt;</code>
- <code title="get /discounts/{discount_id}">client.discounts().retrieve().discount_id(discount_id: impl Into&lt;String&gt;) -&gt; Result&lt;Discount&gt;</code>
- <code title="patch /discounts/{discount_id}">client.discounts().update().discount_id(discount_id: impl Into&lt;String&gt;).body(body: DiscountsUpdateParams) -&gt; Result&lt;Discount&gt;</code>
- <code title="get /discounts">client.discounts().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;Discount&gt;&gt;</code>
- <code title="delete /discounts/{discount_id}">client.discounts().delete().discount_id(discount_id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>
- <code title="get /discounts/code/{code}">client.discounts().retrieve_by_code().code(code: impl Into&lt;String&gt;) -&gt; Result&lt;Discount&gt;</code>

# Addons

Methods:

- <code title="post /addons">client.addons().create().body(body: AddonsCreateParams) -&gt; Result&lt;AddonResponse&gt;</code>
- <code title="get /addons/{id}">client.addons().retrieve().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;AddonResponse&gt;</code>
- <code title="patch /addons/{id}">client.addons().update().id(id: impl Into&lt;String&gt;).body(body: AddonsUpdateParams) -&gt; Result&lt;AddonResponse&gt;</code>
- <code title="get /addons">client.addons().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;AddonResponse&gt;&gt;</code>
- <code title="put /addons/{id}/images">client.addons().update_images().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;AddonUpdateImagesResponse&gt;</code>

# Brands

Methods:

- <code title="post /brands">client.brands().create().body(body: BrandsCreateParams) -&gt; Result&lt;Brand&gt;</code>
- <code title="get /brands/{id}">client.brands().retrieve().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;Brand&gt;</code>
- <code title="patch /brands/{id}">client.brands().update().id(id: impl Into&lt;String&gt;).body(body: BrandsUpdateParams) -&gt; Result&lt;Brand&gt;</code>
- <code title="get /brands">client.brands().list() -&gt; Result&lt;BrandListResponse&gt;</code>
- <code title="put /brands/{id}/images">client.brands().update_images().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;BrandUpdateImagesResponse&gt;</code>

# Webhooks

Methods:

- <code title="post /webhooks">client.webhooks().create().body(body: WebhooksCreateParams) -&gt; Result&lt;WebhookDetails&gt;</code>
- <code title="get /webhooks/{webhook_id}">client.webhooks().retrieve().webhook_id(webhook_id: impl Into&lt;String&gt;) -&gt; Result&lt;WebhookDetails&gt;</code>
- <code title="patch /webhooks/{webhook_id}">client.webhooks().update().webhook_id(webhook_id: impl Into&lt;String&gt;).body(body: WebhooksUpdateParams) -&gt; Result&lt;WebhookDetails&gt;</code>
- <code title="get /webhooks">client.webhooks().list().query(query: serde_json::Value) -&gt; Result&lt;CursorPagePagination&lt;WebhookDetails&gt;&gt;</code>
- <code title="delete /webhooks/{webhook_id}">client.webhooks().delete().webhook_id(webhook_id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>
- <code title="get /webhooks/{webhook_id}/secret">client.webhooks().retrieve_secret().webhook_id(webhook_id: impl Into&lt;String&gt;) -&gt; Result&lt;WebhookRetrieveSecretResponse&gt;</code>

## Headers

Methods:

- <code title="get /webhooks/{webhook_id}/headers">client.webhooks().headers().retrieve().webhook_id(webhook_id: impl Into&lt;String&gt;) -&gt; Result&lt;HeaderRetrieveResponse&gt;</code>
- <code title="patch /webhooks/{webhook_id}/headers">client.webhooks().headers().update().webhook_id(webhook_id: impl Into&lt;String&gt;).body(body: WebhooksHeadersUpdateParams) -&gt; Result&lt;()&gt;</code>

# UsageEvents

Methods:

- <code title="get /events/{event_id}">client.usage_events().retrieve().event_id(event_id: impl Into&lt;String&gt;) -&gt; Result&lt;Event&gt;</code>
- <code title="get /events">client.usage_events().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;Event&gt;&gt;</code>
- <code title="post /events/ingest">client.usage_events().ingest().body(body: UsageEventsIngestParams) -&gt; Result&lt;UsageEventIngestResponse&gt;</code>

# Meters

Methods:

- <code title="post /meters">client.meters().create().body(body: MetersCreateParams) -&gt; Result&lt;Meter&gt;</code>
- <code title="get /meters/{id}">client.meters().retrieve().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;Meter&gt;</code>
- <code title="get /meters">client.meters().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;Meter&gt;&gt;</code>
- <code title="delete /meters/{id}">client.meters().archive().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>
- <code title="post /meters/{id}/unarchive">client.meters().unarchive().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>

# Balances

Methods:

- <code title="get /balances/ledger">client.balances().retrieve_ledger().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;BalanceLedgerEntry&gt;&gt;</code>

# CreditEntitlements

Methods:

- <code title="post /credit-entitlements">client.credit_entitlements().create().body(body: CreditEntitlementsCreateParams) -&gt; Result&lt;CreditEntitlement&gt;</code>
- <code title="get /credit-entitlements/{id}">client.credit_entitlements().retrieve().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;CreditEntitlement&gt;</code>
- <code title="patch /credit-entitlements/{id}">client.credit_entitlements().update().id(id: impl Into&lt;String&gt;).body(body: CreditEntitlementsUpdateParams) -&gt; Result&lt;()&gt;</code>
- <code title="get /credit-entitlements">client.credit_entitlements().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;CreditEntitlement&gt;&gt;</code>
- <code title="delete /credit-entitlements/{id}">client.credit_entitlements().delete().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>
- <code title="post /credit-entitlements/{id}/undelete">client.credit_entitlements().undelete().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>

## Balances

Methods:

- <code title="get /credit-entitlements/{credit_entitlement_id}/balances/{customer_id}">client.credit_entitlements().balances().retrieve().credit_entitlement_id(credit_entitlement_id: impl Into&lt;String&gt;).customer_id(customer_id: impl Into&lt;String&gt;) -&gt; Result&lt;CustomerCreditBalance&gt;</code>
- <code title="get /credit-entitlements/{credit_entitlement_id}/balances">client.credit_entitlements().balances().list().credit_entitlement_id(credit_entitlement_id: impl Into&lt;String&gt;).query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;CustomerCreditBalance&gt;&gt;</code>
- <code title="post /credit-entitlements/{credit_entitlement_id}/balances/{customer_id}/ledger-entries">client.credit_entitlements().balances().create_ledger_entry().credit_entitlement_id(credit_entitlement_id: impl Into&lt;String&gt;).customer_id(customer_id: impl Into&lt;String&gt;).body(body: CreditEntitlementsBalancesCreateLedgerEntryParams) -&gt; Result&lt;BalanceCreateLedgerEntryResponse&gt;</code>
- <code title="get /credit-entitlements/{credit_entitlement_id}/balances/{customer_id}/grants">client.credit_entitlements().balances().list_grants().credit_entitlement_id(credit_entitlement_id: impl Into&lt;String&gt;).customer_id(customer_id: impl Into&lt;String&gt;).query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;BalanceListGrantsResponse&gt;&gt;</code>
- <code title="get /credit-entitlements/{credit_entitlement_id}/balances/{customer_id}/ledger">client.credit_entitlements().balances().list_ledger().credit_entitlement_id(credit_entitlement_id: impl Into&lt;String&gt;).customer_id(customer_id: impl Into&lt;String&gt;).query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;CreditLedgerEntry&gt;&gt;</code>

# Entitlements

Methods:

- <code title="post /entitlements">client.entitlements().create().body(body: EntitlementsCreateParams) -&gt; Result&lt;Entitlement&gt;</code>
- <code title="get /entitlements/{id}">client.entitlements().retrieve().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;Entitlement&gt;</code>
- <code title="patch /entitlements/{id}">client.entitlements().update().id(id: impl Into&lt;String&gt;).body(body: EntitlementsUpdateParams) -&gt; Result&lt;Entitlement&gt;</code>
- <code title="get /entitlements">client.entitlements().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;Entitlement&gt;&gt;</code>
- <code title="delete /entitlements/{id}">client.entitlements().delete().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>

## Files

Methods:

- <code title="delete /entitlements/{id}/files/{file_id}">client.entitlements().files().delete().id(id: impl Into&lt;String&gt;).file_id(file_id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>
- <code title="post /entitlements/{id}/files">client.entitlements().files().upload().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;FileUploadResponse&gt;</code>

## Grants

Methods:

- <code title="get /entitlements/{id}/grants">client.entitlements().grants().list().id(id: impl Into&lt;String&gt;).query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;EntitlementGrant&gt;&gt;</code>
- <code title="delete /entitlements/{id}/grants/{grant_id}">client.entitlements().grants().revoke().id(id: impl Into&lt;String&gt;).grant_id(grant_id: impl Into&lt;String&gt;) -&gt; Result&lt;EntitlementGrant&gt;</code>

# ProductCollections

Methods:

- <code title="post /product-collections">client.product_collections().create().body(body: ProductCollectionsCreateParams) -&gt; Result&lt;ProductCollection&gt;</code>
- <code title="get /product-collections/{id}">client.product_collections().retrieve().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;ProductCollection&gt;</code>
- <code title="patch /product-collections/{id}">client.product_collections().update().id(id: impl Into&lt;String&gt;).body(body: ProductCollectionsUpdateParams) -&gt; Result&lt;()&gt;</code>
- <code title="get /product-collections">client.product_collections().list().query(query: serde_json::Value) -&gt; Result&lt;DefaultPageNumberPagination&lt;ProductCollectionListResponse&gt;&gt;</code>
- <code title="delete /product-collections/{id}">client.product_collections().delete().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>
- <code title="post /product-collections/{id}/unarchive">client.product_collections().unarchive().id(id: impl Into&lt;String&gt;) -&gt; Result&lt;ProductCollectionUnarchiveResponse&gt;</code>
- <code title="put /product-collections/{id}/images">client.product_collections().update_images().id(id: impl Into&lt;String&gt;).query(query: serde_json::Value) -&gt; Result&lt;ProductCollectionUpdateImagesResponse&gt;</code>

## Groups

Methods:

- <code title="post /product-collections/{id}/groups">client.product_collections().groups().create().id(id: impl Into&lt;String&gt;).body(body: ProductCollectionsGroupsCreateParams) -&gt; Result&lt;ProductCollectionGroupResponse&gt;</code>
- <code title="patch /product-collections/{id}/groups/{group_id}">client.product_collections().groups().update().id(id: impl Into&lt;String&gt;).group_id(group_id: impl Into&lt;String&gt;).body(body: ProductCollectionsGroupsUpdateParams) -&gt; Result&lt;()&gt;</code>
- <code title="delete /product-collections/{id}/groups/{group_id}">client.product_collections().groups().delete().id(id: impl Into&lt;String&gt;).group_id(group_id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>

### Items

Methods:

- <code title="post /product-collections/{id}/groups/{group_id}/items">client.product_collections().groups().items().create().id(id: impl Into&lt;String&gt;).group_id(group_id: impl Into&lt;String&gt;).body(body: ProductCollectionsGroupsItemsCreateParams) -&gt; Result&lt;ItemCreateResponse&gt;</code>
- <code title="patch /product-collections/{id}/groups/{group_id}/items/{item_id}">client.product_collections().groups().items().update().id(id: impl Into&lt;String&gt;).group_id(group_id: impl Into&lt;String&gt;).item_id(item_id: impl Into&lt;String&gt;).body(body: ProductCollectionsGroupsItemsUpdateParams) -&gt; Result&lt;()&gt;</code>
- <code title="delete /product-collections/{id}/groups/{group_id}/items/{item_id}">client.product_collections().groups().items().delete().id(id: impl Into&lt;String&gt;).group_id(group_id: impl Into&lt;String&gt;).item_id(item_id: impl Into&lt;String&gt;) -&gt; Result&lt;()&gt;</code>
