impl crate::Client {
    pub async fn checkout_sessions_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::CheckoutSessionResponse> {
        crate::client::handle_response(self.request(reqwest::Method::POST, "/checkouts").json(body))
            .await
    }

    pub async fn checkout_sessions_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::CheckoutSessionStatus> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/checkouts/{}", id)),
        )
        .await
    }

    pub async fn checkout_sessions_preview(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::CheckoutSessionPreviewResponse> {
        crate::client::handle_response(
            self.request(reqwest::Method::POST, "/checkouts/preview")
                .json(body),
        )
        .await
    }
}

impl crate::Client {
    pub async fn payments_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::PaymentCreateResponse> {
        crate::client::handle_response(self.request(reqwest::Method::POST, "/payments").json(body))
            .await
    }

    pub async fn payments_retrieve(
        &self,
        payment_id: &str,
    ) -> crate::error::Result<crate::models::Payment> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/payments/{}", payment_id)),
        )
        .await
    }

    pub async fn payments_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/payments")).await
    }

    pub async fn payments_retrieve_line_items(
        &self,
        payment_id: &str,
    ) -> crate::error::Result<crate::models::PaymentRetrieveLineItemsResponse> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/payments/{}/line-items", payment_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn subscriptions_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::SubscriptionCreateResponse> {
        crate::client::handle_response(
            self.request(reqwest::Method::POST, "/subscriptions")
                .json(body),
        )
        .await
    }

    pub async fn subscriptions_retrieve(
        &self,
        subscription_id: &str,
    ) -> crate::error::Result<crate::models::Subscription> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/subscriptions/{}", subscription_id),
        ))
        .await
    }

    pub async fn subscriptions_update(
        &self,
        subscription_id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Subscription> {
        crate::client::handle_response(
            self.request(
                reqwest::Method::PATCH,
                &format!("/subscriptions/{}", subscription_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn subscriptions_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/subscriptions")).await
    }

    pub async fn subscriptions_cancel_change_plan(
        &self,
        subscription_id: &str,
    ) -> crate::error::Result<()> {
        crate::client::handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/subscriptions/{}/change-plan/scheduled", subscription_id),
        ))
        .await
    }

    pub async fn subscriptions_change_plan(
        &self,
        subscription_id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<()> {
        crate::client::handle_empty(
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
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::SubscriptionChargeResponse> {
        crate::client::handle_response(
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
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::SubscriptionPreviewChangePlanResponse> {
        crate::client::handle_response(
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
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/subscriptions/{}/credit-usage", subscription_id),
        ))
        .await
    }

    pub async fn subscriptions_retrieve_usage_history(
        &self,
        subscription_id: &str,
    ) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/subscriptions/{}/usage-history", subscription_id),
        ))
        .await
    }

    pub async fn subscriptions_update_payment_method(
        &self,
        subscription_id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::SubscriptionUpdatePaymentMethodResponse> {
        crate::client::handle_response(
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
    ) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/invoices/payments/{}", payment_id),
        ))
        .await
    }

    pub async fn invoices_payments_retrieve_payout(
        &self,
        payout_id: &str,
    ) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/invoices/payouts/{}", payout_id),
        ))
        .await
    }

    pub async fn invoices_payments_retrieve_refund(
        &self,
        refund_id: &str,
    ) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/invoices/refunds/{}", refund_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn licenses_activate(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::LicenseActivateResponse> {
        crate::client::handle_response(
            self.request(reqwest::Method::POST, "/licenses/activate")
                .json(body),
        )
        .await
    }

    pub async fn licenses_deactivate(&self, body: &serde_json::Value) -> crate::error::Result<()> {
        crate::client::handle_empty(
            self.request(reqwest::Method::POST, "/licenses/deactivate")
                .json(body),
        )
        .await
    }

    pub async fn licenses_validate(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::LicenseValidateResponse> {
        crate::client::handle_response(
            self.request(reqwest::Method::POST, "/licenses/validate")
                .json(body),
        )
        .await
    }
}

impl crate::Client {
    pub async fn license_keys_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::LicenseKey> {
        crate::client::handle_response(
            self.request(reqwest::Method::POST, "/license_keys")
                .json(body),
        )
        .await
    }

