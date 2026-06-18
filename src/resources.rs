fn build_path(template: &str, params: &[(&str, &str)]) -> String {
    let mut path = template.to_string();
    for &(name, value) in params {
        let placeholder = format!("{{{}}}", name);
        path = path.replace(&placeholder, &encode_path_param(value));
    }
    path
}

fn encode_path_param(value: &str) -> String {
    value
        .split('/')
        .map(encode_path_segment)
        .collect::<Vec<_>>()
        .join("/")
}

fn encode_path_segment(segment: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::new();
    for byte in segment.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push(HEX[(byte >> 4) as usize] as char);
            encoded.push(HEX[(byte & 0x0F) as usize] as char);
        }
    }
    encoded
}

impl crate::Client {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn checkout_sessions(&self) -> CheckoutSessionsResource {
        CheckoutSessionsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn payments(&self) -> PaymentsResource {
        PaymentsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn subscriptions(&self) -> SubscriptionsResource {
        SubscriptionsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn invoices(&self) -> InvoicesResource {
        InvoicesResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn licenses(&self) -> LicensesResource {
        LicensesResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn license_keys(&self) -> LicenseKeysResource {
        LicenseKeysResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn license_key_instances(&self) -> LicenseKeyInstancesResource {
        LicenseKeyInstancesResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn customers(&self) -> CustomersResource {
        CustomersResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn refunds(&self) -> RefundsResource {
        RefundsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn disputes(&self) -> DisputesResource {
        DisputesResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn payouts(&self) -> PayoutsResource {
        PayoutsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn products(&self) -> ProductsResource {
        ProductsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn misc(&self) -> MiscResource {
        MiscResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn discounts(&self) -> DiscountsResource {
        DiscountsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn addons(&self) -> AddonsResource {
        AddonsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn brands(&self) -> BrandsResource {
        BrandsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn webhooks(&self) -> WebhooksResource {
        WebhooksResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn webhook_events(&self) -> WebhookEventsResource {
        WebhookEventsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn usage_events(&self) -> UsageEventsResource {
        UsageEventsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn meters(&self) -> MetersResource {
        MetersResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn balances(&self) -> BalancesResource {
        BalancesResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn credit_entitlements(&self) -> CreditEntitlementsResource {
        CreditEntitlementsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn entitlements(&self) -> EntitlementsResource {
        EntitlementsResource {
            client: self.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn product_collections(&self) -> ProductCollectionsResource {
        ProductCollectionsResource {
            client: self.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CheckoutSessionsResource {
    client: crate::Client,
}

impl CheckoutSessionsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> CheckoutSessionsCreateBuilder {
        CheckoutSessionsCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> CheckoutSessionsRetrieveBuilder {
        CheckoutSessionsRetrieveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn preview(&self) -> CheckoutSessionsPreviewBuilder {
        CheckoutSessionsPreviewBuilder {
            client: self.client.clone(),
            body: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CheckoutSessionsCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::CheckoutSessionsCreateParams>,
}

impl CheckoutSessionsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::CheckoutSessionsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::CheckoutSessionResponse> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "checkout_sessions.create",
        })?;
        let path = "/checkouts".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CheckoutSessionsCreateBuilder {
    type Output = crate::error::Result<crate::models::CheckoutSessionResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::CheckoutSessionResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CheckoutSessionsRetrieveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl CheckoutSessionsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::CheckoutSessionStatus> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "checkout_sessions.retrieve",
            param: "id",
        })?;
        let path = build_path("/checkouts/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CheckoutSessionsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::CheckoutSessionStatus>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::CheckoutSessionStatus>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CheckoutSessionsPreviewBuilder {
    client: crate::Client,
    body: Option<crate::models::CheckoutSessionsCreateParams>,
}

impl CheckoutSessionsPreviewBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::CheckoutSessionsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::CheckoutSessionPreviewResponse> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "checkout_sessions.preview",
        })?;
        let path = "/checkouts/preview".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CheckoutSessionsPreviewBuilder {
    type Output = crate::error::Result<crate::models::CheckoutSessionPreviewResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::CheckoutSessionPreviewResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct PaymentsResource {
    client: crate::Client,
}

impl PaymentsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> PaymentsCreateBuilder {
        PaymentsCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> PaymentsRetrieveBuilder {
        PaymentsRetrieveBuilder {
            client: self.client.clone(),
            payment_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> PaymentsListBuilder {
        PaymentsListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve_line_items(&self) -> PaymentsRetrieveLineItemsBuilder {
        PaymentsRetrieveLineItemsBuilder {
            client: self.client.clone(),
            payment_id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct PaymentsCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::PaymentsCreateParams>,
}

impl PaymentsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::PaymentsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::PaymentCreateResponse> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "payments.create",
        })?;
        let path = "/payments".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for PaymentsCreateBuilder {
    type Output = crate::error::Result<crate::models::PaymentCreateResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::PaymentCreateResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct PaymentsRetrieveBuilder {
    client: crate::Client,
    payment_id: Option<String>,
}

impl PaymentsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn payment_id(mut self, payment_id: impl Into<String>) -> Self {
        self.payment_id = Some(payment_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Payment> {
        let client = self.client;
        let payment_id = self
            .payment_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "payments.retrieve",
                param: "payment_id",
            })?;
        let path = build_path(
            "/payments/{payment_id}",
            &[("payment_id", payment_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for PaymentsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::Payment>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Payment>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct PaymentsListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl PaymentsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::PaymentListResponse>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/payments".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::PaymentListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for PaymentsListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::PaymentListResponse>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::PaymentListResponse,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct PaymentsRetrieveLineItemsBuilder {
    client: crate::Client,
    payment_id: Option<String>,
}

impl PaymentsRetrieveLineItemsBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn payment_id(mut self, payment_id: impl Into<String>) -> Self {
        self.payment_id = Some(payment_id.into());
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::PaymentRetrieveLineItemsResponse> {
        let client = self.client;
        let payment_id = self
            .payment_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "payments.retrieve_line_items",
                param: "payment_id",
            })?;
        let path = build_path(
            "/payments/{payment_id}/line-items",
            &[("payment_id", payment_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for PaymentsRetrieveLineItemsBuilder {
    type Output = crate::error::Result<crate::models::PaymentRetrieveLineItemsResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::PaymentRetrieveLineItemsResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct SubscriptionsResource {
    client: crate::Client,
}

impl SubscriptionsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> SubscriptionsCreateBuilder {
        SubscriptionsCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> SubscriptionsRetrieveBuilder {
        SubscriptionsRetrieveBuilder {
            client: self.client.clone(),
            subscription_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> SubscriptionsUpdateBuilder {
        SubscriptionsUpdateBuilder {
            client: self.client.clone(),
            subscription_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> SubscriptionsListBuilder {
        SubscriptionsListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn cancel_change_plan(&self) -> SubscriptionsCancelChangePlanBuilder {
        SubscriptionsCancelChangePlanBuilder {
            client: self.client.clone(),
            subscription_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn change_plan(&self) -> SubscriptionsChangePlanBuilder {
        SubscriptionsChangePlanBuilder {
            client: self.client.clone(),
            subscription_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn charge(&self) -> SubscriptionsChargeBuilder {
        SubscriptionsChargeBuilder {
            client: self.client.clone(),
            subscription_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn preview_change_plan(&self) -> SubscriptionsPreviewChangePlanBuilder {
        SubscriptionsPreviewChangePlanBuilder {
            client: self.client.clone(),
            subscription_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve_credit_usage(&self) -> SubscriptionsRetrieveCreditUsageBuilder {
        SubscriptionsRetrieveCreditUsageBuilder {
            client: self.client.clone(),
            subscription_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve_usage_history(&self) -> SubscriptionsRetrieveUsageHistoryBuilder {
        SubscriptionsRetrieveUsageHistoryBuilder {
            client: self.client.clone(),
            subscription_id: None,
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update_payment_method(&self) -> SubscriptionsUpdatePaymentMethodBuilder {
        SubscriptionsUpdatePaymentMethodBuilder {
            client: self.client.clone(),
            subscription_id: None,
            body: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct SubscriptionsCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::SubscriptionsCreateParams>,
}

impl SubscriptionsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::SubscriptionsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::SubscriptionCreateResponse> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "subscriptions.create",
        })?;
        let path = "/subscriptions".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for SubscriptionsCreateBuilder {
    type Output = crate::error::Result<crate::models::SubscriptionCreateResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::SubscriptionCreateResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct SubscriptionsRetrieveBuilder {
    client: crate::Client,
    subscription_id: Option<String>,
}

impl SubscriptionsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Subscription> {
        let client = self.client;
        let subscription_id =
            self.subscription_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "subscriptions.retrieve",
                    param: "subscription_id",
                })?;
        let path = build_path(
            "/subscriptions/{subscription_id}",
            &[("subscription_id", subscription_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for SubscriptionsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::Subscription>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Subscription>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct SubscriptionsUpdateBuilder {
    client: crate::Client,
    subscription_id: Option<String>,
    body: Option<crate::models::SubscriptionsUpdateParams>,
}

impl SubscriptionsUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::SubscriptionsUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Subscription> {
        let client = self.client;
        let subscription_id =
            self.subscription_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "subscriptions.update",
                    param: "subscription_id",
                })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "subscriptions.update",
        })?;
        let path = build_path(
            "/subscriptions/{subscription_id}",
            &[("subscription_id", subscription_id.as_str())],
        );
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for SubscriptionsUpdateBuilder {
    type Output = crate::error::Result<crate::models::Subscription>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Subscription>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct SubscriptionsListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl SubscriptionsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::SubscriptionListResponse>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/subscriptions".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::SubscriptionListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for SubscriptionsListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::SubscriptionListResponse>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::SubscriptionListResponse,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct SubscriptionsCancelChangePlanBuilder {
    client: crate::Client,
    subscription_id: Option<String>,
}

impl SubscriptionsCancelChangePlanBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let subscription_id =
            self.subscription_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "subscriptions.cancel_change_plan",
                    param: "subscription_id",
                })?;
        let path = build_path(
            "/subscriptions/{subscription_id}/change-plan/scheduled",
            &[("subscription_id", subscription_id.as_str())],
        );
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for SubscriptionsCancelChangePlanBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct SubscriptionsChangePlanBuilder {
    client: crate::Client,
    subscription_id: Option<String>,
    body: Option<crate::models::SubscriptionsChangePlanParams>,
}

impl SubscriptionsChangePlanBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::SubscriptionsChangePlanParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let subscription_id =
            self.subscription_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "subscriptions.change_plan",
                    param: "subscription_id",
                })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "subscriptions.change_plan",
        })?;
        let path = build_path(
            "/subscriptions/{subscription_id}/change-plan",
            &[("subscription_id", subscription_id.as_str())],
        );
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for SubscriptionsChangePlanBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct SubscriptionsChargeBuilder {
    client: crate::Client,
    subscription_id: Option<String>,
    body: Option<crate::models::SubscriptionsChargeParams>,
}

impl SubscriptionsChargeBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::SubscriptionsChargeParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::SubscriptionChargeResponse> {
        let client = self.client;
        let subscription_id =
            self.subscription_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "subscriptions.charge",
                    param: "subscription_id",
                })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "subscriptions.charge",
        })?;
        let path = build_path(
            "/subscriptions/{subscription_id}/charge",
            &[("subscription_id", subscription_id.as_str())],
        );
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for SubscriptionsChargeBuilder {
    type Output = crate::error::Result<crate::models::SubscriptionChargeResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::SubscriptionChargeResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct SubscriptionsPreviewChangePlanBuilder {
    client: crate::Client,
    subscription_id: Option<String>,
    body: Option<crate::models::SubscriptionsChangePlanParams>,
}

impl SubscriptionsPreviewChangePlanBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::SubscriptionsChangePlanParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::SubscriptionPreviewChangePlanResponse> {
        let client = self.client;
        let subscription_id =
            self.subscription_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "subscriptions.preview_change_plan",
                    param: "subscription_id",
                })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "subscriptions.preview_change_plan",
        })?;
        let path = build_path(
            "/subscriptions/{subscription_id}/change-plan/preview",
            &[("subscription_id", subscription_id.as_str())],
        );
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for SubscriptionsPreviewChangePlanBuilder {
    type Output = crate::error::Result<crate::models::SubscriptionPreviewChangePlanResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::SubscriptionPreviewChangePlanResponse,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct SubscriptionsRetrieveCreditUsageBuilder {
    client: crate::Client,
    subscription_id: Option<String>,
}

impl SubscriptionsRetrieveCreditUsageBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::SubscriptionRetrieveCreditUsageResponse> {
        let client = self.client;
        let subscription_id =
            self.subscription_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "subscriptions.retrieve_credit_usage",
                    param: "subscription_id",
                })?;
        let path = build_path(
            "/subscriptions/{subscription_id}/credit-usage",
            &[("subscription_id", subscription_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for SubscriptionsRetrieveCreditUsageBuilder {
    type Output = crate::error::Result<crate::models::SubscriptionRetrieveCreditUsageResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::SubscriptionRetrieveCreditUsageResponse,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct SubscriptionsRetrieveUsageHistoryBuilder {
    client: crate::Client,
    subscription_id: Option<String>,
    query: Option<serde_json::Value>,
}

impl SubscriptionsRetrieveUsageHistoryBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<
            crate::models::SubscriptionRetrieveUsageHistoryResponse,
        >,
    > {
        let client = self.client;
        let subscription_id =
            self.subscription_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "subscriptions.retrieve_usage_history",
                    param: "subscription_id",
                })?;
        let query = self.query;
        let path = build_path(
            "/subscriptions/{subscription_id}/usage-history",
            &[("subscription_id", subscription_id.as_str())],
        );
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client
            .handle_response::<crate::models::DefaultPageNumberPagination<
                crate::models::SubscriptionRetrieveUsageHistoryResponse,
            >>(request)
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for SubscriptionsRetrieveUsageHistoryBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<
            crate::models::SubscriptionRetrieveUsageHistoryResponse,
        >,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::SubscriptionRetrieveUsageHistoryResponse,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct SubscriptionsUpdatePaymentMethodBuilder {
    client: crate::Client,
    subscription_id: Option<String>,
    body: Option<serde_json::Value>,
}

impl SubscriptionsUpdatePaymentMethodBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.subscription_id = Some(subscription_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::SubscriptionUpdatePaymentMethodResponse> {
        let client = self.client;
        let subscription_id =
            self.subscription_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "subscriptions.update_payment_method",
                    param: "subscription_id",
                })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "subscriptions.update_payment_method",
        })?;
        let path = build_path(
            "/subscriptions/{subscription_id}/update-payment-method",
            &[("subscription_id", subscription_id.as_str())],
        );
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for SubscriptionsUpdatePaymentMethodBuilder {
    type Output = crate::error::Result<crate::models::SubscriptionUpdatePaymentMethodResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::SubscriptionUpdatePaymentMethodResponse,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct InvoicesResource {
    client: crate::Client,
}

impl InvoicesResource {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn payments(&self) -> InvoicesPaymentsResource {
        InvoicesPaymentsResource {
            client: self.client.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InvoicesPaymentsResource {
    client: crate::Client,
}

impl InvoicesPaymentsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> InvoicesPaymentsRetrieveBuilder {
        InvoicesPaymentsRetrieveBuilder {
            client: self.client.clone(),
            payment_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve_payout(&self) -> InvoicesPaymentsRetrievePayoutBuilder {
        InvoicesPaymentsRetrievePayoutBuilder {
            client: self.client.clone(),
            payout_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve_refund(&self) -> InvoicesPaymentsRetrieveRefundBuilder {
        InvoicesPaymentsRetrieveRefundBuilder {
            client: self.client.clone(),
            refund_id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct InvoicesPaymentsRetrieveBuilder {
    client: crate::Client,
    payment_id: Option<String>,
}

impl InvoicesPaymentsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn payment_id(mut self, payment_id: impl Into<String>) -> Self {
        self.payment_id = Some(payment_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<Vec<u8>> {
        let client = self.client;
        let payment_id = self
            .payment_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "invoices.payments.retrieve",
                param: "payment_id",
            })?;
        let path = build_path(
            "/invoices/payments/{payment_id}",
            &[("payment_id", payment_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_bytes(request).await
    }
}

impl std::future::IntoFuture for InvoicesPaymentsRetrieveBuilder {
    type Output = crate::error::Result<Vec<u8>>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<Vec<u8>>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct InvoicesPaymentsRetrievePayoutBuilder {
    client: crate::Client,
    payout_id: Option<String>,
}

impl InvoicesPaymentsRetrievePayoutBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn payout_id(mut self, payout_id: impl Into<String>) -> Self {
        self.payout_id = Some(payout_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<Vec<u8>> {
        let client = self.client;
        let payout_id = self
            .payout_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "invoices.payments.retrieve_payout",
                param: "payout_id",
            })?;
        let path = build_path(
            "/invoices/payouts/{payout_id}",
            &[("payout_id", payout_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_bytes(request).await
    }
}

impl std::future::IntoFuture for InvoicesPaymentsRetrievePayoutBuilder {
    type Output = crate::error::Result<Vec<u8>>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<Vec<u8>>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct InvoicesPaymentsRetrieveRefundBuilder {
    client: crate::Client,
    refund_id: Option<String>,
}

impl InvoicesPaymentsRetrieveRefundBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn refund_id(mut self, refund_id: impl Into<String>) -> Self {
        self.refund_id = Some(refund_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<Vec<u8>> {
        let client = self.client;
        let refund_id = self
            .refund_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "invoices.payments.retrieve_refund",
                param: "refund_id",
            })?;
        let path = build_path(
            "/invoices/refunds/{refund_id}",
            &[("refund_id", refund_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_bytes(request).await
    }
}

impl std::future::IntoFuture for InvoicesPaymentsRetrieveRefundBuilder {
    type Output = crate::error::Result<Vec<u8>>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<Vec<u8>>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct LicensesResource {
    client: crate::Client,
}

impl LicensesResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn activate(&self) -> LicensesActivateBuilder {
        LicensesActivateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn deactivate(&self) -> LicensesDeactivateBuilder {
        LicensesDeactivateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn validate(&self) -> LicensesValidateBuilder {
        LicensesValidateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct LicensesActivateBuilder {
    client: crate::Client,
    body: Option<crate::models::LicensesActivateParams>,
}

impl LicensesActivateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::LicensesActivateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::LicenseActivateResponse> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "licenses.activate",
        })?;
        let path = "/licenses/activate".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for LicensesActivateBuilder {
    type Output = crate::error::Result<crate::models::LicenseActivateResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::LicenseActivateResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct LicensesDeactivateBuilder {
    client: crate::Client,
    body: Option<crate::models::LicensesDeactivateParams>,
}

impl LicensesDeactivateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::LicensesDeactivateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "licenses.deactivate",
        })?;
        let path = "/licenses/deactivate".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for LicensesDeactivateBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct LicensesValidateBuilder {
    client: crate::Client,
    body: Option<crate::models::LicensesValidateParams>,
}

impl LicensesValidateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::LicensesValidateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::LicenseValidateResponse> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "licenses.validate",
        })?;
        let path = "/licenses/validate".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for LicensesValidateBuilder {
    type Output = crate::error::Result<crate::models::LicenseValidateResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::LicenseValidateResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct LicenseKeysResource {
    client: crate::Client,
}

impl LicenseKeysResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> LicenseKeysCreateBuilder {
        LicenseKeysCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> LicenseKeysRetrieveBuilder {
        LicenseKeysRetrieveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> LicenseKeysUpdateBuilder {
        LicenseKeysUpdateBuilder {
            client: self.client.clone(),
            id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> LicenseKeysListBuilder {
        LicenseKeysListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct LicenseKeysCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::LicenseKeysCreateParams>,
}

impl LicenseKeysCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::LicenseKeysCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::LicenseKey> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "license_keys.create",
        })?;
        let path = "/license_keys".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for LicenseKeysCreateBuilder {
    type Output = crate::error::Result<crate::models::LicenseKey>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::LicenseKey>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct LicenseKeysRetrieveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl LicenseKeysRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::LicenseKey> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "license_keys.retrieve",
            param: "id",
        })?;
        let path = build_path("/license_keys/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for LicenseKeysRetrieveBuilder {
    type Output = crate::error::Result<crate::models::LicenseKey>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::LicenseKey>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct LicenseKeysUpdateBuilder {
    client: crate::Client,
    id: Option<String>,
    body: Option<crate::models::LicenseKeysUpdateParams>,
}

impl LicenseKeysUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::LicenseKeysUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::LicenseKey> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "license_keys.update",
            param: "id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "license_keys.update",
        })?;
        let path = build_path("/license_keys/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for LicenseKeysUpdateBuilder {
    type Output = crate::error::Result<crate::models::LicenseKey>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::LicenseKey>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct LicenseKeysListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl LicenseKeysListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::LicenseKey>>
    {
        let client = self.client;
        let query = self.query;
        let path = "/license_keys".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::LicenseKey>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for LicenseKeysListBuilder {
    type Output =
        crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::LicenseKey>>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<crate::models::LicenseKey>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct LicenseKeyInstancesResource {
    client: crate::Client,
}

impl LicenseKeyInstancesResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> LicenseKeyInstancesRetrieveBuilder {
        LicenseKeyInstancesRetrieveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> LicenseKeyInstancesUpdateBuilder {
        LicenseKeyInstancesUpdateBuilder {
            client: self.client.clone(),
            id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> LicenseKeyInstancesListBuilder {
        LicenseKeyInstancesListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct LicenseKeyInstancesRetrieveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl LicenseKeyInstancesRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::LicenseKeyInstance> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "license_key_instances.retrieve",
            param: "id",
        })?;
        let path = build_path("/license_key_instances/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for LicenseKeyInstancesRetrieveBuilder {
    type Output = crate::error::Result<crate::models::LicenseKeyInstance>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::LicenseKeyInstance>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct LicenseKeyInstancesUpdateBuilder {
    client: crate::Client,
    id: Option<String>,
    body: Option<crate::models::LicenseKeyInstancesUpdateParams>,
}

impl LicenseKeyInstancesUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::LicenseKeyInstancesUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::LicenseKeyInstance> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "license_key_instances.update",
            param: "id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "license_key_instances.update",
        })?;
        let path = build_path("/license_key_instances/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for LicenseKeyInstancesUpdateBuilder {
    type Output = crate::error::Result<crate::models::LicenseKeyInstance>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::LicenseKeyInstance>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct LicenseKeyInstancesListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl LicenseKeyInstancesListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::LicenseKeyInstance>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/license_key_instances".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::LicenseKeyInstance>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for LicenseKeyInstancesListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::LicenseKeyInstance>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::LicenseKeyInstance,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct CustomersResource {
    client: crate::Client,
}

impl CustomersResource {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn customer_portal(&self) -> CustomersCustomerPortalResource {
        CustomersCustomerPortalResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn wallets(&self) -> CustomersWalletsResource {
        CustomersWalletsResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> CustomersCreateBuilder {
        CustomersCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> CustomersRetrieveBuilder {
        CustomersRetrieveBuilder {
            client: self.client.clone(),
            customer_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> CustomersUpdateBuilder {
        CustomersUpdateBuilder {
            client: self.client.clone(),
            customer_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> CustomersListBuilder {
        CustomersListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn delete_payment_method(&self) -> CustomersDeletePaymentMethodBuilder {
        CustomersDeletePaymentMethodBuilder {
            client: self.client.clone(),
            customer_id: None,
            payment_method_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list_credit_entitlements(&self) -> CustomersListCreditEntitlementsBuilder {
        CustomersListCreditEntitlementsBuilder {
            client: self.client.clone(),
            customer_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list_entitlements(&self) -> CustomersListEntitlementsBuilder {
        CustomersListEntitlementsBuilder {
            client: self.client.clone(),
            customer_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve_payment_methods(&self) -> CustomersRetrievePaymentMethodsBuilder {
        CustomersRetrievePaymentMethodsBuilder {
            client: self.client.clone(),
            customer_id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::CustomersCreateParams>,
}

impl CustomersCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::CustomersCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Customer> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "customers.create",
        })?;
        let path = "/customers".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CustomersCreateBuilder {
    type Output = crate::error::Result<crate::models::Customer>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Customer>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersRetrieveBuilder {
    client: crate::Client,
    customer_id: Option<String>,
}

impl CustomersRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Customer> {
        let client = self.client;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "customers.retrieve",
                param: "customer_id",
            })?;
        let path = build_path(
            "/customers/{customer_id}",
            &[("customer_id", customer_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CustomersRetrieveBuilder {
    type Output = crate::error::Result<crate::models::Customer>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Customer>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersUpdateBuilder {
    client: crate::Client,
    customer_id: Option<String>,
    body: Option<crate::models::CustomersUpdateParams>,
}

impl CustomersUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::CustomersUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Customer> {
        let client = self.client;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "customers.update",
                param: "customer_id",
            })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "customers.update",
        })?;
        let path = build_path(
            "/customers/{customer_id}",
            &[("customer_id", customer_id.as_str())],
        );
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CustomersUpdateBuilder {
    type Output = crate::error::Result<crate::models::Customer>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Customer>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl CustomersListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Customer>>
    {
        let client = self.client;
        let query = self.query;
        let path = "/customers".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client
            .handle_response::<crate::models::DefaultPageNumberPagination<crate::models::Customer>>(
                request,
            )
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for CustomersListBuilder {
    type Output =
        crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Customer>>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<crate::models::Customer>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersDeletePaymentMethodBuilder {
    client: crate::Client,
    customer_id: Option<String>,
    payment_method_id: Option<String>,
}

impl CustomersDeletePaymentMethodBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn payment_method_id(mut self, payment_method_id: impl Into<String>) -> Self {
        self.payment_method_id = Some(payment_method_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "customers.delete_payment_method",
                param: "customer_id",
            })?;
        let payment_method_id =
            self.payment_method_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "customers.delete_payment_method",
                    param: "payment_method_id",
                })?;
        let path = build_path(
            "/customers/{customer_id}/payment-methods/{payment_method_id}",
            &[
                ("customer_id", customer_id.as_str()),
                ("payment_method_id", payment_method_id.as_str()),
            ],
        );
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for CustomersDeletePaymentMethodBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersListCreditEntitlementsBuilder {
    client: crate::Client,
    customer_id: Option<String>,
}

impl CustomersListCreditEntitlementsBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::CustomerListCreditEntitlementsResponse> {
        let client = self.client;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "customers.list_credit_entitlements",
                param: "customer_id",
            })?;
        let path = build_path(
            "/customers/{customer_id}/credit-entitlements",
            &[("customer_id", customer_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CustomersListCreditEntitlementsBuilder {
    type Output = crate::error::Result<crate::models::CustomerListCreditEntitlementsResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::CustomerListCreditEntitlementsResponse,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersListEntitlementsBuilder {
    client: crate::Client,
    customer_id: Option<String>,
}

impl CustomersListEntitlementsBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::CustomerListEntitlementsResponse> {
        let client = self.client;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "customers.list_entitlements",
                param: "customer_id",
            })?;
        let path = build_path(
            "/customers/{customer_id}/entitlements",
            &[("customer_id", customer_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CustomersListEntitlementsBuilder {
    type Output = crate::error::Result<crate::models::CustomerListEntitlementsResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::CustomerListEntitlementsResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersRetrievePaymentMethodsBuilder {
    client: crate::Client,
    customer_id: Option<String>,
}

impl CustomersRetrievePaymentMethodsBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::CustomerRetrievePaymentMethodsResponse> {
        let client = self.client;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "customers.retrieve_payment_methods",
                param: "customer_id",
            })?;
        let path = build_path(
            "/customers/{customer_id}/payment-methods",
            &[("customer_id", customer_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CustomersRetrievePaymentMethodsBuilder {
    type Output = crate::error::Result<crate::models::CustomerRetrievePaymentMethodsResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::CustomerRetrievePaymentMethodsResponse,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct CustomersCustomerPortalResource {
    client: crate::Client,
}

impl CustomersCustomerPortalResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> CustomersCustomerPortalCreateBuilder {
        CustomersCustomerPortalCreateBuilder {
            client: self.client.clone(),
            customer_id: None,
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersCustomerPortalCreateBuilder {
    client: crate::Client,
    customer_id: Option<String>,
    query: Option<serde_json::Value>,
}

impl CustomersCustomerPortalCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::CustomerPortalSession> {
        let client = self.client;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "customers.customer_portal.create",
                param: "customer_id",
            })?;
        let query = self.query;
        let path = build_path(
            "/customers/{customer_id}/customer-portal/session",
            &[("customer_id", customer_id.as_str())],
        );
        let mut request = client.request(reqwest::Method::POST, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CustomersCustomerPortalCreateBuilder {
    type Output = crate::error::Result<crate::models::CustomerPortalSession>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::CustomerPortalSession>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct CustomersWalletsResource {
    client: crate::Client,
}

impl CustomersWalletsResource {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn ledger_entries(&self) -> CustomersWalletsLedgerEntriesResource {
        CustomersWalletsLedgerEntriesResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> CustomersWalletsListBuilder {
        CustomersWalletsListBuilder {
            client: self.client.clone(),
            customer_id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersWalletsListBuilder {
    client: crate::Client,
    customer_id: Option<String>,
}

impl CustomersWalletsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::WalletListResponse> {
        let client = self.client;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "customers.wallets.list",
                param: "customer_id",
            })?;
        let path = build_path(
            "/customers/{customer_id}/wallets",
            &[("customer_id", customer_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CustomersWalletsListBuilder {
    type Output = crate::error::Result<crate::models::WalletListResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::WalletListResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct CustomersWalletsLedgerEntriesResource {
    client: crate::Client,
}

impl CustomersWalletsLedgerEntriesResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> CustomersWalletsLedgerEntriesCreateBuilder {
        CustomersWalletsLedgerEntriesCreateBuilder {
            client: self.client.clone(),
            customer_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> CustomersWalletsLedgerEntriesListBuilder {
        CustomersWalletsLedgerEntriesListBuilder {
            client: self.client.clone(),
            customer_id: None,
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersWalletsLedgerEntriesCreateBuilder {
    client: crate::Client,
    customer_id: Option<String>,
    body: Option<crate::models::CustomersWalletsLedgerEntriesCreateParams>,
}

impl CustomersWalletsLedgerEntriesCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::CustomersWalletsLedgerEntriesCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::CustomerWallet> {
        let client = self.client;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "customers.wallets.ledger_entries.create",
                param: "customer_id",
            })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "customers.wallets.ledger_entries.create",
        })?;
        let path = build_path(
            "/customers/{customer_id}/wallets/ledger-entries",
            &[("customer_id", customer_id.as_str())],
        );
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CustomersWalletsLedgerEntriesCreateBuilder {
    type Output = crate::error::Result<crate::models::CustomerWallet>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::CustomerWallet>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CustomersWalletsLedgerEntriesListBuilder {
    client: crate::Client,
    customer_id: Option<String>,
    query: Option<serde_json::Value>,
}

impl CustomersWalletsLedgerEntriesListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CustomerWalletTransaction>,
    > {
        let client = self.client;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "customers.wallets.ledger_entries.list",
                param: "customer_id",
            })?;
        let query = self.query;
        let path = build_path(
            "/customers/{customer_id}/wallets/ledger-entries",
            &[("customer_id", customer_id.as_str())],
        );
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::CustomerWalletTransaction>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for CustomersWalletsLedgerEntriesListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CustomerWalletTransaction>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::CustomerWalletTransaction,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct RefundsResource {
    client: crate::Client,
}

impl RefundsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> RefundsCreateBuilder {
        RefundsCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> RefundsRetrieveBuilder {
        RefundsRetrieveBuilder {
            client: self.client.clone(),
            refund_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> RefundsListBuilder {
        RefundsListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct RefundsCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::RefundsCreateParams>,
}

impl RefundsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::RefundsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Refund> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "refunds.create",
        })?;
        let path = "/refunds".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for RefundsCreateBuilder {
    type Output = crate::error::Result<crate::models::Refund>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Refund>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct RefundsRetrieveBuilder {
    client: crate::Client,
    refund_id: Option<String>,
}

impl RefundsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn refund_id(mut self, refund_id: impl Into<String>) -> Self {
        self.refund_id = Some(refund_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Refund> {
        let client = self.client;
        let refund_id = self
            .refund_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "refunds.retrieve",
                param: "refund_id",
            })?;
        let path = build_path("/refunds/{refund_id}", &[("refund_id", refund_id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for RefundsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::Refund>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Refund>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct RefundsListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl RefundsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::RefundListItem>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/refunds".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::RefundListItem>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for RefundsListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::RefundListItem>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<crate::models::RefundListItem>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct DisputesResource {
    client: crate::Client,
}

impl DisputesResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> DisputesRetrieveBuilder {
        DisputesRetrieveBuilder {
            client: self.client.clone(),
            dispute_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> DisputesListBuilder {
        DisputesListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct DisputesRetrieveBuilder {
    client: crate::Client,
    dispute_id: Option<String>,
}

impl DisputesRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn dispute_id(mut self, dispute_id: impl Into<String>) -> Self {
        self.dispute_id = Some(dispute_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::GetDispute> {
        let client = self.client;
        let dispute_id = self
            .dispute_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "disputes.retrieve",
                param: "dispute_id",
            })?;
        let path = build_path(
            "/disputes/{dispute_id}",
            &[("dispute_id", dispute_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for DisputesRetrieveBuilder {
    type Output = crate::error::Result<crate::models::GetDispute>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::GetDispute>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct DisputesListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl DisputesListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::DisputeListResponse>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/disputes".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::DisputeListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for DisputesListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::DisputeListResponse>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::DisputeListResponse,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct PayoutsResource {
    client: crate::Client,
}

impl PayoutsResource {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn breakup(&self) -> PayoutsBreakupResource {
        PayoutsBreakupResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> PayoutsListBuilder {
        PayoutsListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct PayoutsListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl PayoutsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::PayoutListResponse>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/payouts".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::PayoutListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for PayoutsListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::PayoutListResponse>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::PayoutListResponse,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct PayoutsBreakupResource {
    client: crate::Client,
}

impl PayoutsBreakupResource {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn details(&self) -> PayoutsBreakupDetailsResource {
        PayoutsBreakupDetailsResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> PayoutsBreakupRetrieveBuilder {
        PayoutsBreakupRetrieveBuilder {
            client: self.client.clone(),
            payout_id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct PayoutsBreakupRetrieveBuilder {
    client: crate::Client,
    payout_id: Option<String>,
}

impl PayoutsBreakupRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn payout_id(mut self, payout_id: impl Into<String>) -> Self {
        self.payout_id = Some(payout_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::BreakupRetrieveResponse> {
        let client = self.client;
        let payout_id = self
            .payout_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "payouts.breakup.retrieve",
                param: "payout_id",
            })?;
        let path = build_path(
            "/payouts/{payout_id}/breakup",
            &[("payout_id", payout_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for PayoutsBreakupRetrieveBuilder {
    type Output = crate::error::Result<crate::models::BreakupRetrieveResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::BreakupRetrieveResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct PayoutsBreakupDetailsResource {
    client: crate::Client,
}

impl PayoutsBreakupDetailsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> PayoutsBreakupDetailsListBuilder {
        PayoutsBreakupDetailsListBuilder {
            client: self.client.clone(),
            payout_id: None,
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn download_csv(&self) -> PayoutsBreakupDetailsDownloadCsvBuilder {
        PayoutsBreakupDetailsDownloadCsvBuilder {
            client: self.client.clone(),
            payout_id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct PayoutsBreakupDetailsListBuilder {
    client: crate::Client,
    payout_id: Option<String>,
    query: Option<serde_json::Value>,
}

impl PayoutsBreakupDetailsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn payout_id(mut self, payout_id: impl Into<String>) -> Self {
        self.payout_id = Some(payout_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::DetailListResponse>,
    > {
        let client = self.client;
        let payout_id = self
            .payout_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "payouts.breakup.details.list",
                param: "payout_id",
            })?;
        let query = self.query;
        let path = build_path(
            "/payouts/{payout_id}/breakup/details",
            &[("payout_id", payout_id.as_str())],
        );
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::DetailListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for PayoutsBreakupDetailsListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::DetailListResponse>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::DetailListResponse,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct PayoutsBreakupDetailsDownloadCsvBuilder {
    client: crate::Client,
    payout_id: Option<String>,
}

impl PayoutsBreakupDetailsDownloadCsvBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn payout_id(mut self, payout_id: impl Into<String>) -> Self {
        self.payout_id = Some(payout_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let payout_id = self
            .payout_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "payouts.breakup.details.download_csv",
                param: "payout_id",
            })?;
        let path = build_path(
            "/payouts/{payout_id}/breakup/details/csv",
            &[("payout_id", payout_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for PayoutsBreakupDetailsDownloadCsvBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct ProductsResource {
    client: crate::Client,
}

impl ProductsResource {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn images(&self) -> ProductsImagesResource {
        ProductsImagesResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn short_links(&self) -> ProductsShortLinksResource {
        ProductsShortLinksResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> ProductsCreateBuilder {
        ProductsCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> ProductsRetrieveBuilder {
        ProductsRetrieveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> ProductsUpdateBuilder {
        ProductsUpdateBuilder {
            client: self.client.clone(),
            id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> ProductsListBuilder {
        ProductsListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn archive(&self) -> ProductsArchiveBuilder {
        ProductsArchiveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn unarchive(&self) -> ProductsUnarchiveBuilder {
        ProductsUnarchiveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update_files(&self) -> ProductsUpdateFilesBuilder {
        ProductsUpdateFilesBuilder {
            client: self.client.clone(),
            id: None,
            body: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductsCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::ProductsCreateParams>,
}

impl ProductsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::ProductsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Product> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "products.create",
        })?;
        let path = "/products".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for ProductsCreateBuilder {
    type Output = crate::error::Result<crate::models::Product>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Product>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductsRetrieveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl ProductsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Product> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "products.retrieve",
            param: "id",
        })?;
        let path = build_path("/products/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for ProductsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::Product>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Product>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductsUpdateBuilder {
    client: crate::Client,
    id: Option<String>,
    body: Option<crate::models::ProductsUpdateParams>,
}

impl ProductsUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::ProductsUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "products.update",
            param: "id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "products.update",
        })?;
        let path = build_path("/products/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for ProductsUpdateBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductsListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl ProductsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::ProductListResponse>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/products".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::ProductListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for ProductsListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::ProductListResponse>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::ProductListResponse,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductsArchiveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl ProductsArchiveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "products.archive",
            param: "id",
        })?;
        let path = build_path("/products/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for ProductsArchiveBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductsUnarchiveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl ProductsUnarchiveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "products.unarchive",
            param: "id",
        })?;
        let path = build_path("/products/{id}/unarchive", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::POST, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for ProductsUnarchiveBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductsUpdateFilesBuilder {
    client: crate::Client,
    id: Option<String>,
    body: Option<crate::models::ProductsUpdateFilesParams>,
}

impl ProductsUpdateFilesBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::ProductsUpdateFilesParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::ProductUpdateFilesResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "products.update_files",
            param: "id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "products.update_files",
        })?;
        let path = build_path("/products/{id}/files", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::PUT, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for ProductsUpdateFilesBuilder {
    type Output = crate::error::Result<crate::models::ProductUpdateFilesResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::ProductUpdateFilesResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct ProductsImagesResource {
    client: crate::Client,
}

impl ProductsImagesResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> ProductsImagesUpdateBuilder {
        ProductsImagesUpdateBuilder {
            client: self.client.clone(),
            id: None,
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductsImagesUpdateBuilder {
    client: crate::Client,
    id: Option<String>,
    query: Option<serde_json::Value>,
}

impl ProductsImagesUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::ImageUpdateResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "products.images.update",
            param: "id",
        })?;
        let query = self.query;
        let path = build_path("/products/{id}/images", &[("id", id.as_str())]);
        let mut request = client.request(reqwest::Method::PUT, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for ProductsImagesUpdateBuilder {
    type Output = crate::error::Result<crate::models::ImageUpdateResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::ImageUpdateResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct ProductsShortLinksResource {
    client: crate::Client,
}

impl ProductsShortLinksResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> ProductsShortLinksCreateBuilder {
        ProductsShortLinksCreateBuilder {
            client: self.client.clone(),
            id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> ProductsShortLinksListBuilder {
        ProductsShortLinksListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductsShortLinksCreateBuilder {
    client: crate::Client,
    id: Option<String>,
    body: Option<crate::models::ProductsShortLinksCreateParams>,
}

impl ProductsShortLinksCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::ProductsShortLinksCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::ShortLinkCreateResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "products.short_links.create",
            param: "id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "products.short_links.create",
        })?;
        let path = build_path("/products/{id}/short_links", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for ProductsShortLinksCreateBuilder {
    type Output = crate::error::Result<crate::models::ShortLinkCreateResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::ShortLinkCreateResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductsShortLinksListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl ProductsShortLinksListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::ShortLinkListResponse>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/products/short_links".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::ShortLinkListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for ProductsShortLinksListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::ShortLinkListResponse>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::ShortLinkListResponse,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct MiscResource {
    client: crate::Client,
}

impl MiscResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list_supported_countries(&self) -> MiscListSupportedCountriesBuilder {
        MiscListSupportedCountriesBuilder {
            client: self.client.clone(),
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct MiscListSupportedCountriesBuilder {
    client: crate::Client,
}

impl MiscListSupportedCountriesBuilder {
    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::MiscListSupportedCountriesResponse> {
        let client = self.client;
        let path = "/checkout/supported_countries".to_string();
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for MiscListSupportedCountriesBuilder {
    type Output = crate::error::Result<crate::models::MiscListSupportedCountriesResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::MiscListSupportedCountriesResponse,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct DiscountsResource {
    client: crate::Client,
}

impl DiscountsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> DiscountsCreateBuilder {
        DiscountsCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> DiscountsRetrieveBuilder {
        DiscountsRetrieveBuilder {
            client: self.client.clone(),
            discount_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> DiscountsUpdateBuilder {
        DiscountsUpdateBuilder {
            client: self.client.clone(),
            discount_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> DiscountsListBuilder {
        DiscountsListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn delete(&self) -> DiscountsDeleteBuilder {
        DiscountsDeleteBuilder {
            client: self.client.clone(),
            discount_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve_by_code(&self) -> DiscountsRetrieveByCodeBuilder {
        DiscountsRetrieveByCodeBuilder {
            client: self.client.clone(),
            code: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct DiscountsCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::DiscountsCreateParams>,
}

impl DiscountsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::DiscountsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Discount> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "discounts.create",
        })?;
        let path = "/discounts".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for DiscountsCreateBuilder {
    type Output = crate::error::Result<crate::models::Discount>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Discount>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct DiscountsRetrieveBuilder {
    client: crate::Client,
    discount_id: Option<String>,
}

impl DiscountsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn discount_id(mut self, discount_id: impl Into<String>) -> Self {
        self.discount_id = Some(discount_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Discount> {
        let client = self.client;
        let discount_id = self
            .discount_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "discounts.retrieve",
                param: "discount_id",
            })?;
        let path = build_path(
            "/discounts/{discount_id}",
            &[("discount_id", discount_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for DiscountsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::Discount>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Discount>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct DiscountsUpdateBuilder {
    client: crate::Client,
    discount_id: Option<String>,
    body: Option<crate::models::DiscountsUpdateParams>,
}

impl DiscountsUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn discount_id(mut self, discount_id: impl Into<String>) -> Self {
        self.discount_id = Some(discount_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::DiscountsUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Discount> {
        let client = self.client;
        let discount_id = self
            .discount_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "discounts.update",
                param: "discount_id",
            })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "discounts.update",
        })?;
        let path = build_path(
            "/discounts/{discount_id}",
            &[("discount_id", discount_id.as_str())],
        );
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for DiscountsUpdateBuilder {
    type Output = crate::error::Result<crate::models::Discount>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Discount>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct DiscountsListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl DiscountsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Discount>>
    {
        let client = self.client;
        let query = self.query;
        let path = "/discounts".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client
            .handle_response::<crate::models::DefaultPageNumberPagination<crate::models::Discount>>(
                request,
            )
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for DiscountsListBuilder {
    type Output =
        crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Discount>>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<crate::models::Discount>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct DiscountsDeleteBuilder {
    client: crate::Client,
    discount_id: Option<String>,
}

impl DiscountsDeleteBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn discount_id(mut self, discount_id: impl Into<String>) -> Self {
        self.discount_id = Some(discount_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let discount_id = self
            .discount_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "discounts.delete",
                param: "discount_id",
            })?;
        let path = build_path(
            "/discounts/{discount_id}",
            &[("discount_id", discount_id.as_str())],
        );
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for DiscountsDeleteBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct DiscountsRetrieveByCodeBuilder {
    client: crate::Client,
    code: Option<String>,
}

impl DiscountsRetrieveByCodeBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Discount> {
        let client = self.client;
        let code = self.code.ok_or(crate::error::Error::MissingPathParam {
            operation: "discounts.retrieve_by_code",
            param: "code",
        })?;
        let path = build_path("/discounts/code/{code}", &[("code", code.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for DiscountsRetrieveByCodeBuilder {
    type Output = crate::error::Result<crate::models::Discount>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Discount>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct AddonsResource {
    client: crate::Client,
}

impl AddonsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> AddonsCreateBuilder {
        AddonsCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> AddonsRetrieveBuilder {
        AddonsRetrieveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> AddonsUpdateBuilder {
        AddonsUpdateBuilder {
            client: self.client.clone(),
            id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> AddonsListBuilder {
        AddonsListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update_images(&self) -> AddonsUpdateImagesBuilder {
        AddonsUpdateImagesBuilder {
            client: self.client.clone(),
            id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct AddonsCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::AddonsCreateParams>,
}

impl AddonsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::AddonsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::AddonResponse> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "addons.create",
        })?;
        let path = "/addons".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for AddonsCreateBuilder {
    type Output = crate::error::Result<crate::models::AddonResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::AddonResponse>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct AddonsRetrieveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl AddonsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::AddonResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "addons.retrieve",
            param: "id",
        })?;
        let path = build_path("/addons/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for AddonsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::AddonResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::AddonResponse>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct AddonsUpdateBuilder {
    client: crate::Client,
    id: Option<String>,
    body: Option<crate::models::AddonsUpdateParams>,
}

impl AddonsUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::AddonsUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::AddonResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "addons.update",
            param: "id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "addons.update",
        })?;
        let path = build_path("/addons/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for AddonsUpdateBuilder {
    type Output = crate::error::Result<crate::models::AddonResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::AddonResponse>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct AddonsListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl AddonsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::AddonResponse>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/addons".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::AddonResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for AddonsListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::AddonResponse>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<crate::models::AddonResponse>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct AddonsUpdateImagesBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl AddonsUpdateImagesBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::AddonUpdateImagesResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "addons.update_images",
            param: "id",
        })?;
        let path = build_path("/addons/{id}/images", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::PUT, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for AddonsUpdateImagesBuilder {
    type Output = crate::error::Result<crate::models::AddonUpdateImagesResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::AddonUpdateImagesResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct BrandsResource {
    client: crate::Client,
}

impl BrandsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> BrandsCreateBuilder {
        BrandsCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> BrandsRetrieveBuilder {
        BrandsRetrieveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> BrandsUpdateBuilder {
        BrandsUpdateBuilder {
            client: self.client.clone(),
            id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> BrandsListBuilder {
        BrandsListBuilder {
            client: self.client.clone(),
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update_images(&self) -> BrandsUpdateImagesBuilder {
        BrandsUpdateImagesBuilder {
            client: self.client.clone(),
            id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct BrandsCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::BrandsCreateParams>,
}

impl BrandsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::BrandsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Brand> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "brands.create",
        })?;
        let path = "/brands".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for BrandsCreateBuilder {
    type Output = crate::error::Result<crate::models::Brand>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Brand>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct BrandsRetrieveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl BrandsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Brand> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "brands.retrieve",
            param: "id",
        })?;
        let path = build_path("/brands/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for BrandsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::Brand>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Brand>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct BrandsUpdateBuilder {
    client: crate::Client,
    id: Option<String>,
    body: Option<crate::models::BrandsUpdateParams>,
}

impl BrandsUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::BrandsUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Brand> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "brands.update",
            param: "id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "brands.update",
        })?;
        let path = build_path("/brands/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for BrandsUpdateBuilder {
    type Output = crate::error::Result<crate::models::Brand>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Brand>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct BrandsListBuilder {
    client: crate::Client,
}

impl BrandsListBuilder {
    pub async fn send(self) -> crate::error::Result<crate::models::BrandListResponse> {
        let client = self.client;
        let path = "/brands".to_string();
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for BrandsListBuilder {
    type Output = crate::error::Result<crate::models::BrandListResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::BrandListResponse>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct BrandsUpdateImagesBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl BrandsUpdateImagesBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::BrandUpdateImagesResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "brands.update_images",
            param: "id",
        })?;
        let path = build_path("/brands/{id}/images", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::PUT, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for BrandsUpdateImagesBuilder {
    type Output = crate::error::Result<crate::models::BrandUpdateImagesResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::BrandUpdateImagesResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct WebhooksResource {
    client: crate::Client,
}

impl WebhooksResource {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn headers(&self) -> WebhooksHeadersResource {
        WebhooksHeadersResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> WebhooksCreateBuilder {
        WebhooksCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> WebhooksRetrieveBuilder {
        WebhooksRetrieveBuilder {
            client: self.client.clone(),
            webhook_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> WebhooksUpdateBuilder {
        WebhooksUpdateBuilder {
            client: self.client.clone(),
            webhook_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> WebhooksListBuilder {
        WebhooksListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn delete(&self) -> WebhooksDeleteBuilder {
        WebhooksDeleteBuilder {
            client: self.client.clone(),
            webhook_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve_secret(&self) -> WebhooksRetrieveSecretBuilder {
        WebhooksRetrieveSecretBuilder {
            client: self.client.clone(),
            webhook_id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct WebhooksCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::WebhooksCreateParams>,
}

impl WebhooksCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::WebhooksCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::WebhookDetails> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "webhooks.create",
        })?;
        let path = "/webhooks".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for WebhooksCreateBuilder {
    type Output = crate::error::Result<crate::models::WebhookDetails>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::WebhookDetails>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct WebhooksRetrieveBuilder {
    client: crate::Client,
    webhook_id: Option<String>,
}

impl WebhooksRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn webhook_id(mut self, webhook_id: impl Into<String>) -> Self {
        self.webhook_id = Some(webhook_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::WebhookDetails> {
        let client = self.client;
        let webhook_id = self
            .webhook_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "webhooks.retrieve",
                param: "webhook_id",
            })?;
        let path = build_path(
            "/webhooks/{webhook_id}",
            &[("webhook_id", webhook_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for WebhooksRetrieveBuilder {
    type Output = crate::error::Result<crate::models::WebhookDetails>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::WebhookDetails>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct WebhooksUpdateBuilder {
    client: crate::Client,
    webhook_id: Option<String>,
    body: Option<crate::models::WebhooksUpdateParams>,
}

impl WebhooksUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn webhook_id(mut self, webhook_id: impl Into<String>) -> Self {
        self.webhook_id = Some(webhook_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::WebhooksUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::WebhookDetails> {
        let client = self.client;
        let webhook_id = self
            .webhook_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "webhooks.update",
                param: "webhook_id",
            })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "webhooks.update",
        })?;
        let path = build_path(
            "/webhooks/{webhook_id}",
            &[("webhook_id", webhook_id.as_str())],
        );
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for WebhooksUpdateBuilder {
    type Output = crate::error::Result<crate::models::WebhookDetails>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::WebhookDetails>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct WebhooksListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl WebhooksListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::CursorPagePagination<crate::models::WebhookDetails>>
    {
        let client = self.client;
        let query = self.query;
        let path = "/webhooks".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client
            .handle_response::<crate::models::CursorPagePagination<crate::models::WebhookDetails>>(
                request,
            )
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for WebhooksListBuilder {
    type Output =
        crate::error::Result<crate::models::CursorPagePagination<crate::models::WebhookDetails>>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::CursorPagePagination<crate::models::WebhookDetails>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct WebhooksDeleteBuilder {
    client: crate::Client,
    webhook_id: Option<String>,
}

impl WebhooksDeleteBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn webhook_id(mut self, webhook_id: impl Into<String>) -> Self {
        self.webhook_id = Some(webhook_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let webhook_id = self
            .webhook_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "webhooks.delete",
                param: "webhook_id",
            })?;
        let path = build_path(
            "/webhooks/{webhook_id}",
            &[("webhook_id", webhook_id.as_str())],
        );
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for WebhooksDeleteBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct WebhooksRetrieveSecretBuilder {
    client: crate::Client,
    webhook_id: Option<String>,
}

impl WebhooksRetrieveSecretBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn webhook_id(mut self, webhook_id: impl Into<String>) -> Self {
        self.webhook_id = Some(webhook_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::WebhookRetrieveSecretResponse> {
        let client = self.client;
        let webhook_id = self
            .webhook_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "webhooks.retrieve_secret",
                param: "webhook_id",
            })?;
        let path = build_path(
            "/webhooks/{webhook_id}/secret",
            &[("webhook_id", webhook_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for WebhooksRetrieveSecretBuilder {
    type Output = crate::error::Result<crate::models::WebhookRetrieveSecretResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::WebhookRetrieveSecretResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct WebhooksHeadersResource {
    client: crate::Client,
}

impl WebhooksHeadersResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> WebhooksHeadersRetrieveBuilder {
        WebhooksHeadersRetrieveBuilder {
            client: self.client.clone(),
            webhook_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> WebhooksHeadersUpdateBuilder {
        WebhooksHeadersUpdateBuilder {
            client: self.client.clone(),
            webhook_id: None,
            body: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct WebhooksHeadersRetrieveBuilder {
    client: crate::Client,
    webhook_id: Option<String>,
}

impl WebhooksHeadersRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn webhook_id(mut self, webhook_id: impl Into<String>) -> Self {
        self.webhook_id = Some(webhook_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::HeaderRetrieveResponse> {
        let client = self.client;
        let webhook_id = self
            .webhook_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "webhooks.headers.retrieve",
                param: "webhook_id",
            })?;
        let path = build_path(
            "/webhooks/{webhook_id}/headers",
            &[("webhook_id", webhook_id.as_str())],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for WebhooksHeadersRetrieveBuilder {
    type Output = crate::error::Result<crate::models::HeaderRetrieveResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::HeaderRetrieveResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct WebhooksHeadersUpdateBuilder {
    client: crate::Client,
    webhook_id: Option<String>,
    body: Option<crate::models::WebhooksHeadersUpdateParams>,
}

impl WebhooksHeadersUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn webhook_id(mut self, webhook_id: impl Into<String>) -> Self {
        self.webhook_id = Some(webhook_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::WebhooksHeadersUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let webhook_id = self
            .webhook_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "webhooks.headers.update",
                param: "webhook_id",
            })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "webhooks.headers.update",
        })?;
        let path = build_path(
            "/webhooks/{webhook_id}/headers",
            &[("webhook_id", webhook_id.as_str())],
        );
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for WebhooksHeadersUpdateBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct WebhookEventsResource {
    client: crate::Client,
}

impl WebhookEventsResource {}

#[derive(Clone, Debug)]
pub struct UsageEventsResource {
    client: crate::Client,
}

impl UsageEventsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> UsageEventsRetrieveBuilder {
        UsageEventsRetrieveBuilder {
            client: self.client.clone(),
            event_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> UsageEventsListBuilder {
        UsageEventsListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn ingest(&self) -> UsageEventsIngestBuilder {
        UsageEventsIngestBuilder {
            client: self.client.clone(),
            body: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct UsageEventsRetrieveBuilder {
    client: crate::Client,
    event_id: Option<String>,
}

impl UsageEventsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn event_id(mut self, event_id: impl Into<String>) -> Self {
        self.event_id = Some(event_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Event> {
        let client = self.client;
        let event_id = self.event_id.ok_or(crate::error::Error::MissingPathParam {
            operation: "usage_events.retrieve",
            param: "event_id",
        })?;
        let path = build_path("/events/{event_id}", &[("event_id", event_id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for UsageEventsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::Event>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Event>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct UsageEventsListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl UsageEventsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Event>>
    {
        let client = self.client;
        let query = self.query;
        let path = "/events".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client
            .handle_response::<crate::models::DefaultPageNumberPagination<crate::models::Event>>(
                request,
            )
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for UsageEventsListBuilder {
    type Output =
        crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Event>>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<crate::models::Event>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct UsageEventsIngestBuilder {
    client: crate::Client,
    body: Option<crate::models::UsageEventsIngestParams>,
}

impl UsageEventsIngestBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::UsageEventsIngestParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::UsageEventIngestResponse> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "usage_events.ingest",
        })?;
        let path = "/events/ingest".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for UsageEventsIngestBuilder {
    type Output = crate::error::Result<crate::models::UsageEventIngestResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::UsageEventIngestResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct MetersResource {
    client: crate::Client,
}

impl MetersResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> MetersCreateBuilder {
        MetersCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> MetersRetrieveBuilder {
        MetersRetrieveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> MetersListBuilder {
        MetersListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn archive(&self) -> MetersArchiveBuilder {
        MetersArchiveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn unarchive(&self) -> MetersUnarchiveBuilder {
        MetersUnarchiveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct MetersCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::MetersCreateParams>,
}

impl MetersCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::MetersCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Meter> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "meters.create",
        })?;
        let path = "/meters".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for MetersCreateBuilder {
    type Output = crate::error::Result<crate::models::Meter>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Meter>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct MetersRetrieveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl MetersRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Meter> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "meters.retrieve",
            param: "id",
        })?;
        let path = build_path("/meters/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for MetersRetrieveBuilder {
    type Output = crate::error::Result<crate::models::Meter>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Meter>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct MetersListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl MetersListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Meter>>
    {
        let client = self.client;
        let query = self.query;
        let path = "/meters".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client
            .handle_response::<crate::models::DefaultPageNumberPagination<crate::models::Meter>>(
                request,
            )
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for MetersListBuilder {
    type Output =
        crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Meter>>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<crate::models::Meter>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct MetersArchiveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl MetersArchiveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "meters.archive",
            param: "id",
        })?;
        let path = build_path("/meters/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for MetersArchiveBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct MetersUnarchiveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl MetersUnarchiveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "meters.unarchive",
            param: "id",
        })?;
        let path = build_path("/meters/{id}/unarchive", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::POST, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for MetersUnarchiveBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct BalancesResource {
    client: crate::Client,
}

impl BalancesResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve_ledger(&self) -> BalancesRetrieveLedgerBuilder {
        BalancesRetrieveLedgerBuilder {
            client: self.client.clone(),
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct BalancesRetrieveLedgerBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl BalancesRetrieveLedgerBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::BalanceLedgerEntry>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/balances/ledger".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::BalanceLedgerEntry>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for BalancesRetrieveLedgerBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::BalanceLedgerEntry>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::BalanceLedgerEntry,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct CreditEntitlementsResource {
    client: crate::Client,
}

impl CreditEntitlementsResource {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn balances(&self) -> CreditEntitlementsBalancesResource {
        CreditEntitlementsBalancesResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> CreditEntitlementsCreateBuilder {
        CreditEntitlementsCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> CreditEntitlementsRetrieveBuilder {
        CreditEntitlementsRetrieveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> CreditEntitlementsUpdateBuilder {
        CreditEntitlementsUpdateBuilder {
            client: self.client.clone(),
            id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> CreditEntitlementsListBuilder {
        CreditEntitlementsListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn delete(&self) -> CreditEntitlementsDeleteBuilder {
        CreditEntitlementsDeleteBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn undelete(&self) -> CreditEntitlementsUndeleteBuilder {
        CreditEntitlementsUndeleteBuilder {
            client: self.client.clone(),
            id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CreditEntitlementsCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::CreditEntitlementsCreateParams>,
}

impl CreditEntitlementsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::CreditEntitlementsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::CreditEntitlement> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "credit_entitlements.create",
        })?;
        let path = "/credit-entitlements".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CreditEntitlementsCreateBuilder {
    type Output = crate::error::Result<crate::models::CreditEntitlement>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::CreditEntitlement>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CreditEntitlementsRetrieveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl CreditEntitlementsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::CreditEntitlement> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "credit_entitlements.retrieve",
            param: "id",
        })?;
        let path = build_path("/credit-entitlements/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CreditEntitlementsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::CreditEntitlement>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::CreditEntitlement>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CreditEntitlementsUpdateBuilder {
    client: crate::Client,
    id: Option<String>,
    body: Option<crate::models::CreditEntitlementsUpdateParams>,
}

impl CreditEntitlementsUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::CreditEntitlementsUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "credit_entitlements.update",
            param: "id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "credit_entitlements.update",
        })?;
        let path = build_path("/credit-entitlements/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for CreditEntitlementsUpdateBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CreditEntitlementsListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl CreditEntitlementsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CreditEntitlement>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/credit-entitlements".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::CreditEntitlement>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for CreditEntitlementsListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CreditEntitlement>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::CreditEntitlement,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CreditEntitlementsDeleteBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl CreditEntitlementsDeleteBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "credit_entitlements.delete",
            param: "id",
        })?;
        let path = build_path("/credit-entitlements/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for CreditEntitlementsDeleteBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CreditEntitlementsUndeleteBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl CreditEntitlementsUndeleteBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "credit_entitlements.undelete",
            param: "id",
        })?;
        let path = build_path("/credit-entitlements/{id}/undelete", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::POST, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for CreditEntitlementsUndeleteBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct CreditEntitlementsBalancesResource {
    client: crate::Client,
}

impl CreditEntitlementsBalancesResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> CreditEntitlementsBalancesRetrieveBuilder {
        CreditEntitlementsBalancesRetrieveBuilder {
            client: self.client.clone(),
            credit_entitlement_id: None,
            customer_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> CreditEntitlementsBalancesListBuilder {
        CreditEntitlementsBalancesListBuilder {
            client: self.client.clone(),
            credit_entitlement_id: None,
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create_ledger_entry(&self) -> CreditEntitlementsBalancesCreateLedgerEntryBuilder {
        CreditEntitlementsBalancesCreateLedgerEntryBuilder {
            client: self.client.clone(),
            credit_entitlement_id: None,
            customer_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list_grants(&self) -> CreditEntitlementsBalancesListGrantsBuilder {
        CreditEntitlementsBalancesListGrantsBuilder {
            client: self.client.clone(),
            credit_entitlement_id: None,
            customer_id: None,
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list_ledger(&self) -> CreditEntitlementsBalancesListLedgerBuilder {
        CreditEntitlementsBalancesListLedgerBuilder {
            client: self.client.clone(),
            credit_entitlement_id: None,
            customer_id: None,
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CreditEntitlementsBalancesRetrieveBuilder {
    client: crate::Client,
    credit_entitlement_id: Option<String>,
    customer_id: Option<String>,
}

impl CreditEntitlementsBalancesRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn credit_entitlement_id(mut self, credit_entitlement_id: impl Into<String>) -> Self {
        self.credit_entitlement_id = Some(credit_entitlement_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::CustomerCreditBalance> {
        let client = self.client;
        let credit_entitlement_id =
            self.credit_entitlement_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "credit_entitlements.balances.retrieve",
                    param: "credit_entitlement_id",
                })?;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "credit_entitlements.balances.retrieve",
                param: "customer_id",
            })?;
        let path = build_path(
            "/credit-entitlements/{credit_entitlement_id}/balances/{customer_id}",
            &[
                ("credit_entitlement_id", credit_entitlement_id.as_str()),
                ("customer_id", customer_id.as_str()),
            ],
        );
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CreditEntitlementsBalancesRetrieveBuilder {
    type Output = crate::error::Result<crate::models::CustomerCreditBalance>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::CustomerCreditBalance>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CreditEntitlementsBalancesListBuilder {
    client: crate::Client,
    credit_entitlement_id: Option<String>,
    query: Option<serde_json::Value>,
}

impl CreditEntitlementsBalancesListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn credit_entitlement_id(mut self, credit_entitlement_id: impl Into<String>) -> Self {
        self.credit_entitlement_id = Some(credit_entitlement_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CustomerCreditBalance>,
    > {
        let client = self.client;
        let credit_entitlement_id =
            self.credit_entitlement_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "credit_entitlements.balances.list",
                    param: "credit_entitlement_id",
                })?;
        let query = self.query;
        let path = build_path(
            "/credit-entitlements/{credit_entitlement_id}/balances",
            &[("credit_entitlement_id", credit_entitlement_id.as_str())],
        );
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::CustomerCreditBalance>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for CreditEntitlementsBalancesListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CustomerCreditBalance>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::CustomerCreditBalance,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CreditEntitlementsBalancesCreateLedgerEntryBuilder {
    client: crate::Client,
    credit_entitlement_id: Option<String>,
    customer_id: Option<String>,
    body: Option<crate::models::CreditEntitlementsBalancesCreateLedgerEntryParams>,
}

impl CreditEntitlementsBalancesCreateLedgerEntryBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn credit_entitlement_id(mut self, credit_entitlement_id: impl Into<String>) -> Self {
        self.credit_entitlement_id = Some(credit_entitlement_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(
        mut self,
        body: crate::models::CreditEntitlementsBalancesCreateLedgerEntryParams,
    ) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::BalanceCreateLedgerEntryResponse> {
        let client = self.client;
        let credit_entitlement_id =
            self.credit_entitlement_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "credit_entitlements.balances.create_ledger_entry",
                    param: "credit_entitlement_id",
                })?;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "credit_entitlements.balances.create_ledger_entry",
                param: "customer_id",
            })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "credit_entitlements.balances.create_ledger_entry",
        })?;
        let path = build_path(
            "/credit-entitlements/{credit_entitlement_id}/balances/{customer_id}/ledger-entries",
            &[
                ("credit_entitlement_id", credit_entitlement_id.as_str()),
                ("customer_id", customer_id.as_str()),
            ],
        );
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for CreditEntitlementsBalancesCreateLedgerEntryBuilder {
    type Output = crate::error::Result<crate::models::BalanceCreateLedgerEntryResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::BalanceCreateLedgerEntryResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CreditEntitlementsBalancesListGrantsBuilder {
    client: crate::Client,
    credit_entitlement_id: Option<String>,
    customer_id: Option<String>,
    query: Option<serde_json::Value>,
}

impl CreditEntitlementsBalancesListGrantsBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn credit_entitlement_id(mut self, credit_entitlement_id: impl Into<String>) -> Self {
        self.credit_entitlement_id = Some(credit_entitlement_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::BalanceListGrantsResponse>,
    > {
        let client = self.client;
        let credit_entitlement_id =
            self.credit_entitlement_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "credit_entitlements.balances.list_grants",
                    param: "credit_entitlement_id",
                })?;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "credit_entitlements.balances.list_grants",
                param: "customer_id",
            })?;
        let query = self.query;
        let path = build_path(
            "/credit-entitlements/{credit_entitlement_id}/balances/{customer_id}/grants",
            &[
                ("credit_entitlement_id", credit_entitlement_id.as_str()),
                ("customer_id", customer_id.as_str()),
            ],
        );
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::BalanceListGrantsResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for CreditEntitlementsBalancesListGrantsBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::BalanceListGrantsResponse>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::BalanceListGrantsResponse,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct CreditEntitlementsBalancesListLedgerBuilder {
    client: crate::Client,
    credit_entitlement_id: Option<String>,
    customer_id: Option<String>,
    query: Option<serde_json::Value>,
}

impl CreditEntitlementsBalancesListLedgerBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn credit_entitlement_id(mut self, credit_entitlement_id: impl Into<String>) -> Self {
        self.credit_entitlement_id = Some(credit_entitlement_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn customer_id(mut self, customer_id: impl Into<String>) -> Self {
        self.customer_id = Some(customer_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CreditLedgerEntry>,
    > {
        let client = self.client;
        let credit_entitlement_id =
            self.credit_entitlement_id
                .ok_or(crate::error::Error::MissingPathParam {
                    operation: "credit_entitlements.balances.list_ledger",
                    param: "credit_entitlement_id",
                })?;
        let customer_id = self
            .customer_id
            .ok_or(crate::error::Error::MissingPathParam {
                operation: "credit_entitlements.balances.list_ledger",
                param: "customer_id",
            })?;
        let query = self.query;
        let path = build_path(
            "/credit-entitlements/{credit_entitlement_id}/balances/{customer_id}/ledger",
            &[
                ("credit_entitlement_id", credit_entitlement_id.as_str()),
                ("customer_id", customer_id.as_str()),
            ],
        );
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::CreditLedgerEntry>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for CreditEntitlementsBalancesListLedgerBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CreditLedgerEntry>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::CreditLedgerEntry,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct EntitlementsResource {
    client: crate::Client,
}

impl EntitlementsResource {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn files(&self) -> EntitlementsFilesResource {
        EntitlementsFilesResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn grants(&self) -> EntitlementsGrantsResource {
        EntitlementsGrantsResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> EntitlementsCreateBuilder {
        EntitlementsCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> EntitlementsRetrieveBuilder {
        EntitlementsRetrieveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> EntitlementsUpdateBuilder {
        EntitlementsUpdateBuilder {
            client: self.client.clone(),
            id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> EntitlementsListBuilder {
        EntitlementsListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn delete(&self) -> EntitlementsDeleteBuilder {
        EntitlementsDeleteBuilder {
            client: self.client.clone(),
            id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct EntitlementsCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::EntitlementsCreateParams>,
}

impl EntitlementsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::EntitlementsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Entitlement> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "entitlements.create",
        })?;
        let path = "/entitlements".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for EntitlementsCreateBuilder {
    type Output = crate::error::Result<crate::models::Entitlement>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Entitlement>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct EntitlementsRetrieveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl EntitlementsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Entitlement> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "entitlements.retrieve",
            param: "id",
        })?;
        let path = build_path("/entitlements/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for EntitlementsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::Entitlement>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Entitlement>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct EntitlementsUpdateBuilder {
    client: crate::Client,
    id: Option<String>,
    body: Option<crate::models::EntitlementsUpdateParams>,
}

impl EntitlementsUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::EntitlementsUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::Entitlement> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "entitlements.update",
            param: "id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "entitlements.update",
        })?;
        let path = build_path("/entitlements/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for EntitlementsUpdateBuilder {
    type Output = crate::error::Result<crate::models::Entitlement>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::Entitlement>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct EntitlementsListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl EntitlementsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Entitlement>>
    {
        let client = self.client;
        let query = self.query;
        let path = "/entitlements".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::Entitlement>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for EntitlementsListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::Entitlement>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<crate::models::Entitlement>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct EntitlementsDeleteBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl EntitlementsDeleteBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "entitlements.delete",
            param: "id",
        })?;
        let path = build_path("/entitlements/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for EntitlementsDeleteBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct EntitlementsFilesResource {
    client: crate::Client,
}

impl EntitlementsFilesResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn delete(&self) -> EntitlementsFilesDeleteBuilder {
        EntitlementsFilesDeleteBuilder {
            client: self.client.clone(),
            id: None,
            file_id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn upload(&self) -> EntitlementsFilesUploadBuilder {
        EntitlementsFilesUploadBuilder {
            client: self.client.clone(),
            id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct EntitlementsFilesDeleteBuilder {
    client: crate::Client,
    id: Option<String>,
    file_id: Option<String>,
}

impl EntitlementsFilesDeleteBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn file_id(mut self, file_id: impl Into<String>) -> Self {
        self.file_id = Some(file_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "entitlements.files.delete",
            param: "id",
        })?;
        let file_id = self.file_id.ok_or(crate::error::Error::MissingPathParam {
            operation: "entitlements.files.delete",
            param: "file_id",
        })?;
        let path = build_path(
            "/entitlements/{id}/files/{file_id}",
            &[("id", id.as_str()), ("file_id", file_id.as_str())],
        );
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for EntitlementsFilesDeleteBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct EntitlementsFilesUploadBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl EntitlementsFilesUploadBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::FileUploadResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "entitlements.files.upload",
            param: "id",
        })?;
        let path = build_path("/entitlements/{id}/files", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::POST, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for EntitlementsFilesUploadBuilder {
    type Output = crate::error::Result<crate::models::FileUploadResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::FileUploadResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct EntitlementsGrantsResource {
    client: crate::Client,
}

impl EntitlementsGrantsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> EntitlementsGrantsListBuilder {
        EntitlementsGrantsListBuilder {
            client: self.client.clone(),
            id: None,
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn revoke(&self) -> EntitlementsGrantsRevokeBuilder {
        EntitlementsGrantsRevokeBuilder {
            client: self.client.clone(),
            id: None,
            grant_id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct EntitlementsGrantsListBuilder {
    client: crate::Client,
    id: Option<String>,
    query: Option<serde_json::Value>,
}

impl EntitlementsGrantsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::EntitlementGrant>,
    > {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "entitlements.grants.list",
            param: "id",
        })?;
        let query = self.query;
        let path = build_path("/entitlements/{id}/grants", &[("id", id.as_str())]);
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page = client.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::EntitlementGrant>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for EntitlementsGrantsListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::EntitlementGrant>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<crate::models::EntitlementGrant>,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct EntitlementsGrantsRevokeBuilder {
    client: crate::Client,
    id: Option<String>,
    grant_id: Option<String>,
}

impl EntitlementsGrantsRevokeBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn grant_id(mut self, grant_id: impl Into<String>) -> Self {
        self.grant_id = Some(grant_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::EntitlementGrant> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "entitlements.grants.revoke",
            param: "id",
        })?;
        let grant_id = self.grant_id.ok_or(crate::error::Error::MissingPathParam {
            operation: "entitlements.grants.revoke",
            param: "grant_id",
        })?;
        let path = build_path(
            "/entitlements/{id}/grants/{grant_id}",
            &[("id", id.as_str()), ("grant_id", grant_id.as_str())],
        );
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for EntitlementsGrantsRevokeBuilder {
    type Output = crate::error::Result<crate::models::EntitlementGrant>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::EntitlementGrant>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct ProductCollectionsResource {
    client: crate::Client,
}

impl ProductCollectionsResource {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn groups(&self) -> ProductCollectionsGroupsResource {
        ProductCollectionsGroupsResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> ProductCollectionsCreateBuilder {
        ProductCollectionsCreateBuilder {
            client: self.client.clone(),
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn retrieve(&self) -> ProductCollectionsRetrieveBuilder {
        ProductCollectionsRetrieveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> ProductCollectionsUpdateBuilder {
        ProductCollectionsUpdateBuilder {
            client: self.client.clone(),
            id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn list(&self) -> ProductCollectionsListBuilder {
        ProductCollectionsListBuilder {
            client: self.client.clone(),
            query: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn delete(&self) -> ProductCollectionsDeleteBuilder {
        ProductCollectionsDeleteBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn unarchive(&self) -> ProductCollectionsUnarchiveBuilder {
        ProductCollectionsUnarchiveBuilder {
            client: self.client.clone(),
            id: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update_images(&self) -> ProductCollectionsUpdateImagesBuilder {
        ProductCollectionsUpdateImagesBuilder {
            client: self.client.clone(),
            id: None,
            query: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsCreateBuilder {
    client: crate::Client,
    body: Option<crate::models::ProductCollectionsCreateParams>,
}

impl ProductCollectionsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::ProductCollectionsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::ProductCollection> {
        let client = self.client;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "product_collections.create",
        })?;
        let path = "/product-collections".to_string();
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsCreateBuilder {
    type Output = crate::error::Result<crate::models::ProductCollection>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::ProductCollection>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsRetrieveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl ProductCollectionsRetrieveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::ProductCollection> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.retrieve",
            param: "id",
        })?;
        let path = build_path("/product-collections/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::GET, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsRetrieveBuilder {
    type Output = crate::error::Result<crate::models::ProductCollection>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = crate::error::Result<crate::models::ProductCollection>>
                + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsUpdateBuilder {
    client: crate::Client,
    id: Option<String>,
    body: Option<crate::models::ProductCollectionsUpdateParams>,
}

impl ProductCollectionsUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::ProductCollectionsUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.update",
            param: "id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "product_collections.update",
        })?;
        let path = build_path("/product-collections/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsUpdateBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsListBuilder {
    client: crate::Client,
    query: Option<serde_json::Value>,
}

impl ProductCollectionsListBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::ProductCollectionListResponse>,
    > {
        let client = self.client;
        let query = self.query;
        let path = "/product-collections".to_string();
        let mut request = client.request(reqwest::Method::GET, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        let mut page =
            client
                .handle_response::<crate::models::DefaultPageNumberPagination<
                    crate::models::ProductCollectionListResponse,
                >>(request)
                .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            client.clone(),
            reqwest::Method::GET,
            path,
            query.unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl std::future::IntoFuture for ProductCollectionsListBuilder {
    type Output = crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::ProductCollectionListResponse>,
    >;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::DefaultPageNumberPagination<
                            crate::models::ProductCollectionListResponse,
                        >,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsDeleteBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl ProductCollectionsDeleteBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.delete",
            param: "id",
        })?;
        let path = build_path("/product-collections/{id}", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsDeleteBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsUnarchiveBuilder {
    client: crate::Client,
    id: Option<String>,
}

impl ProductCollectionsUnarchiveBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::ProductCollectionUnarchiveResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.unarchive",
            param: "id",
        })?;
        let path = build_path(
            "/product-collections/{id}/unarchive",
            &[("id", id.as_str())],
        );
        let request = client.request(reqwest::Method::POST, &path);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsUnarchiveBuilder {
    type Output = crate::error::Result<crate::models::ProductCollectionUnarchiveResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::ProductCollectionUnarchiveResponse,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsUpdateImagesBuilder {
    client: crate::Client,
    id: Option<String>,
    query: Option<serde_json::Value>,
}

impl ProductCollectionsUpdateImagesBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn query(mut self, query: serde_json::Value) -> Self {
        self.query = Some(query);
        self
    }

    pub async fn send(
        self,
    ) -> crate::error::Result<crate::models::ProductCollectionUpdateImagesResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.update_images",
            param: "id",
        })?;
        let query = self.query;
        let path = build_path("/product-collections/{id}/images", &[("id", id.as_str())]);
        let mut request = client.request(reqwest::Method::PUT, &path);
        if let Some(query) = &query {
            request = request.query(query);
        }
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsUpdateImagesBuilder {
    type Output = crate::error::Result<crate::models::ProductCollectionUpdateImagesResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<
                        crate::models::ProductCollectionUpdateImagesResponse,
                    >,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct ProductCollectionsGroupsResource {
    client: crate::Client,
}

impl ProductCollectionsGroupsResource {
    #[must_use = "resource accessors do nothing unless chained to a request"]
    pub fn items(&self) -> ProductCollectionsGroupsItemsResource {
        ProductCollectionsGroupsItemsResource {
            client: self.client.clone(),
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> ProductCollectionsGroupsCreateBuilder {
        ProductCollectionsGroupsCreateBuilder {
            client: self.client.clone(),
            id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> ProductCollectionsGroupsUpdateBuilder {
        ProductCollectionsGroupsUpdateBuilder {
            client: self.client.clone(),
            id: None,
            group_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn delete(&self) -> ProductCollectionsGroupsDeleteBuilder {
        ProductCollectionsGroupsDeleteBuilder {
            client: self.client.clone(),
            id: None,
            group_id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsGroupsCreateBuilder {
    client: crate::Client,
    id: Option<String>,
    body: Option<crate::models::ProductCollectionsGroupsCreateParams>,
}

impl ProductCollectionsGroupsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::ProductCollectionsGroupsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::ProductCollectionGroupResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.create",
            param: "id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "product_collections.groups.create",
        })?;
        let path = build_path("/product-collections/{id}/groups", &[("id", id.as_str())]);
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsGroupsCreateBuilder {
    type Output = crate::error::Result<crate::models::ProductCollectionGroupResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::ProductCollectionGroupResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsGroupsUpdateBuilder {
    client: crate::Client,
    id: Option<String>,
    group_id: Option<String>,
    body: Option<crate::models::ProductCollectionsGroupsUpdateParams>,
}

impl ProductCollectionsGroupsUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = Some(group_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::ProductCollectionsGroupsUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.update",
            param: "id",
        })?;
        let group_id = self.group_id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.update",
            param: "group_id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "product_collections.groups.update",
        })?;
        let path = build_path(
            "/product-collections/{id}/groups/{group_id}",
            &[("id", id.as_str()), ("group_id", group_id.as_str())],
        );
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsGroupsUpdateBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsGroupsDeleteBuilder {
    client: crate::Client,
    id: Option<String>,
    group_id: Option<String>,
}

impl ProductCollectionsGroupsDeleteBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = Some(group_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.delete",
            param: "id",
        })?;
        let group_id = self.group_id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.delete",
            param: "group_id",
        })?;
        let path = build_path(
            "/product-collections/{id}/groups/{group_id}",
            &[("id", id.as_str()), ("group_id", group_id.as_str())],
        );
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsGroupsDeleteBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[derive(Clone, Debug)]
pub struct ProductCollectionsGroupsItemsResource {
    client: crate::Client,
}

impl ProductCollectionsGroupsItemsResource {
    #[must_use = "request builders do nothing until you send or await them"]
    pub fn create(&self) -> ProductCollectionsGroupsItemsCreateBuilder {
        ProductCollectionsGroupsItemsCreateBuilder {
            client: self.client.clone(),
            id: None,
            group_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn update(&self) -> ProductCollectionsGroupsItemsUpdateBuilder {
        ProductCollectionsGroupsItemsUpdateBuilder {
            client: self.client.clone(),
            id: None,
            group_id: None,
            item_id: None,
            body: None,
        }
    }

    #[must_use = "request builders do nothing until you send or await them"]
    pub fn delete(&self) -> ProductCollectionsGroupsItemsDeleteBuilder {
        ProductCollectionsGroupsItemsDeleteBuilder {
            client: self.client.clone(),
            id: None,
            group_id: None,
            item_id: None,
        }
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsGroupsItemsCreateBuilder {
    client: crate::Client,
    id: Option<String>,
    group_id: Option<String>,
    body: Option<crate::models::ProductCollectionsGroupsItemsCreateParams>,
}

impl ProductCollectionsGroupsItemsCreateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = Some(group_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::ProductCollectionsGroupsItemsCreateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<crate::models::ItemCreateResponse> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.items.create",
            param: "id",
        })?;
        let group_id = self.group_id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.items.create",
            param: "group_id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "product_collections.groups.items.create",
        })?;
        let path = build_path(
            "/product-collections/{id}/groups/{group_id}/items",
            &[("id", id.as_str()), ("group_id", group_id.as_str())],
        );
        let request = client.request(reqwest::Method::POST, &path).json(&body);
        client.handle_response(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsGroupsItemsCreateBuilder {
    type Output = crate::error::Result<crate::models::ItemCreateResponse>;
    type IntoFuture = std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = crate::error::Result<crate::models::ItemCreateResponse>,
                > + Send
                + 'static,
        >,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsGroupsItemsUpdateBuilder {
    client: crate::Client,
    id: Option<String>,
    group_id: Option<String>,
    item_id: Option<String>,
    body: Option<crate::models::ProductCollectionsGroupsItemsUpdateParams>,
}

impl ProductCollectionsGroupsItemsUpdateBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = Some(group_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn item_id(mut self, item_id: impl Into<String>) -> Self {
        self.item_id = Some(item_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn body(mut self, body: crate::models::ProductCollectionsGroupsItemsUpdateParams) -> Self {
        self.body = Some(body);
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.items.update",
            param: "id",
        })?;
        let group_id = self.group_id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.items.update",
            param: "group_id",
        })?;
        let item_id = self.item_id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.items.update",
            param: "item_id",
        })?;
        let body = self.body.ok_or(crate::error::Error::MissingBody {
            operation: "product_collections.groups.items.update",
        })?;
        let path = build_path(
            "/product-collections/{id}/groups/{group_id}/items/{item_id}",
            &[
                ("id", id.as_str()),
                ("group_id", group_id.as_str()),
                ("item_id", item_id.as_str()),
            ],
        );
        let request = client.request(reqwest::Method::PATCH, &path).json(&body);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsGroupsItemsUpdateBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}

#[must_use = "request builders do nothing until you send or await them"]
#[derive(Clone, Debug)]
pub struct ProductCollectionsGroupsItemsDeleteBuilder {
    client: crate::Client,
    id: Option<String>,
    group_id: Option<String>,
    item_id: Option<String>,
}

impl ProductCollectionsGroupsItemsDeleteBuilder {
    #[must_use = "setters return an updated request builder"]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = Some(group_id.into());
        self
    }

    #[must_use = "setters return an updated request builder"]
    pub fn item_id(mut self, item_id: impl Into<String>) -> Self {
        self.item_id = Some(item_id.into());
        self
    }

    pub async fn send(self) -> crate::error::Result<()> {
        let client = self.client;
        let id = self.id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.items.delete",
            param: "id",
        })?;
        let group_id = self.group_id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.items.delete",
            param: "group_id",
        })?;
        let item_id = self.item_id.ok_or(crate::error::Error::MissingPathParam {
            operation: "product_collections.groups.items.delete",
            param: "item_id",
        })?;
        let path = build_path(
            "/product-collections/{id}/groups/{group_id}/items/{item_id}",
            &[
                ("id", id.as_str()),
                ("group_id", group_id.as_str()),
                ("item_id", item_id.as_str()),
            ],
        );
        let request = client.request(reqwest::Method::DELETE, &path);
        client.handle_empty(request).await
    }
}

impl std::future::IntoFuture for ProductCollectionsGroupsItemsDeleteBuilder {
    type Output = crate::error::Result<()>;
    type IntoFuture = std::pin::Pin<
        Box<dyn std::future::Future<Output = crate::error::Result<()>> + Send + 'static>,
    >;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
