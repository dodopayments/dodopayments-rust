impl crate::Client {
    pub async fn checkout_sessions_create(
        &self,
        body: &crate::models::CheckoutSessionsCreateParams,
    ) -> crate::error::Result<crate::models::CheckoutSessionResponse> {
        self.handle_response(self.request(reqwest::Method::POST, "/checkouts").json(body))
            .await
    }

    pub async fn checkout_sessions_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::CheckoutSessionStatus> {
        self.handle_response(self.request(reqwest::Method::GET, &format!("/checkouts/{}", id)))
            .await
    }

    pub async fn checkout_sessions_preview(
        &self,
        body: &crate::models::CheckoutSessionsCreateParams,
    ) -> crate::error::Result<crate::models::CheckoutSessionPreviewResponse> {
        self.handle_response(
            self.request(reqwest::Method::POST, "/checkouts/preview")
                .json(body),
        )
        .await
    }
}

impl crate::Client {
    pub async fn payments_create(
        &self,
        body: &crate::models::PaymentsCreateParams,
    ) -> crate::error::Result<crate::models::PaymentCreateResponse> {
        self.handle_response(self.request(reqwest::Method::POST, "/payments").json(body))
            .await
    }

    pub async fn payments_retrieve(
        &self,
        payment_id: &str,
    ) -> crate::error::Result<crate::models::Payment> {
        self.handle_response(
            self.request(reqwest::Method::GET, &format!("/payments/{}", payment_id)),
        )
        .await
    }