    pub async fn license_keys_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::LicenseKey> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/license_keys/{}", id)),
        )
        .await
    }

    pub async fn license_keys_update(
        &self,
        id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::LicenseKey> {
        crate::client::handle_response(
            self.request(reqwest::Method::PATCH, &format!("/license_keys/{}", id))
                .json(body),
        )
        .await
    }

    pub async fn license_keys_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/license_keys")).await
    }
}

impl crate::Client {
    pub async fn license_key_instances_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::LicenseKeyInstance> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/license_key_instances/{}", id),
        ))
        .await
    }

    pub async fn license_key_instances_update(
        &self,
        id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::LicenseKeyInstance> {
        crate::client::handle_response(
            self.request(
                reqwest::Method::PATCH,
                &format!("/license_key_instances/{}", id),
            )
            .json(body),
        )
        .await
    }

    pub async fn license_key_instances_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/license_key_instances"))
            .await
    }
}

impl crate::Client {
    pub async fn customers_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Customer> {
        crate::client::handle_response(self.request(reqwest::Method::POST, "/customers").json(body))
            .await
    }

    pub async fn customers_retrieve(
        &self,
        customer_id: &str,
    ) -> crate::error::Result<crate::models::Customer> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/customers/{}", customer_id)),
        )
        .await
    }

    pub async fn customers_update(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Customer> {
        crate::client::handle_response(
            self.request(
                reqwest::Method::PATCH,
                &format!("/customers/{}", customer_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn customers_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/customers")).await
    }

    pub async fn customers_delete_payment_method(
        &self,
        customer_id: &str,
        payment_method_id: &str,
    ) -> crate::error::Result<()> {
        crate::client::handle_empty(self.request(
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
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/customers/{}/credit-entitlements", customer_id),
        ))
        .await
    }

    pub async fn customers_list_entitlements(
        &self,
        customer_id: &str,
    ) -> crate::error::Result<crate::models::CustomerListEntitlementsResponse> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/customers/{}/entitlements", customer_id),
        ))
        .await
    }

    pub async fn customers_retrieve_payment_methods(
        &self,
        customer_id: &str,
    ) -> crate::error::Result<crate::models::CustomerRetrievePaymentMethodsResponse> {
        crate::client::handle_response(self.request(
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
    ) -> crate::error::Result<crate::models::CustomerPortalSession> {
        crate::client::handle_response(self.request(
            reqwest::Method::POST,
            &format!("/customers/{}/customer-portal/session", customer_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn customers_wallets_list(
        &self,
        customer_id: &str,
    ) -> crate::error::Result<crate::models::WalletListResponse> {
        crate::client::handle_response(self.request(
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
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::CustomerWallet> {
        crate::client::handle_response(
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
    ) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/customers/{}/wallets/ledger-entries", customer_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn refunds_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Refund> {
        crate::client::handle_response(self.request(reqwest::Method::POST, "/refunds").json(body))
            .await
    }

    pub async fn refunds_retrieve(
        &self,
        refund_id: &str,
    ) -> crate::error::Result<crate::models::Refund> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/refunds/{}", refund_id)),
        )
        .await
    }

    pub async fn refunds_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/refunds")).await
    }
}

impl crate::Client {
    pub async fn disputes_retrieve(
        &self,
        dispute_id: &str,
    ) -> crate::error::Result<crate::models::GetDispute> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/disputes/{}", dispute_id)),
        )
        .await
    }

    pub async fn disputes_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/disputes")).await
    }
}

impl crate::Client {
    pub async fn payouts_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/payouts")).await
    }
}

impl crate::Client {
    pub async fn payouts_breakup_retrieve(
        &self,
        payout_id: &str,
    ) -> crate::error::Result<crate::models::BreakupRetrieveResponse> {
        crate::client::handle_response(self.request(
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
    ) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/payouts/{}/breakup/details", payout_id),
        ))
        .await
    }

    pub async fn payouts_breakup_details_download_csv(
        &self,
        payout_id: &str,
    ) -> crate::error::Result<()> {
        crate::client::handle_empty(self.request(
            reqwest::Method::GET,
            &format!("/payouts/{}/breakup/details/csv", payout_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn products_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Product> {
        crate::client::handle_response(self.request(reqwest::Method::POST, "/products").json(body))
            .await
    }

    pub async fn products_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::Product> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/products/{}", id)),
        )
        .await
    }

    pub async fn products_update(
        &self,
        id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<()> {
        crate::client::handle_empty(
            self.request(reqwest::Method::PATCH, &format!("/products/{}", id))
                .json(body),
        )
        .await
    }

    pub async fn products_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/products")).await
    }

    pub async fn products_archive(&self, id: &str) -> crate::error::Result<()> {
        crate::client::handle_empty(
            self.request(reqwest::Method::DELETE, &format!("/products/{}", id)),
        )
        .await
    }

    pub async fn products_unarchive(&self, id: &str) -> crate::error::Result<()> {
        crate::client::handle_empty(self.request(
            reqwest::Method::POST,
            &format!("/products/{}/unarchive", id),
        ))
        .await
    }

    pub async fn products_update_files(
        &self,
        id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::ProductUpdateFilesResponse> {
        crate::client::handle_response(
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
    ) -> crate::error::Result<crate::models::ImageUpdateResponse> {
        crate::client::handle_response(
            self.request(reqwest::Method::PUT, &format!("/products/{}/images", id)),
        )
        .await
    }
}

impl crate::Client {
    pub async fn products_short_links_create(
        &self,
        id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::ShortLinkCreateResponse> {
        crate::client::handle_response(
            self.request(
                reqwest::Method::POST,
                &format!("/products/{}/short_links", id),
            )
            .json(body),
        )
        .await
    }

    pub async fn products_short_links_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/products/short_links"))
            .await
    }
}

impl crate::Client {
    pub async fn misc_list_supported_countries(
        &self,
    ) -> crate::error::Result<crate::models::MiscListSupportedCountriesResponse> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, "/checkout/supported_countries"),
        )
        .await
    }
}

impl crate::Client {
    pub async fn discounts_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Discount> {
        crate::client::handle_response(self.request(reqwest::Method::POST, "/discounts").json(body))
            .await
    }

    pub async fn discounts_retrieve(
        &self,
        discount_id: &str,
    ) -> crate::error::Result<crate::models::Discount> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/discounts/{}", discount_id)),
        )
        .await
    }

    pub async fn discounts_update(
        &self,
        discount_id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Discount> {
        crate::client::handle_response(
            self.request(
                reqwest::Method::PATCH,
                &format!("/discounts/{}", discount_id),
            )
            .json(body),
        )
        .await
    }

    pub async fn discounts_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/discounts")).await
    }

    pub async fn discounts_delete(&self, discount_id: &str) -> crate::error::Result<()> {
        crate::client::handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/discounts/{}", discount_id),
        ))
        .await
    }

    pub async fn discounts_retrieve_by_code(
        &self,
        code: &str,
    ) -> crate::error::Result<crate::models::Discount> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/discounts/code/{}", code)),
        )
        .await
    }
}