    pub async fn payments_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::PaymentListResponse>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/payments");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::PaymentListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/payments".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn payments_retrieve_line_items(
        &self,
        payment_id: &str,
    ) -> crate::error::Result<crate::models::PaymentRetrieveLineItemsResponse> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/payments/{}/line-items", payment_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn subscriptions_create(
        &self,
        body: &crate::models::SubscriptionsCreateParams,
    ) -> crate::error::Result<crate::models::SubscriptionCreateResponse> {
        self.handle_response(
            self.request(reqwest::Method::POST, "/subscriptions")
                .json(body),
        )
        .await
    }

    pub async fn subscriptions_retrieve(
        &self,
        subscription_id: &str,
    ) -> crate::error::Result<crate::models::Subscription> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/subscriptions/{}", subscription_id),
        ))
        .await
    }

    pub async fn subscriptions_update(
        &self,
        subscription_id: &str,
        body: &crate::models::SubscriptionsUpdateParams,
    ) -> crate::error::Result<crate::models::Subscription> {
        self.handle_response(
            self.request(
                reqwest::Method::PATCH,
                &format!("/subscriptions/{}", subscription_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn subscriptions_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::SubscriptionListResponse>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/subscriptions");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::SubscriptionListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/subscriptions".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn subscriptions_cancel_change_plan(
        &self,
        subscription_id: &str,
    ) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/subscriptions/{}/change-plan/scheduled", subscription_id),
        ))
        .await
    }

    pub async fn subscriptions_change_plan(
        &self,
        subscription_id: &str,
        body: &crate::models::SubscriptionsChangePlanParams,
    ) -> crate::error::Result<()> {
        self.handle_empty(
            self.request(
                reqwest::Method::POST,
                &format!("/subscriptions/{}/change-plan", subscription_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn subscriptions_charge(
        &self,
        subscription_id: &str,
        body: &crate::models::SubscriptionsChargeParams,
    ) -> crate::error::Result<crate::models::SubscriptionChargeResponse> {
        self.handle_response(
            self.request(
                reqwest::Method::POST,
                &format!("/subscriptions/{}/charge", subscription_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn subscriptions_preview_change_plan(
        &self,
        subscription_id: &str,
        body: &crate::models::SubscriptionsChangePlanParams,
    ) -> crate::error::Result<crate::models::SubscriptionPreviewChangePlanResponse> {
        self.handle_response(
            self.request(
                reqwest::Method::POST,
                &format!("/subscriptions/{}/change-plan/preview", subscription_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn subscriptions_retrieve_credit_usage(
        &self,
        subscription_id: &str,
    ) -> crate::error::Result<crate::models::SubscriptionRetrieveCreditUsageResponse> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/subscriptions/{}/credit-usage", subscription_id),
        ))
        .await
    }

    pub async fn subscriptions_retrieve_usage_history(
        &self,
        subscription_id: &str,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<
            crate::models::SubscriptionRetrieveUsageHistoryResponse,
        >,
    > {
        let mut request = self.request(
            reqwest::Method::GET,
            &format!("/subscriptions/{}/usage-history", subscription_id),
        );
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self
            .handle_response::<crate::models::DefaultPageNumberPagination<
                crate::models::SubscriptionRetrieveUsageHistoryResponse,
            >>(request)
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            format!("/subscriptions/{}/usage-history", subscription_id),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn subscriptions_update_payment_method(
        &self,
        subscription_id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::SubscriptionUpdatePaymentMethodResponse> {
        self.handle_response(
            self.request(
                reqwest::Method::POST,
                &format!("/subscriptions/{}/update-payment-method", subscription_id),
            )
            .json(body),
        )
        .await
    }
}

impl crate::Client {
    pub async fn invoices_payments_retrieve(
        &self,
        payment_id: &str,
    ) -> crate::error::Result<Vec<u8>> {
        self.handle_bytes(self.request(
            reqwest::Method::GET,
            &format!("/invoices/payments/{}", payment_id),
        ))
        .await
    }

    pub async fn invoices_payments_retrieve_payout(
        &self,
        payout_id: &str,
    ) -> crate::error::Result<Vec<u8>> {
        self.handle_bytes(self.request(
            reqwest::Method::GET,
            &format!("/invoices/payouts/{}", payout_id),
        ))
        .await
    }

    pub async fn invoices_payments_retrieve_refund(
        &self,
        refund_id: &str,
    ) -> crate::error::Result<Vec<u8>> {
        self.handle_bytes(self.request(
            reqwest::Method::GET,
            &format!("/invoices/refunds/{}", refund_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn licenses_activate(
        &self,
        body: &crate::models::LicensesActivateParams,
    ) -> crate::error::Result<crate::models::LicenseActivateResponse> {
        self.handle_response(
            self.request(reqwest::Method::POST, "/licenses/activate")
                .json(body),
        )
        .await
    }

    pub async fn licenses_deactivate(
        &self,
        body: &crate::models::LicensesDeactivateParams,
    ) -> crate::error::Result<()> {
        self.handle_empty(
            self.request(reqwest::Method::POST, "/licenses/deactivate")
                .json(body),
        )
        .await
    }

    pub async fn licenses_validate(
        &self,
        body: &crate::models::LicensesValidateParams,
    ) -> crate::error::Result<crate::models::LicenseValidateResponse> {
        self.handle_response(
            self.request(reqwest::Method::POST, "/licenses/validate")
                .json(body),
        )
        .await
    }
}

impl crate::Client {
    pub async fn license_keys_create(
        &self,
        body: &crate::models::LicenseKeysCreateParams,
    ) -> crate::error::Result<crate::models::LicenseKey> {
        self.handle_response(
            self.request(reqwest::Method::POST, "/license_keys")
                .json(body),
        )
        .await
    }

    pub async fn license_keys_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::LicenseKey> {
        self.handle_response(self.request(reqwest::Method::GET, &format!("/license_keys/{}", id)))
            .await
    }

    pub async fn license_keys_update(
        &self,
        id: &str,
        body: &crate::models::LicenseKeysUpdateParams,
    ) -> crate::error::Result<crate::models::LicenseKey> {
        self.handle_response(
            self.request(reqwest::Method::PATCH, &format!("/license_keys/{}", id))
                .json(body),
        )
        .await
    }

    pub async fn license_keys_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::LicenseKey>>
    {
        let mut request = self.request(reqwest::Method::GET, "/license_keys");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::LicenseKey>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/license_keys".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl crate::Client {
    pub async fn license_key_instances_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::LicenseKeyInstance> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/license_key_instances/{}", id),
        ))
        .await
    }

    pub async fn license_key_instances_update(
        &self,
        id: &str,
        body: &crate::models::LicenseKeyInstancesUpdateParams,
    ) -> crate::error::Result<crate::models::LicenseKeyInstance> {
        self.handle_response(
            self.request(
                reqwest::Method::PATCH,
                &format!("/license_key_instances/{}", id),
            )
            .json(body),
        )
        .await
    }

    pub async fn license_key_instances_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::LicenseKeyInstance>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/license_key_instances");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::LicenseKeyInstance>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/license_key_instances".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl crate::Client {
    pub async fn customers_create(
        &self,
        body: &crate::models::CustomersCreateParams,
    ) -> crate::error::Result<crate::models::Customer> {
        self.handle_response(self.request(reqwest::Method::POST, "/customers").json(body))
            .await
    }

    pub async fn customers_retrieve(
        &self,
        customer_id: &str,
    ) -> crate::error::Result<crate::models::Customer> {
        self.handle_response(
            self.request(reqwest::Method::GET, &format!("/customers/{}", customer_id)),
        )
        .await
    }

    pub async fn customers_update(
        &self,
        customer_id: &str,
        body: &crate::models::CustomersUpdateParams,
    ) -> crate::error::Result<crate::models::Customer> {
        self.handle_response(
            self.request(
                reqwest::Method::PATCH,
                &format!("/customers/{}", customer_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn customers_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Customer>>
    {
        let mut request = self.request(reqwest::Method::GET, "/customers");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self
            .handle_response::<crate::models::DefaultPageNumberPagination<crate::models::Customer>>(
                request,
            )
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/customers".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn customers_delete_payment_method(
        &self,
        customer_id: &str,
        payment_method_id: &str,
    ) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!(
                "/customers/{}/payment-methods/{}",
                customer_id, payment_method_id
            ),
        ))
        .await
    }

    pub async fn customers_list_credit_entitlements(
        &self,
        customer_id: &str,
    ) -> crate::error::Result<crate::models::CustomerListCreditEntitlementsResponse> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/customers/{}/credit-entitlements", customer_id),
        ))
        .await
    }

    pub async fn customers_list_entitlements(
        &self,
        customer_id: &str,
    ) -> crate::error::Result<crate::models::CustomerListEntitlementsResponse> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/customers/{}/entitlements", customer_id),
        ))
        .await
    }

    pub async fn customers_retrieve_payment_methods(
        &self,
        customer_id: &str,
    ) -> crate::error::Result<crate::models::CustomerRetrievePaymentMethodsResponse> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/customers/{}/payment-methods", customer_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn customers_customer_portal_create(
        &self,
        customer_id: &str,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<crate::models::CustomerPortalSession> {
        let mut request = self.request(
            reqwest::Method::POST,
            &format!("/customers/{}/customer-portal/session", customer_id),
        );
        if let Some(query) = query {
            request = request.query(query);
        }
        self.handle_response(request).await
    }
}

impl crate::Client {
    pub async fn customers_wallets_list(
        &self,
        customer_id: &str,
    ) -> crate::error::Result<crate::models::WalletListResponse> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/customers/{}/wallets", customer_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn customers_wallets_ledger_entries_create(
        &self,
        customer_id: &str,
        body: &crate::models::CustomersWalletsLedgerEntriesCreateParams,
    ) -> crate::error::Result<crate::models::CustomerWallet> {
        self.handle_response(
            self.request(
                reqwest::Method::POST,
                &format!("/customers/{}/wallets/ledger-entries", customer_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn customers_wallets_ledger_entries_list(
        &self,
        customer_id: &str,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CustomerWalletTransaction>,
    > {
        let mut request = self.request(
            reqwest::Method::GET,
            &format!("/customers/{}/wallets/ledger-entries", customer_id),
        );
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page =
            self.handle_response::<crate::models::DefaultPageNumberPagination<
                crate::models::CustomerWalletTransaction,
            >>(request)
                .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            format!("/customers/{}/wallets/ledger-entries", customer_id),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl crate::Client {
    pub async fn refunds_create(
        &self,
        body: &crate::models::RefundsCreateParams,
    ) -> crate::error::Result<crate::models::Refund> {
        self.handle_response(self.request(reqwest::Method::POST, "/refunds").json(body))
            .await
    }

    pub async fn refunds_retrieve(
        &self,
        refund_id: &str,
    ) -> crate::error::Result<crate::models::Refund> {
        self.handle_response(self.request(reqwest::Method::GET, &format!("/refunds/{}", refund_id)))
            .await
    }

    pub async fn refunds_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::RefundListItem>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/refunds");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::RefundListItem>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/refunds".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl crate::Client {
    pub async fn disputes_retrieve(
        &self,
        dispute_id: &str,
    ) -> crate::error::Result<crate::models::GetDispute> {
        self.handle_response(
            self.request(reqwest::Method::GET, &format!("/disputes/{}", dispute_id)),
        )
        .await
    }

    pub async fn disputes_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::DisputeListResponse>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/disputes");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::DisputeListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/disputes".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl crate::Client {
    pub async fn payouts_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::PayoutListResponse>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/payouts");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::PayoutListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/payouts".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl crate::Client {
    pub async fn payouts_breakup_retrieve(
        &self,
        payout_id: &str,
    ) -> crate::error::Result<crate::models::BreakupRetrieveResponse> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/payouts/{}/breakup", payout_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn payouts_breakup_details_list(
        &self,
        payout_id: &str,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::DetailListResponse>,
    > {
        let mut request = self.request(
            reqwest::Method::GET,
            &format!("/payouts/{}/breakup/details", payout_id),
        );
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::DetailListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            format!("/payouts/{}/breakup/details", payout_id),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn payouts_breakup_details_download_csv(
        &self,
        payout_id: &str,
    ) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::GET,
            &format!("/payouts/{}/breakup/details/csv", payout_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn products_create(
        &self,
        body: &crate::models::ProductsCreateParams,
    ) -> crate::error::Result<crate::models::Product> {
        self.handle_response(self.request(reqwest::Method::POST, "/products").json(body))
            .await
    }

    pub async fn products_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::Product> {
        self.handle_response(self.request(reqwest::Method::GET, &format!("/products/{}", id)))
            .await
    }

    pub async fn products_update(
        &self,
        id: &str,
        body: &crate::models::ProductsUpdateParams,
    ) -> crate::error::Result<()> {
        self.handle_empty(
            self.request(reqwest::Method::PATCH, &format!("/products/{}", id))
                .json(body),
        )
        .await
    }

    pub async fn products_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::ProductListResponse>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/products");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::ProductListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/products".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn products_archive(&self, id: &str) -> crate::error::Result<()> {
        self.handle_empty(self.request(reqwest::Method::DELETE, &format!("/products/{}", id)))
            .await
    }

    pub async fn products_unarchive(&self, id: &str) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::POST,
            &format!("/products/{}/unarchive", id),
        ))
        .await
    }

    pub async fn products_update_files(
        &self,
        id: &str,
        body: &crate::models::ProductsUpdateFilesParams,
    ) -> crate::error::Result<crate::models::ProductUpdateFilesResponse> {
        self.handle_response(
            self.request(reqwest::Method::PUT, &format!("/products/{}/files", id))
                .json(body),
        )
        .await
    }
}

impl crate::Client {
    pub async fn products_images_update(
        &self,
        id: &str,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<crate::models::ImageUpdateResponse> {
        let mut request = self.request(reqwest::Method::PUT, &format!("/products/{}/images", id));
        if let Some(query) = query {
            request = request.query(query);
        }
        self.handle_response(request).await
    }
}

impl crate::Client {
    pub async fn products_short_links_create(
        &self,
        id: &str,
        body: &crate::models::ProductsShortLinksCreateParams,
    ) -> crate::error::Result<crate::models::ShortLinkCreateResponse> {
        self.handle_response(
            self.request(
                reqwest::Method::POST,
                &format!("/products/{}/short_links", id),
            )
            .json(body),
        )
        .await
    }

    pub async fn products_short_links_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::ShortLinkListResponse>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/products/short_links");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::ShortLinkListResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/products/short_links".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl crate::Client {
    pub async fn misc_list_supported_countries(
        &self,
    ) -> crate::error::Result<crate::models::MiscListSupportedCountriesResponse> {
        self.handle_response(self.request(reqwest::Method::GET, "/checkout/supported_countries"))
            .await
    }
}

impl crate::Client {
    pub async fn discounts_create(
        &self,
        body: &crate::models::DiscountsCreateParams,
    ) -> crate::error::Result<crate::models::Discount> {
        self.handle_response(self.request(reqwest::Method::POST, "/discounts").json(body))
            .await
    }

    pub async fn discounts_retrieve(
        &self,
        discount_id: &str,
    ) -> crate::error::Result<crate::models::Discount> {
        self.handle_response(
            self.request(reqwest::Method::GET, &format!("/discounts/{}", discount_id)),
        )
        .await
    }

    pub async fn discounts_update(
        &self,
        discount_id: &str,
        body: &crate::models::DiscountsUpdateParams,
    ) -> crate::error::Result<crate::models::Discount> {
        self.handle_response(
            self.request(
                reqwest::Method::PATCH,
                &format!("/discounts/{}", discount_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn discounts_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Discount>>
    {
        let mut request = self.request(reqwest::Method::GET, "/discounts");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self
            .handle_response::<crate::models::DefaultPageNumberPagination<crate::models::Discount>>(
                request,
            )
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/discounts".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn discounts_delete(&self, discount_id: &str) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/discounts/{}", discount_id),
        ))
        .await
    }

    pub async fn discounts_retrieve_by_code(
        &self,
        code: &str,
    ) -> crate::error::Result<crate::models::Discount> {
        self.handle_response(
            self.request(reqwest::Method::GET, &format!("/discounts/code/{}", code)),
        )
        .await
    }
}

impl crate::Client {
    pub async fn addons_create(
        &self,
        body: &crate::models::AddonsCreateParams,
    ) -> crate::error::Result<crate::models::AddonResponse> {
        self.handle_response(self.request(reqwest::Method::POST, "/addons").json(body))
            .await
    }

    pub async fn addons_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::AddonResponse> {
        self.handle_response(self.request(reqwest::Method::GET, &format!("/addons/{}", id)))
            .await
    }

    pub async fn addons_update(
        &self,
        id: &str,
        body: &crate::models::AddonsUpdateParams,
    ) -> crate::error::Result<crate::models::AddonResponse> {
        self.handle_response(
            self.request(reqwest::Method::PATCH, &format!("/addons/{}", id))
                .json(body),
        )
        .await
    }

    pub async fn addons_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::AddonResponse>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/addons");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::AddonResponse>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/addons".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn addons_update_images(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::AddonUpdateImagesResponse> {
        self.handle_response(self.request(reqwest::Method::PUT, &format!("/addons/{}/images", id)))
            .await
    }
}

impl crate::Client {
    pub async fn brands_create(
        &self,
        body: &crate::models::BrandsCreateParams,
    ) -> crate::error::Result<crate::models::Brand> {
        self.handle_response(self.request(reqwest::Method::POST, "/brands").json(body))
            .await
    }

    pub async fn brands_retrieve(&self, id: &str) -> crate::error::Result<crate::models::Brand> {
        self.handle_response(self.request(reqwest::Method::GET, &format!("/brands/{}", id)))
            .await
    }

    pub async fn brands_update(
        &self,
        id: &str,
        body: &crate::models::BrandsUpdateParams,
    ) -> crate::error::Result<crate::models::Brand> {
        self.handle_response(
            self.request(reqwest::Method::PATCH, &format!("/brands/{}", id))
                .json(body),
        )
        .await
    }

    pub async fn brands_list(&self) -> crate::error::Result<crate::models::BrandListResponse> {
        self.handle_response(self.request(reqwest::Method::GET, "/brands"))
            .await
    }

    pub async fn brands_update_images(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::BrandUpdateImagesResponse> {
        self.handle_response(self.request(reqwest::Method::PUT, &format!("/brands/{}/images", id)))
            .await
    }
}

impl crate::Client {
    pub async fn webhooks_create(
        &self,
        body: &crate::models::WebhooksCreateParams,
    ) -> crate::error::Result<crate::models::WebhookDetails> {
        self.handle_response(self.request(reqwest::Method::POST, "/webhooks").json(body))
            .await
    }

    pub async fn webhooks_retrieve(
        &self,
        webhook_id: &str,
    ) -> crate::error::Result<crate::models::WebhookDetails> {
        self.handle_response(
            self.request(reqwest::Method::GET, &format!("/webhooks/{}", webhook_id)),
        )
        .await
    }

    pub async fn webhooks_update(
        &self,
        webhook_id: &str,
        body: &crate::models::WebhooksUpdateParams,
    ) -> crate::error::Result<crate::models::WebhookDetails> {
        self.handle_response(
            self.request(reqwest::Method::PATCH, &format!("/webhooks/{}", webhook_id))
                .json(body),
        )
        .await
    }

    pub async fn webhooks_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<crate::models::CursorPagePagination<crate::models::WebhookDetails>>
    {
        let mut request = self.request(reqwest::Method::GET, "/webhooks");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self
            .handle_response::<crate::models::CursorPagePagination<crate::models::WebhookDetails>>(
                request,
            )
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/webhooks".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn webhooks_delete(&self, webhook_id: &str) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/webhooks/{}", webhook_id),
        ))
        .await
    }

    pub async fn webhooks_retrieve_secret(
        &self,
        webhook_id: &str,
    ) -> crate::error::Result<crate::models::WebhookRetrieveSecretResponse> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/webhooks/{}/secret", webhook_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn webhooks_headers_retrieve(
        &self,
        webhook_id: &str,
    ) -> crate::error::Result<crate::models::HeaderRetrieveResponse> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/webhooks/{}/headers", webhook_id),
        ))
        .await
    }

    pub async fn webhooks_headers_update(
        &self,
        webhook_id: &str,
        body: &crate::models::WebhooksHeadersUpdateParams,
    ) -> crate::error::Result<()> {
        self.handle_empty(
            self.request(
                reqwest::Method::PATCH,
                &format!("/webhooks/{}/headers", webhook_id),
            )
            .json(body),
        )
        .await
    }
}

impl crate::Client {
    pub async fn usage_events_retrieve(
        &self,
        event_id: &str,
    ) -> crate::error::Result<crate::models::Event> {
        self.handle_response(self.request(reqwest::Method::GET, &format!("/events/{}", event_id)))
            .await
    }

    pub async fn usage_events_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Event>>
    {
        let mut request = self.request(reqwest::Method::GET, "/events");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self
            .handle_response::<crate::models::DefaultPageNumberPagination<crate::models::Event>>(
                request,
            )
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/events".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn usage_events_ingest(
        &self,
        body: &crate::models::UsageEventsIngestParams,
    ) -> crate::error::Result<crate::models::UsageEventIngestResponse> {
        self.handle_response(
            self.request(reqwest::Method::POST, "/events/ingest")
                .json(body),
        )
        .await
    }
}

impl crate::Client {
    pub async fn meters_create(
        &self,
        body: &crate::models::MetersCreateParams,
    ) -> crate::error::Result<crate::models::Meter> {
        self.handle_response(self.request(reqwest::Method::POST, "/meters").json(body))
            .await
    }

    pub async fn meters_retrieve(&self, id: &str) -> crate::error::Result<crate::models::Meter> {
        self.handle_response(self.request(reqwest::Method::GET, &format!("/meters/{}", id)))
            .await
    }

    pub async fn meters_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Meter>>
    {
        let mut request = self.request(reqwest::Method::GET, "/meters");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self
            .handle_response::<crate::models::DefaultPageNumberPagination<crate::models::Meter>>(
                request,
            )
            .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/meters".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn meters_archive(&self, id: &str) -> crate::error::Result<()> {
        self.handle_empty(self.request(reqwest::Method::DELETE, &format!("/meters/{}", id)))
            .await
    }

    pub async fn meters_unarchive(&self, id: &str) -> crate::error::Result<()> {
        self.handle_empty(self.request(reqwest::Method::POST, &format!("/meters/{}/unarchive", id)))
            .await
    }
}

impl crate::Client {
    pub async fn balances_retrieve_ledger(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::BalanceLedgerEntry>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/balances/ledger");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::BalanceLedgerEntry>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/balances/ledger".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl crate::Client {
    pub async fn credit_entitlements_create(
        &self,
        body: &crate::models::CreditEntitlementsCreateParams,
    ) -> crate::error::Result<crate::models::CreditEntitlement> {
        self.handle_response(
            self.request(reqwest::Method::POST, "/credit-entitlements")
                .json(body),
        )
        .await
    }

    pub async fn credit_entitlements_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::CreditEntitlement> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/credit-entitlements/{}", id),
        ))
        .await
    }

    pub async fn credit_entitlements_update(
        &self,
        id: &str,
        body: &crate::models::CreditEntitlementsUpdateParams,
    ) -> crate::error::Result<()> {
        self.handle_empty(
            self.request(
                reqwest::Method::PATCH,
                &format!("/credit-entitlements/{}", id),
            )
            .json(body),
        )
        .await
    }

    pub async fn credit_entitlements_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CreditEntitlement>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/credit-entitlements");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::CreditEntitlement>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/credit-entitlements".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn credit_entitlements_delete(&self, id: &str) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/credit-entitlements/{}", id),
        ))
        .await
    }

    pub async fn credit_entitlements_undelete(&self, id: &str) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::POST,
            &format!("/credit-entitlements/{}/undelete", id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn credit_entitlements_balances_retrieve(
        &self,
        credit_entitlement_id: &str,
        customer_id: &str,
    ) -> crate::error::Result<crate::models::CustomerCreditBalance> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!(
                "/credit-entitlements/{}/balances/{}",
                credit_entitlement_id, customer_id
            ),
        ))
        .await
    }

    pub async fn credit_entitlements_balances_list(
        &self,
        credit_entitlement_id: &str,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CustomerCreditBalance>,
    > {
        let mut request = self.request(
            reqwest::Method::GET,
            &format!("/credit-entitlements/{}/balances", credit_entitlement_id),
        );
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::CustomerCreditBalance>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            format!("/credit-entitlements/{}/balances", credit_entitlement_id),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn credit_entitlements_balances_create_ledger_entry(
        &self,
        credit_entitlement_id: &str,
        customer_id: &str,
        body: &crate::models::CreditEntitlementsBalancesCreateLedgerEntryParams,
    ) -> crate::error::Result<crate::models::BalanceCreateLedgerEntryResponse> {
        self.handle_response(
            self.request(
                reqwest::Method::POST,
                &format!(
                    "/credit-entitlements/{}/balances/{}/ledger-entries",
                    credit_entitlement_id, customer_id
                ),
            )
            .json(body),
        )
        .await
    }

    pub async fn credit_entitlements_balances_list_grants(
        &self,
        credit_entitlement_id: &str,
        customer_id: &str,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::BalanceListGrantsResponse>,
    > {
        let mut request = self.request(
            reqwest::Method::GET,
            &format!(
                "/credit-entitlements/{}/balances/{}/grants",
                credit_entitlement_id, customer_id
            ),
        );
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page =
            self.handle_response::<crate::models::DefaultPageNumberPagination<
                crate::models::BalanceListGrantsResponse,
            >>(request)
                .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            format!(
                "/credit-entitlements/{}/balances/{}/grants",
                credit_entitlement_id, customer_id
            ),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn credit_entitlements_balances_list_ledger(
        &self,
        credit_entitlement_id: &str,
        customer_id: &str,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::CreditLedgerEntry>,
    > {
        let mut request = self.request(
            reqwest::Method::GET,
            &format!(
                "/credit-entitlements/{}/balances/{}/ledger",
                credit_entitlement_id, customer_id
            ),
        );
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::CreditLedgerEntry>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            format!(
                "/credit-entitlements/{}/balances/{}/ledger",
                credit_entitlement_id, customer_id
            ),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }
}

impl crate::Client {
    pub async fn entitlements_create(
        &self,
        body: &crate::models::EntitlementsCreateParams,
    ) -> crate::error::Result<crate::models::Entitlement> {
        self.handle_response(
            self.request(reqwest::Method::POST, "/entitlements")
                .json(body),
        )
        .await
    }

    pub async fn entitlements_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::Entitlement> {
        self.handle_response(self.request(reqwest::Method::GET, &format!("/entitlements/{}", id)))
            .await
    }

    pub async fn entitlements_update(
        &self,
        id: &str,
        body: &crate::models::EntitlementsUpdateParams,
    ) -> crate::error::Result<crate::models::Entitlement> {
        self.handle_response(
            self.request(reqwest::Method::PATCH, &format!("/entitlements/{}", id))
                .json(body),
        )
        .await
    }

    pub async fn entitlements_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<crate::models::DefaultPageNumberPagination<crate::models::Entitlement>>
    {
        let mut request = self.request(reqwest::Method::GET, "/entitlements");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::Entitlement>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/entitlements".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn entitlements_delete(&self, id: &str) -> crate::error::Result<()> {
        self.handle_empty(self.request(reqwest::Method::DELETE, &format!("/entitlements/{}", id)))
            .await
    }
}

impl crate::Client {
    pub async fn entitlements_files_delete(
        &self,
        id: &str,
        file_id: &str,
    ) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/entitlements/{}/files/{}", id, file_id),
        ))
        .await
    }

    pub async fn entitlements_files_upload(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::FileUploadResponse> {
        self.handle_response(self.request(
            reqwest::Method::POST,
            &format!("/entitlements/{}/files", id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn entitlements_grants_list(
        &self,
        id: &str,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::EntitlementGrant>,
    > {
        let mut request = self.request(
            reqwest::Method::GET,
            &format!("/entitlements/{}/grants", id),
        );
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page = self.handle_response::<crate::models::DefaultPageNumberPagination<crate::models::EntitlementGrant>>(request).await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            format!("/entitlements/{}/grants", id),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn entitlements_grants_revoke(
        &self,
        id: &str,
        grant_id: &str,
    ) -> crate::error::Result<crate::models::EntitlementGrant> {
        self.handle_response(self.request(
            reqwest::Method::DELETE,
            &format!("/entitlements/{}/grants/{}", id, grant_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn product_collections_create(
        &self,
        body: &crate::models::ProductCollectionsCreateParams,
    ) -> crate::error::Result<crate::models::ProductCollection> {
        self.handle_response(
            self.request(reqwest::Method::POST, "/product-collections")
                .json(body),
        )
        .await
    }

    pub async fn product_collections_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::ProductCollection> {
        self.handle_response(self.request(
            reqwest::Method::GET,
            &format!("/product-collections/{}", id),
        ))
        .await
    }

    pub async fn product_collections_update(
        &self,
        id: &str,
        body: &crate::models::ProductCollectionsUpdateParams,
    ) -> crate::error::Result<()> {
        self.handle_empty(
            self.request(
                reqwest::Method::PATCH,
                &format!("/product-collections/{}", id),
            )
            .json(body),
        )
        .await
    }

    pub async fn product_collections_list(
        &self,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<
        crate::models::DefaultPageNumberPagination<crate::models::ProductCollectionListResponse>,
    > {
        let mut request = self.request(reqwest::Method::GET, "/product-collections");
        if let Some(query) = query {
            request = request.query(query);
        }
        let mut page =
            self.handle_response::<crate::models::DefaultPageNumberPagination<
                crate::models::ProductCollectionListResponse,
            >>(request)
                .await?;
        page.set_pagination_context(crate::client::PaginationContext::new(
            self.clone(),
            reqwest::Method::GET,
            "/product-collections".to_string(),
            query.cloned().unwrap_or_else(|| serde_json::json!({})),
        ));
        Ok(page)
    }

    pub async fn product_collections_delete(&self, id: &str) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/product-collections/{}", id),
        ))
        .await
    }

    pub async fn product_collections_unarchive(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::ProductCollectionUnarchiveResponse> {
        self.handle_response(self.request(
            reqwest::Method::POST,
            &format!("/product-collections/{}/unarchive", id),
        ))
        .await
    }

    pub async fn product_collections_update_images(
        &self,
        id: &str,
        query: Option<&serde_json::Value>,
    ) -> crate::error::Result<crate::models::ProductCollectionUpdateImagesResponse> {
        let mut request = self.request(
            reqwest::Method::PUT,
            &format!("/product-collections/{}/images", id),
        );
        if let Some(query) = query {
            request = request.query(query);
        }
        self.handle_response(request).await
    }
}

impl crate::Client {
    pub async fn product_collections_groups_create(
        &self,
        id: &str,
        body: &crate::models::ProductCollectionsGroupsCreateParams,
    ) -> crate::error::Result<crate::models::ProductCollectionGroupResponse> {
        self.handle_response(
            self.request(
                reqwest::Method::POST,
                &format!("/product-collections/{}/groups", id),
            )
            .json(body),
        )
        .await
    }

    pub async fn product_collections_groups_update(
        &self,
        id: &str,
        group_id: &str,
        body: &crate::models::ProductCollectionsGroupsUpdateParams,
    ) -> crate::error::Result<()> {
        self.handle_empty(
            self.request(
                reqwest::Method::PATCH,
                &format!("/product-collections/{}/groups/{}", id, group_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn product_collections_groups_delete(
        &self,
        id: &str,
        group_id: &str,
    ) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/product-collections/{}/groups/{}", id, group_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn product_collections_groups_items_create(
        &self,
        id: &str,
        group_id: &str,
        body: &crate::models::ProductCollectionsGroupsItemsCreateParams,
    ) -> crate::error::Result<crate::models::ItemCreateResponse> {
        self.handle_response(
            self.request(
                reqwest::Method::POST,
                &format!("/product-collections/{}/groups/{}/items", id, group_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn product_collections_groups_items_update(
        &self,
        id: &str,
        group_id: &str,
        item_id: &str,
        body: &crate::models::ProductCollectionsGroupsItemsUpdateParams,
    ) -> crate::error::Result<()> {
        self.handle_empty(
            self.request(
                reqwest::Method::PATCH,
                &format!(
                    "/product-collections/{}/groups/{}/items/{}",
                    id, group_id, item_id
                ),
            )
            .json(body),
        )
        .await
    }

    pub async fn product_collections_groups_items_delete(
        &self,
        id: &str,
        group_id: &str,
        item_id: &str,
    ) -> crate::error::Result<()> {
        self.handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!(
                "/product-collections/{}/groups/{}/items/{}",
                id, group_id, item_id
            ),
        ))
        .await
    }
}