impl crate::Client {
    pub async fn addons_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::AddonResponse> {
        crate::client::handle_response(self.request(reqwest::Method::POST, "/addons").json(body))
            .await
    }

    pub async fn addons_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::AddonResponse> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/addons/{}", id)),
        )
        .await
    }

    pub async fn addons_update(
        &self,
        id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::AddonResponse> {
        crate::client::handle_response(
            self.request(reqwest::Method::PATCH, &format!("/addons/{}", id))
                .json(body),
        )
        .await
    }

    pub async fn addons_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/addons")).await
    }

    pub async fn addons_update_images(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::AddonUpdateImagesResponse> {
        crate::client::handle_response(
            self.request(reqwest::Method::PUT, &format!("/addons/{}/images", id)),
        )
        .await
    }
}

impl crate::Client {
    pub async fn brands_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Brand> {
        crate::client::handle_response(self.request(reqwest::Method::POST, "/brands").json(body))
            .await
    }

    pub async fn brands_retrieve(&self, id: &str) -> crate::error::Result<crate::models::Brand> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/brands/{}", id)),
        )
        .await
    }

    pub async fn brands_update(
        &self,
        id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Brand> {
        crate::client::handle_response(
            self.request(reqwest::Method::PATCH, &format!("/brands/{}", id))
                .json(body),
        )
        .await
    }

    pub async fn brands_list(&self) -> crate::error::Result<crate::models::BrandListResponse> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/brands")).await
    }

    pub async fn brands_update_images(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::BrandUpdateImagesResponse> {
        crate::client::handle_response(
            self.request(reqwest::Method::PUT, &format!("/brands/{}/images", id)),
        )
        .await
    }
}

impl crate::Client {
    pub async fn webhooks_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::WebhookDetails> {
        crate::client::handle_response(self.request(reqwest::Method::POST, "/webhooks").json(body))
            .await
    }

    pub async fn webhooks_retrieve(
        &self,
        webhook_id: &str,
    ) -> crate::error::Result<crate::models::WebhookDetails> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/webhooks/{}", webhook_id)),
        )
        .await
    }

    pub async fn webhooks_update(
        &self,
        webhook_id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::WebhookDetails> {
        crate::client::handle_response(
            self.request(reqwest::Method::PATCH, &format!("/webhooks/{}", webhook_id))
                .json(body),
        )
        .await
    }

    pub async fn webhooks_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/webhooks")).await
    }

    pub async fn webhooks_delete(&self, webhook_id: &str) -> crate::error::Result<()> {
        crate::client::handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/webhooks/{}", webhook_id),
        ))
        .await
    }

    pub async fn webhooks_retrieve_secret(
        &self,
        webhook_id: &str,
    ) -> crate::error::Result<crate::models::WebhookRetrieveSecretResponse> {
        crate::client::handle_response(self.request(
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
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/webhooks/{}/headers", webhook_id),
        ))
        .await
    }

    pub async fn webhooks_headers_update(
        &self,
        webhook_id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<()> {
        crate::client::handle_empty(
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
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/events/{}", event_id)),
        )
        .await
    }

    pub async fn usage_events_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/events")).await
    }

    pub async fn usage_events_ingest(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::UsageEventIngestResponse> {
        crate::client::handle_response(
            self.request(reqwest::Method::POST, "/events/ingest")
                .json(body),
        )
        .await
    }
}

impl crate::Client {
    pub async fn meters_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Meter> {
        crate::client::handle_response(self.request(reqwest::Method::POST, "/meters").json(body))
            .await
    }

    pub async fn meters_retrieve(&self, id: &str) -> crate::error::Result<crate::models::Meter> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/meters/{}", id)),
        )
        .await
    }

    pub async fn meters_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/meters")).await
    }

    pub async fn meters_archive(&self, id: &str) -> crate::error::Result<()> {
        crate::client::handle_empty(
            self.request(reqwest::Method::DELETE, &format!("/meters/{}", id)),
        )
        .await
    }

    pub async fn meters_unarchive(&self, id: &str) -> crate::error::Result<()> {
        crate::client::handle_empty(
            self.request(reqwest::Method::POST, &format!("/meters/{}/unarchive", id)),
        )
        .await
    }
}

impl crate::Client {
    pub async fn balances_retrieve_ledger(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/balances/ledger")).await
    }
}

impl crate::Client {
    pub async fn credit_entitlements_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::CreditEntitlement> {
        crate::client::handle_response(
            self.request(reqwest::Method::POST, "/credit-entitlements")
                .json(body),
        )
        .await
    }

    pub async fn credit_entitlements_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::CreditEntitlement> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/credit-entitlements/{}", id),
        ))
        .await
    }

    pub async fn credit_entitlements_update(
        &self,
        id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<()> {
        crate::client::handle_empty(
            self.request(
                reqwest::Method::PATCH,
                &format!("/credit-entitlements/{}", id),
            )
            .json(body),
        )
        .await
    }

    pub async fn credit_entitlements_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/credit-entitlements"))
            .await
    }

    pub async fn credit_entitlements_delete(&self, id: &str) -> crate::error::Result<()> {
        crate::client::handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/credit-entitlements/{}", id),
        ))
        .await
    }

    pub async fn credit_entitlements_undelete(&self, id: &str) -> crate::error::Result<()> {
        crate::client::handle_empty(self.request(
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
        crate::client::handle_response(self.request(
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
    ) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/credit-entitlements/{}/balances", credit_entitlement_id),
        ))
        .await
    }

    pub async fn credit_entitlements_balances_create_ledger_entry(
        &self,
        credit_entitlement_id: &str,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::BalanceCreateLedgerEntryResponse> {
        crate::client::handle_response(
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
    ) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!(
                "/credit-entitlements/{}/balances/{}/grants",
                credit_entitlement_id, customer_id
            ),
        ))
        .await
    }

    pub async fn credit_entitlements_balances_list_ledger(
        &self,
        credit_entitlement_id: &str,
        customer_id: &str,
    ) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!(
                "/credit-entitlements/{}/balances/{}/ledger",
                credit_entitlement_id, customer_id
            ),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn entitlements_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Entitlement> {
        crate::client::handle_response(
            self.request(reqwest::Method::POST, "/entitlements")
                .json(body),
        )
        .await
    }

    pub async fn entitlements_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::Entitlement> {
        crate::client::handle_response(
            self.request(reqwest::Method::GET, &format!("/entitlements/{}", id)),
        )
        .await
    }

    pub async fn entitlements_update(
        &self,
        id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::Entitlement> {
        crate::client::handle_response(
            self.request(reqwest::Method::PATCH, &format!("/entitlements/{}", id))
                .json(body),
        )
        .await
    }

    pub async fn entitlements_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/entitlements")).await
    }

    pub async fn entitlements_delete(&self, id: &str) -> crate::error::Result<()> {
        crate::client::handle_empty(
            self.request(reqwest::Method::DELETE, &format!("/entitlements/{}", id)),
        )
        .await
    }
}

impl crate::Client {
    pub async fn entitlements_files_delete(
        &self,
        id: &str,
        file_id: &str,
    ) -> crate::error::Result<()> {
        crate::client::handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/entitlements/{}/files/{}", id, file_id),
        ))
        .await
    }

    pub async fn entitlements_files_upload(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::FileUploadResponse> {
        crate::client::handle_response(self.request(
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
    ) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/entitlements/{}/grants", id),
        ))
        .await
    }

    pub async fn entitlements_grants_revoke(
        &self,
        id: &str,
        grant_id: &str,
    ) -> crate::error::Result<crate::models::EntitlementGrant> {
        crate::client::handle_response(self.request(
            reqwest::Method::DELETE,
            &format!("/entitlements/{}/grants/{}", id, grant_id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn product_collections_create(
        &self,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::ProductCollection> {
        crate::client::handle_response(
            self.request(reqwest::Method::POST, "/product-collections")
                .json(body),
        )
        .await
    }

    pub async fn product_collections_retrieve(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::ProductCollection> {
        crate::client::handle_response(self.request(
            reqwest::Method::GET,
            &format!("/product-collections/{}", id),
        ))
        .await
    }

    pub async fn product_collections_update(
        &self,
        id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<()> {
        crate::client::handle_empty(
            self.request(
                reqwest::Method::PATCH,
                &format!("/product-collections/{}", id),
            )
            .json(body),
        )
        .await
    }

    pub async fn product_collections_list(&self) -> crate::error::Result<serde_json::Value> {
        crate::client::handle_response(self.request(reqwest::Method::GET, "/product-collections"))
            .await
    }

    pub async fn product_collections_delete(&self, id: &str) -> crate::error::Result<()> {
        crate::client::handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!("/product-collections/{}", id),
        ))
        .await
    }

    pub async fn product_collections_unarchive(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::ProductCollectionUnarchiveResponse> {
        crate::client::handle_response(self.request(
            reqwest::Method::POST,
            &format!("/product-collections/{}/unarchive", id),
        ))
        .await
    }

    pub async fn product_collections_update_images(
        &self,
        id: &str,
    ) -> crate::error::Result<crate::models::ProductCollectionUpdateImagesResponse> {
        crate::client::handle_response(self.request(
            reqwest::Method::PUT,
            &format!("/product-collections/{}/images", id),
        ))
        .await
    }
}

impl crate::Client {
    pub async fn product_collections_groups_create(
        &self,
        id: &str,
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::ProductCollectionGroupResponse> {
        crate::client::handle_response(
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
        body: &serde_json::Value,
    ) -> crate::error::Result<()> {
        crate::client::handle_empty(
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
        crate::client::handle_empty(self.request(
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
        body: &serde_json::Value,
    ) -> crate::error::Result<crate::models::ItemCreateResponse> {
        crate::client::handle_response(
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
        body: &serde_json::Value,
    ) -> crate::error::Result<()> {
        crate::client::handle_empty(
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
        crate::client::handle_empty(self.request(
            reqwest::Method::DELETE,
            &format!(
                "/product-collections/{}/groups/{}/items/{}",
                id, group_id, item_id
            ),
        ))
        .await
    }
}
